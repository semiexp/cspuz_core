use std::cmp;
use std::collections::VecDeque;
use std::vec;

use crate::util;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, HexInt, Map, MultiDigit, Optionalize, Size, Spaces, Tuple2, UnlimitedSeq,
};
use cspuz_rs::solver::{sum, Solver};

pub fn solve_bosanowa(
    circles: &[Vec<bool>],
    clues: &Vec<Option<i32>>,
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(circles);

    let mut solver = Solver::new();
    let range = range_optimization(circles, clues);
    let num = &solver.int_var_2d_from_ranges((h, w), &range);
    let mut diff_ranges = vec![vec![(0, 1); 2 * w - 1]; 2 * h - 1];

    for y in 0..(2 * h - 1) {
        // Take the max number on the edge as the max difference
        for x in 0..(2 * w - 1) {
            if (x + y) % 2 == 1 {
                if x % 2 == 1 {
                    if range[y / 2][(x - 1) / 2].1 != -1 && range[y / 2][(x + 1) / 2].1 != -1 {
                        diff_ranges[y][x].1 =
                            cmp::max(range[y / 2][(x - 1) / 2].1, range[y / 2][(x + 1) / 2].1);
                    } else {
                        diff_ranges[y][x].1 = 0;
                    }
                } else {
                    if range[(y - 1) / 2][x / 2].1 != -1 && range[(y + 1) / 2][x / 2].1 != -1 {
                        diff_ranges[y][x].1 =
                            cmp::max(range[(y - 1) / 2][x / 2].1, range[(y + 1) / 2][x / 2].1);
                    } else {
                        diff_ranges[y][x].1 = 0;
                    }
                }
            } else {
                diff_ranges[y][x].1 = 0;
            }
        }
    }

    let diff = &solver.int_var_2d_from_ranges((2 * h - 1, 2 * w - 1), &diff_ranges);
    let is_num = &solver.bool_var_2d((h, w));
    solver.add_answer_key_int(num);

    solver.add_expr(num.ne(0));

    for y in 0..(2 * h - 1) {
        for x in 0..(2 * w - 1) {
            if (x + y) % 2 == 1 {
                if x % 2 == 1 {
                    solver.add_expr(diff.at((y, x)).eq(
                        (is_num.at((y / 2, (x - 1) / 2)) & is_num.at((y / 2, (x + 1) / 2))).ite(
                            (num.at((y / 2, (x - 1) / 2)) - (num.at((y / 2, (x + 1) / 2)))).abs(),
                            0,
                        ),
                    ));
                } else {
                    solver.add_expr(diff.at((y, x)).eq(
                        (is_num.at(((y - 1) / 2, x / 2)) & is_num.at(((y + 1) / 2, x / 2))).ite(
                            (num.at(((y - 1) / 2, x / 2)) - (num.at(((y + 1) / 2, x / 2)))).abs(),
                            0,
                        ),
                    ));
                }
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y * w + x] {
                if n == -1 {
                    solver.add_expr(num.at((y, x)).ge(1));
                } else {
                    solver.add_expr(num.at((y, x)).eq(n));
                }
            } else if circles[y][x] {
                solver.add_expr(num.at((y, x)).ne(-1));
                solver.add_expr(is_num.at((y, x)));
            } else {
                solver.add_expr(num.at((y, x)).eq(-1));
            }

            solver.add_expr(
                is_num
                    .at((y, x))
                    .imp(num.at((y, x)).eq(sum(diff.four_neighbors((2 * y, 2 * x))))),
            );
        }
    }

    solver.irrefutable_facts().map(|f| f.get(num))
}

fn range_optimization(circles: &[Vec<bool>], clues: &Vec<Option<i32>>) -> Vec<Vec<(i32, i32)>> {
    let (h, w) = util::infer_shape(circles);
    let mut range = vec![vec![(-1, -1); w]; h];
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y * w + x] {
                let mut cells = VecDeque::new();
                let mut visited = circles.to_vec();
                visited[y][x] = false;
                range[y][x].1 = n;

                cells.push_back((y, x));
                while cells.len() > 0 {
                    let Some((p, q)) = cells.pop_front() else {
                        break;
                    };
                    let mut neighbors = vec![];
                    if p > 0 {
                        neighbors.push((p - 1, q));
                    }
                    if p < h - 1 {
                        neighbors.push((p + 1, q));
                    }
                    if q > 0 {
                        neighbors.push((p, q - 1));
                    }
                    if q < w - 1 {
                        neighbors.push((p, q + 1));
                    }
                    for (r, s) in neighbors {
                        // A number is surrouned at most by a number twice itself. By propagation, we can determine an upper bound on the numbers
                        if visited[r][s] {
                            visited[r][s] = false;
                            if range[r][s].1 == -1 || range[r][s].1 > 2 * range[p][q].1 {
                                // Upper bound
                                range[r][s].1 = 2 * range[p][q].1;
                                cells.push_back((r, s));
                            }
                        }
                    }
                }
            }
        }
    }

    range
}

type Problem = (Vec<Vec<bool>>, Vec<Option<i32>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        ContextBasedGrid::new(Map::new(
            MultiDigit::new(2, 5),
            |x: bool| match x {
                true => Some(0),
                false => Some(1),
            },
            |n: i32| match n {
                0 => Some(true),
                1 => Some(false),
                _ => None,
            },
        )),
        UnlimitedSeq::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url_with_context(
        combinator(),
        "bosanowa",
        problem.clone(),
        &Context::sized(problem.0.len(), problem.0[0].len()),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["bosanowa"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let circles = crate::util::tests::to_bool_2d([
            [0, 1, 1, 0, 0, 0],
            [0, 1, 1, 1, 1, 0],
            [1, 1, 0, 1, 1, 1],
            [1, 1, 1, 1, 1, 0],
            [0, 1, 1, 0, 0, 0],
        ]);

        let clues = vec![
            None,
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(3),
            None,
            Some(4),
            None,
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
        ];

        (circles, clues)
    }

    #[test]
    fn test_bosanowa_problem() {
        let (circles, clues) = problem_for_tests();
        let ans = solve_bosanowa(&circles, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [-1, 2, 3, -1, -1, -1],
            [-1, 3, 5, 4, 2, -1],
            [6, 3, -1, 3, 2, 1],
            [3, 3, 5, 4, 2, -1],
            [-1, 2, 3, -1, -1, -1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_bosanowa_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?bosanowa/6/5/jo9037g2n2n3g4j3i"; // Example puzzle on puzz.link
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
