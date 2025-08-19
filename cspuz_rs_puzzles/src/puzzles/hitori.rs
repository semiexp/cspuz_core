use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, MultiDigit,
};
use cspuz_rs::solver::Solver;

pub fn solve_hitori(clues: &[Vec<i32>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    graph::active_vertices_connected_2d(&mut solver, !is_black);
    solver.add_expr(!is_black.conv2d_and((1, 2)));
    solver.add_expr(!is_black.conv2d_and((2, 1)));

    for y in 0..h {
        for x0 in 0..w {
            for x1 in 0..x0 {
                if clues[y][x0] == clues[y][x1] && clues[y][x0] > 0 && clues[y][x1] > 0 {
                    solver.add_expr(is_black.at((y, x0)) | is_black.at((y, x1)));
                }
            }
        }
    }

    for x in 0..w {
        for y0 in 0..h {
            for y1 in 0..y0 {
                if clues[y0][x] == clues[y1][x] && clues[y0][x] > 0 && clues[y1][x] > 0 {
                    solver.add_expr(is_black.at((y0, x)) | is_black.at((y1, x)));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = Vec<Vec<i32>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Dict::new(0, ".")),
        Box::new(MultiDigit::new(36, 1)),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "hitori", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["hitori"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![1, 1, 1, 0, 4, 5],
            vec![0, 2, 0, 0, 4, 5],
            vec![3, 0, 3, 0, 1, 0],
            vec![0, 2, 0, 0, 0, 0],
            vec![3, 0, 3, 0, 1, 0],
        ]
    }

    #[test]
    fn test_hitori_problem() {
        let problem = problem_for_tests();
        let ans = solve_hitori(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [1, 0, 1, 0, 0, 1],
            [0, 0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0, 0],
            [1, 0, 0, 0, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_akari_rgb_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?hitori/6/5/111.45.2..453.3.1..2....3.3.1.";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
