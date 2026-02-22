use crate::puzzles::loop_common::force_shaded_outside;
use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_pzprxs, url_to_problem, Choice, Combinator, Dict, Grid, MaybeSkip, NumSpaces,
    Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_koburin(
    outside: bool,
    minesweeper: bool,
    clues: &[Vec<Option<i32>>],
) -> Option<(graph::BoolGridEdgesIrrefutableFacts, Vec<Vec<Option<bool>>>)> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, is_line);
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);
    solver.add_expr(!is_black.conv2d_and((1, 2)));
    solver.add_expr(!is_black.conv2d_and((2, 1)));

    if outside {
        force_shaded_outside(&mut solver, is_black, is_line, h, w);
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_passed.at((y, x)));
                solver.add_expr(!is_black.at((y, x)));

                if n >= 0 {
                    if minesweeper {
                        solver.add_expr(is_black.eight_neighbors((y, x)).count_true().eq(n));
                    } else {
                        solver.add_expr(is_black.four_neighbors((y, x)).count_true().eq(n));
                    }
                }
            } else {
                solver.add_expr(is_passed.at((y, x)) ^ is_black.at((y, x)));
            }
        }
    }

    solver
        .irrefutable_facts()
        .map(|f| (f.get(is_line), f.get(is_black)))
}

type Problem = ((bool, bool), Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Tuple2::new(
        Choice::new(vec![
            Box::new(Dict::new((true, false), "o/")),
            Box::new(Dict::new((true, false), "ob/")),
            Box::new(Dict::new((true, true), "om/")),
            Box::new(Dict::new((true, true), "omb/")),
            Box::new(Dict::new((false, true), "m/")),
            Box::new(Dict::new((false, true), "mb/")),
            Box::new(Dict::new((false, false), "")),
        ]),
        MaybeSkip::new(
            "b/",
            Grid::new(Choice::new(vec![
                Box::new(Dict::new(Some(-1), ".")),
                Box::new(NumSpaces::new(4, 2)),
                Box::new(Spaces::new(None, 'g')),
            ])),
        ),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url_pzprxs(combinator(), "koburin", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["koburin"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        (
            (false, false),
            vec![
                vec![None, None, None, None, Some(-1), None],
                vec![None, Some(2), None, None, None, None],
                vec![None, None, None, None, Some(1), None],
                vec![None, None, None, None, None, None],
                vec![Some(0), None, None, None, None, None],
            ],
        )
    }

    fn problem_for_tests2() -> Problem {
        (
            (true, false),
            vec![
                vec![None, None, None, None, None, None, None],
                vec![None, None, None, Some(2), None, None, None],
                vec![None, None, None, None, None, None, Some(-1)],
                vec![None, None, None, None, None, None, None],
                vec![Some(1), None, None, None, None, None, None],
                vec![None, None, None, Some(0), None, None, None],
                vec![None, None, None, None, None, None, None],
            ],
        )
    }

    fn problem_for_tests3() -> Problem {
        (
            (false, true),
            vec![
                vec![None, None, None, None],
                vec![None, None, None, None],
                vec![Some(-1), None, None, None],
                vec![Some(-1), Some(-1), Some(1), Some(0)],
            ],
        )
    }

    #[test]
    fn test_koburin_problem1() {
        let ((outside, minesweeper), problem) = problem_for_tests1();
        let ans = solve_koburin(outside, minesweeper, &problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected_edges = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 0, 0],
                [0, 0, 0, 1, 1],
                [0, 0, 1, 0, 0],
                [1, 0, 0, 0, 0],
                [0, 1, 0, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 1, 0, 0],
                [1, 0, 0, 0, 0, 1],
                [1, 0, 1, 1, 0, 1],
                [0, 1, 1, 1, 0, 1],
            ]),
        };
        let expected_cells = crate::util::tests::to_option_bool_2d([
            [0, 0, 0, 0, 0, 1],
            [0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0],
        ]);
        assert_eq!(ans, (expected_edges, expected_cells));
    }

    #[test]
    fn test_koburin_problem2() {
        let ((outside, minesweeper), problem) = problem_for_tests2();
        let ans = solve_koburin(outside, minesweeper, &problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected_edges = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 0, 1, 1],
                [1, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0],
                [0, 1, 0, 1, 0, 1],
                [1, 1, 0, 0, 1, 0],
                [1, 1, 1, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 0, 1, 0, 1],
                [1, 0, 1, 0, 1, 1, 0],
                [0, 1, 1, 0, 1, 1, 0],
                [0, 1, 0, 1, 1, 1, 0],
                [0, 0, 1, 0, 0, 0, 1],
                [1, 0, 0, 0, 1, 1, 1],
            ]),
        };
        let expected_cells = crate::util::tests::to_option_bool_2d([
            [1, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
        ]);
        assert_eq!(ans, (expected_edges, expected_cells));
    }

    #[test]
    fn test_koburin_problem3() {
        let ((outside, minesweeper), problem) = problem_for_tests3();
        let ans = solve_koburin(outside, minesweeper, &problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected_edges = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1],
                [1, 1, 0],
                [0, 0, 1],
                [0, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 1],
                [0, 0, 1, 1],
                [0, 0, 0, 0],
            ]),
        };
        let expected_cells = crate::util::tests::to_option_bool_2d([
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0],
        ]);
        assert_eq!(ans, (expected_edges, expected_cells));
    }

    #[test]
    fn test_koburin_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://pzprxs.vercel.app/p?koburin/6/5/j.hclbkai";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://pzprxs.vercel.app/p?koburin/o/7/7/pcm.mbman"; // Credits to Rubricas
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests3();
            let url = "https://pzprxs.vercel.app/p?koburin/m/4/4/n.i..10";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
