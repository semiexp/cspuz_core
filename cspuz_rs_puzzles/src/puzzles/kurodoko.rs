use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_kurodoko(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    solver.add_expr(!(is_black.conv2d_and((1, 2))));
    solver.add_expr(!(is_black.conv2d_and((2, 1))));
    graph::active_vertices_connected_2d(&mut solver, !is_black);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                if n < 0 {
                    continue;
                }
                solver.add_expr(
                    ((!is_black.slice_fixed_y((y, ..x)).reverse()).consecutive_prefix_true()
                        + (!is_black.slice_fixed_y((y, x + 1..))).consecutive_prefix_true()
                        + (!is_black.slice_fixed_x((..y, x)).reverse()).consecutive_prefix_true()
                        + (!is_black.slice_fixed_x((y + 1.., x))).consecutive_prefix_true())
                    .eq(n - 1),
                );
            }
        }
    }
    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "kurodoko", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["kurodoko"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        vec![
            vec![None, None, Some(2), None, None, None],
            vec![None, Some(2), None, None, Some(3), None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, Some(5)],
            vec![None, Some(2), None, None, None, None],
        ]
    }

    #[test]
    fn test_kurodoko_problem() {
        let problem = problem_for_tests();
        let ans = solve_kurodoko(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 0, 1, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 1, 0],
            [0, 0, 0, 1, 0, 0],
            [1, 0, 1, 0, 0, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_kurodoko_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?kurodoko/6/5/h2j2h3r5g2j";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
