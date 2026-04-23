use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Grid, HexInt, Map, Optionalize, Spaces,
};
use cspuz_rs::solver::{Solver, FALSE};

pub fn solve_balance_loop(
    clues: &[Vec<Option<(i32, bool)>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, is_line);

    for y in 0..h {
        for x in 0..w {
            let Some((n, is_black)) = clues[y][x] else {
                continue;
            };

            solver.add_expr(is_passed.at((y, x)));

            let has_left = if x > 0 {
                is_line.horizontal.at((y, x - 1)).expr()
            } else {
                FALSE
            };
            let has_right = if x < w - 1 {
                is_line.horizontal.at((y, x)).expr()
            } else {
                FALSE
            };
            let has_up = if y > 0 {
                is_line.vertical.at((y - 1, x)).expr()
            } else {
                FALSE
            };
            let has_down = if y < h - 1 {
                is_line.vertical.at((y, x)).expr()
            } else {
                FALSE
            };

            let left_len = is_line
                .horizontal
                .slice_fixed_y((y, ..x))
                .reverse()
                .consecutive_prefix_true();
            let right_len = is_line.horizontal.slice_fixed_y((y, x..)).consecutive_prefix_true();
            let up_len = is_line
                .vertical
                .slice_fixed_x((..y, x))
                .reverse()
                .consecutive_prefix_true();
            let down_len = is_line.vertical.slice_fixed_x((y.., x)).consecutive_prefix_true();

            solver.add_expr(
                (left_len.clone() + right_len.clone() + up_len.clone() + down_len.clone()).eq(n),
            );

            let rel_left_right = if is_black {
                left_len.clone().ne(right_len.clone())
            } else {
                left_len.clone().eq(right_len.clone())
            };
            solver.add_expr((has_left.clone() & has_right.clone()).imp(rel_left_right));

            let rel_left_up = if is_black {
                left_len.clone().ne(up_len.clone())
            } else {
                left_len.clone().eq(up_len.clone())
            };
            solver.add_expr((has_left.clone() & has_up.clone()).imp(rel_left_up));

            let rel_left_down = if is_black {
                left_len.clone().ne(down_len.clone())
            } else {
                left_len.clone().eq(down_len.clone())
            };
            solver.add_expr((has_left.clone() & has_down.clone()).imp(rel_left_down));

            let rel_right_up = if is_black {
                right_len.clone().ne(up_len.clone())
            } else {
                right_len.clone().eq(up_len.clone())
            };
            solver.add_expr((has_right.clone() & has_up.clone()).imp(rel_right_up));

            let rel_right_down = if is_black {
                right_len.clone().ne(down_len.clone())
            } else {
                right_len.clone().eq(down_len.clone())
            };
            solver.add_expr((has_right.clone() & has_down.clone()).imp(rel_right_down));

            let rel_up_down = if is_black {
                up_len.clone().ne(down_len.clone())
            } else {
                up_len.clone().eq(down_len.clone())
            };
            solver.add_expr((has_up & has_down).imp(rel_up_down));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = Vec<Vec<Option<(i32, bool)>>>;

fn clue_combinator() -> impl Combinator<(i32, bool)> {
    Map::new(
        HexInt,
        |(n, is_black): (i32, bool)| Some(2 * n + if is_black { 1 } else { 0 }),
        |v: i32| Some((v / 2, v % 2 == 1)),
    )
}

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(clue_combinator())),
        Box::new(Spaces::new(None, 'g')),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "balance", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["balance"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some((4, false)), Some((2, false)), Some((4, false))],
            vec![Some((2, false)), None, Some((2, false))],
            vec![Some((4, false)), Some((2, false)), Some((4, false))],
        ]
    }

    #[test]
    fn test_balance_loop_problem() {
        let problem = problem_for_tests();
        let ans = solve_balance_loop(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([[1, 1], [0, 0], [1, 1]]),
            vertical: crate::util::tests::to_option_bool_2d([[1, 0, 1], [1, 0, 1]]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_balance_loop_serializer() {
        let problem = problem_for_tests();
        let url = serialize_problem(&problem).unwrap();
        assert_eq!(deserialize_problem(&url), Some(problem));
    }
}
