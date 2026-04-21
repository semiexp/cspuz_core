use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_canalview(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    graph::active_vertices_connected_2d(&mut solver, is_black);
    solver.add_expr(!is_black.conv2d_and((2, 2)));

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                if n < 0 {
                    continue;
                }
                let up = is_black.slice_fixed_x((..y, x)).reverse();
                let down = is_black.slice_fixed_x(((y + 1).., x));
                let left = is_black.slice_fixed_y((y, ..x)).reverse();
                let right = is_black.slice_fixed_y((y, (x + 1)..));
                solver.add_expr(
                    (up.consecutive_prefix_true()
                        + down.consecutive_prefix_true()
                        + left.consecutive_prefix_true()
                        + right.consecutive_prefix_true())
                    .eq(n),
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
    problem_to_url(combinator(), "canal", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["canal"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(3), None, None, None, Some(3), None],
            vec![None, Some(2), None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, Some(4), None],
            vec![None, Some(5), None, None, None, Some(2)],
        ]
    }

    #[test]
    fn test_canalview_problem() {
        let problem = problem_for_tests();
        let ans = solve_canalview(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 1, 1, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 1, 0, 1, 0],
            [0, 1, 1, 1, 1, 1],
            [1, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_canalview_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?canal/6/6/3i3h2z4h5i2";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
