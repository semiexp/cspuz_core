use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::{Solver, false_};

pub fn solve_geradeweg(clues: &[Vec<Option<i32>>]) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, &is_line);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(is_passed.at((y, x)));

                if n > 0 {
                    let has_left = if x > 0 {
                        is_line.horizontal.at((y, x - 1)).expr()
                    } else {
                        false_()
                    };
                    let has_right = if x < w - 1 {
                        is_line.horizontal.at((y, x)).expr()
                    } else {
                        false_()
                    };

                    solver.add_expr(
                        (has_left | has_right).imp(
                            (is_line
                                .horizontal
                                .slice_fixed_y((y, ..x))
                                .reverse()
                                .consecutive_prefix_true()
                                + is_line
                                    .horizontal
                                    .slice_fixed_y((y, x..))
                                    .consecutive_prefix_true())
                            .eq(n),
                        ),
                    );

                    let has_up = if y > 0 {
                        is_line.vertical.at((y - 1, x)).expr()
                    } else {
                        false_()
                    };
                    let has_down = if y < h - 1 {
                        is_line.vertical.at((y, x)).expr()
                    } else {
                        false_()
                    };

                    solver.add_expr(
                        (has_up | has_down).imp(
                            (is_line
                                .vertical
                                .slice_fixed_x((..y, x))
                                .reverse()
                                .consecutive_prefix_true()
                                + is_line
                                    .vertical
                                    .slice_fixed_x((y.., x))
                                    .consecutive_prefix_true())
                            .eq(n),
                        ),
                    );
                }
            }
        }
    }
    solver.irrefutable_facts().map(|f| f.get(is_line))
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
    problem_to_url(combinator(), "geradeweg", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["geradeweg"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![None, None, None, None, Some(1), None],
            vec![None, Some(2), Some(3), None, None, None],
            vec![None, None, None, None, Some(-1), None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, Some(4)],
        ]
    }

    #[test]
    fn test_geradeweg_problem() {
        let problem = problem_for_tests();
        let ans = solve_geradeweg(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 0, 1],
                [0, 0, 0, 1, 0],
                [1, 0, 0, 1, 0],
                [1, 0, 1, 1, 0],
                [0, 1, 1, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 0, 1, 1],
                [0, 1, 1, 1, 0, 1],
                [1, 0, 1, 0, 1, 1],
                [0, 1, 0, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_geradeweg_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?geradeweg/6/5/j1h23m.r4";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
