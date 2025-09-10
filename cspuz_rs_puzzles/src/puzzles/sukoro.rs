use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_sukoro(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((h, w), 0, 4);
    solver.add_answer_key_int(num);

    let is_num = &solver.bool_var_2d((h, w));

    solver.add_expr(num.ge(1).iff(is_num));

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                if c == -1 {
                    solver.add_expr(num.at((y, x)).ne(0));
                } else {
                    solver.add_expr(num.at((y, x)).eq(c));
                }
            }

            if x < w - 1 {
                solver.add_expr(
                    (is_num.at((y, x)) & is_num.at((y, x + 1)))
                        .imp(num.at((y, x)).ne(num.at((y, x + 1)))),
                );
            }

            if y < h - 1 {
                solver.add_expr(
                    (is_num.at((y, x)) & is_num.at((y + 1, x)))
                        .imp(num.at((y, x)).ne(num.at((y + 1, x)))),
                );
            }

            solver.add_expr(
                is_num.at((y, x)).imp(
                    is_num
                        .four_neighbors((y, x))
                        .count_true()
                        .eq(num.at((y, x))),
                ),
            );
        }
    }

    graph::active_vertices_connected_2d(&mut solver, is_num);

    solver.irrefutable_facts().map(|f| f.get(num))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Spaces::new(None, 'a')),
        Box::new(Dict::new(Some(-1), ".")),
        Box::new(Dict::new(Some(1), "1")),
        Box::new(Dict::new(Some(2), "2")),
        Box::new(Dict::new(Some(3), "3")),
        Box::new(Dict::new(Some(4), "4")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "sukoro", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["sukoro"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let clues = vec![
            vec![None, Some(3), None, None],
            vec![None, None, Some(4), None],
            vec![None, None, None, None],
            vec![None, Some(2), None, None],
        ];

        clues
    }

    #[test]
    fn test_sukoro_problem() {
        let clues = problem_for_tests();
        let ans = solve_sukoro(&clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [1, 3, 2, 0],
            [0, 2, 4, 1],
            [0, 0, 2, 0],
            [1, 2, 3, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_sukoro_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?sukoro/4/4/a3d4f2b";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
