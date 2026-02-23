use crate::puzzles::loop_common::add_full_loop_constraints;
use crate::puzzles::walk_common::{merge_walk_answers, walk_not_passing_colored_cell};
use crate::util;
use cspuz_rs::complex_constraints::walk_line_size;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, Map, MultiDigit, Optionalize, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_icewalk(
    full: bool,
    icebarn: &[Vec<bool>],
    num: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(icebarn);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_expr(is_line.horizontal.any() | is_line.vertical.any());
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let (is_passed, is_cross) = graph::crossable_single_cycle_grid_edges(&mut solver, is_line);
    for y in 0..h {
        for x in 0..w {
            if num[y][x].is_some() {
                solver.add_expr(is_passed.at((y, x)));
            }
            if icebarn[y][x] {
                if x == 0 {
                    solver.add_expr(!is_line.horizontal.at((y, x)));
                } else if x == w - 1 {
                    solver.add_expr(!is_line.horizontal.at((y, x - 1)));
                } else {
                    solver.add_expr(
                        is_line
                            .horizontal
                            .at((y, x - 1))
                            .iff(is_line.horizontal.at((y, x))),
                    );
                }
                if y == 0 {
                    solver.add_expr(!is_line.vertical.at((y, x)));
                } else if y == h - 1 {
                    solver.add_expr(!is_line.vertical.at((y - 1, x)));
                } else {
                    solver.add_expr(
                        is_line
                            .vertical
                            .at((y - 1, x))
                            .iff(is_line.vertical.at((y, x))),
                    );
                }
            } else {
                solver.add_expr(!is_cross.at((y, x)));
            }
        }
    }

    if full {
        add_full_loop_constraints(&mut solver, is_line, h - 1, w - 1);
    }

    let line_size = &walk_line_size(&mut solver, &is_line, icebarn, false);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = num[y][x] {
                if n >= 0 {
                    solver.add_expr(line_size.at((y, x)).eq(n));
                }
            }
        }
    }

    let ans1 = solver.irrefutable_facts().map(|f| f.get(is_line));
    let ans2 = walk_not_passing_colored_cell(full, icebarn, num);
    merge_walk_answers(ans1, ans2)
}

type Problem = (bool, (Vec<Vec<bool>>, Vec<Vec<Option<i32>>>));

fn combinator() -> impl Combinator<Problem> {
    Tuple2::new(
        Choice::new(vec![
            Box::new(Dict::new(true, "f/")),
            Box::new(Dict::new(false, "")),
        ]),
        Size::new(Tuple2::new(
            ContextBasedGrid::new(Map::new(
                MultiDigit::new(2, 5),
                |x| Some(if x { 1 } else { 0 }),
                |x| Some(x == 1),
            )),
            ContextBasedGrid::new(Choice::new(vec![
                Box::new(Optionalize::new(HexInt)),
                Box::new(Spaces::new(None, 'g')),
                Box::new(Dict::new(Some(-1), ".")),
            ])),
        )),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem.1 .0);
    problem_to_url_with_context_pzprxs(
        combinator(),
        "icewalk",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["icewalk"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        (
            false,
            (
                crate::util::tests::to_bool_2d([
                    [1, 0, 0, 0, 0, 0],
                    [0, 1, 1, 0, 0, 0],
                    [0, 1, 1, 0, 1, 0],
                    [0, 0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0, 1],
                    [1, 1, 0, 1, 0, 1],
                    [0, 0, 0, 0, 0, 0],
                ]),
                vec![
                    vec![None, None, None, None, None, None],
                    vec![Some(2), None, None, Some(2), None, None],
                    vec![None, None, None, Some(3), None, None],
                    vec![None, None, None, None, None, None],
                    vec![None, None, Some(5), None, Some(1), None],
                    vec![None, None, None, None, Some(3), None],
                    vec![None, None, None, None, None, Some(3)],
                ],
            ),
        )
    }

    fn problem_for_tests2() -> Problem {
        (
            true,
            (
                crate::util::tests::to_bool_2d([[0, 0, 0], [0, 0, 0]]),
                vec![vec![None, None, None], vec![None, None, None]],
            ),
        )
    }

    #[test]
    fn test_icewalk_problem1() {
        let (full, (icebarn, num)) = problem_for_tests1();
        let ans = solve_icewalk(full, &icebarn, &num);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 1, 1],
                [1, 1, 1, 1, 0],
                [1, 1, 1, 0, 0],
                [1, 0, 1, 0, 0],
                [0, 1, 1, 1, 0],
                [0, 0, 1, 1, 0],
                [1, 1, 0, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 0, 0, 1],
                [1, 0, 1, 0, 1, 1],
                [0, 0, 1, 1, 1, 1],
                [1, 1, 0, 0, 1, 1],
                [1, 0, 0, 0, 0, 1],
                [1, 0, 1, 0, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_icerwalk_problem2() {
        let (full, (ice, num)) = problem_for_tests2();
        let ans = solve_icewalk(full, &ice, &num);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([[1, 1], [1, 1]]),
            vertical: crate::util::tests::to_option_bool_2d([[1, 0, 1]]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_icewalk_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://pzprxs.vercel.app/p?icewalk/6/7/g63845qg0l2h2k3p5g1k3l3";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://pzprxs.vercel.app/p?icewalk/f/3/2/00l";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
