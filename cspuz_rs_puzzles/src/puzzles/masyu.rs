use crate::puzzles::loop_common::add_full_loop_constraints;
use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, Map, MultiDigit, Tuple2,
};
use cspuz_rs::solver::Solver;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasyuClue {
    None,
    White,
    Black,
}

pub fn solve_masyu(
    full: bool,
    clues: &[Vec<MasyuClue>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    graph::single_cycle_grid_edges(&mut solver, is_line);

    for y in 0..h {
        for x in 0..w {
            let p = (y, x);
            match clues[y][x] {
                MasyuClue::None => (),
                MasyuClue::White => {
                    solver.add_expr(
                        (is_line.vertical.at_offset(p, (-1, 0), false)
                            & is_line.vertical.at_offset(p, (0, 0), false)
                            & !(is_line.vertical.at_offset(p, (-2, 0), false)
                                & is_line.vertical.at_offset(p, (1, 0), false)))
                            | (is_line.horizontal.at_offset(p, (0, -1), false)
                                & is_line.horizontal.at_offset(p, (0, 0), false)
                                & !(is_line.horizontal.at_offset(p, (0, -2), false)
                                    & is_line.horizontal.at_offset(p, (0, 1), false))),
                    );
                }
                MasyuClue::Black => {
                    solver.add_expr(
                        (is_line.vertical.at_offset(p, (-2, 0), false)
                            & is_line.vertical.at_offset(p, (-1, 0), false))
                            | (is_line.vertical.at_offset(p, (0, 0), false)
                                & is_line.vertical.at_offset(p, (1, 0), false)),
                    );
                    solver.add_expr(
                        (is_line.horizontal.at_offset(p, (0, -2), false)
                            & is_line.horizontal.at_offset(p, (0, -1), false))
                            | (is_line.horizontal.at_offset(p, (0, 0), false)
                                & is_line.horizontal.at_offset(p, (0, 1), false)),
                    );
                }
            }
        }
    }

    if full {
        add_full_loop_constraints(&mut solver, is_line, h - 1, w - 1);
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = (bool, Vec<Vec<MasyuClue>>);

fn combinator() -> impl Combinator<Problem> {
    Tuple2::new(
        Choice::new(vec![
            Box::new(Dict::new(true, "f/")),
            Box::new(Dict::new(false, "")),
        ]),
        Grid::new(Map::new(
            MultiDigit::new(3, 3),
            |x: MasyuClue| {
                Some(match x {
                    MasyuClue::None => 0,
                    MasyuClue::White => 1,
                    MasyuClue::Black => 2,
                })
            },
            |n: i32| match n {
                0 => Some(MasyuClue::None),
                1 => Some(MasyuClue::White),
                2 => Some(MasyuClue::Black),
                _ => None,
            },
        )),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "masyu", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["masyu", "mashu"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        // https://puzsq.jp/main/puzzle_play.php?pid=9833
        let mut ret = vec![vec![MasyuClue::None; 10]; 10];
        ret[0][4] = MasyuClue::Black;
        ret[1][9] = MasyuClue::White;
        ret[2][1] = MasyuClue::Black;
        ret[2][8] = MasyuClue::Black;
        ret[3][0] = MasyuClue::White;
        ret[3][2] = MasyuClue::Black;
        ret[3][5] = MasyuClue::White;
        ret[3][7] = MasyuClue::White;
        ret[4][6] = MasyuClue::Black;
        ret[6][3] = MasyuClue::White;
        ret[6][5] = MasyuClue::White;
        ret[6][7] = MasyuClue::White;
        ret[7][3] = MasyuClue::Black;
        ret[8][1] = MasyuClue::Black;
        ret[8][7] = MasyuClue::White;
        ret[9][4] = MasyuClue::White;
        ret[9][7] = MasyuClue::White;
        (false, ret)
    }

    fn problem_for_tests2() -> Problem {
        let mut ret = vec![vec![MasyuClue::None; 4]; 4];
        ret[2][2] = MasyuClue::White;
        ret[3][0] = MasyuClue::Black;
        (true, ret)
    }

    #[test]
    fn test_masyu_problem1() {
        let (full, problem) = problem_for_tests1();
        let ans = solve_masyu(full, &problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        for y in 0..problem.len() {
            for x in 0..problem[0].len() {
                if y + 1 < problem.len() {
                    assert!(ans.vertical[y][x].is_some());
                }
                if x + 1 < problem[0].len() {
                    assert!(ans.horizontal[y][x].is_some());
                }
            }
        }
        assert_eq!(ans.horizontal[4][0], Some(true));
        assert_eq!(ans.horizontal[0][4], Some(false));
    }

    #[test]
    fn test_masyu_problem2() {
        let (full, problem) = problem_for_tests2();
        let ans = solve_masyu(full, &problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        #[rustfmt::skip]
        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1],
                [0, 1, 1],
                [0, 1, 1],
                [1, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 1],
                [1, 1, 0, 0],
                [1, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_masyu_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://puzz.link/p?masyu/10/10/0600003i06b1300600000a30600i090330";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://puzz.link/p?masyu/f/4/4/0003i0";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
