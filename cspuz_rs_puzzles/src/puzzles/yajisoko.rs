use crate::util;
use cspuz_rs::graph;
use cspuz_rs::items::{Arrow, NumberedArrow};
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context, Dict, Grid,
    HexInt, Map, Spaces,
};
use cspuz_rs::solver::{any, count_true, Solver};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YajisokoCell {
    Empty,
    Block,
    Arrow(NumberedArrow),
}

pub fn solve_yajisoko(
    clues: &[Vec<YajisokoCell>],
) -> Option<(graph::BoolGridEdgesIrrefutableFacts, Vec<Vec<Option<bool>>>)> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let direction = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    let up = &(&is_line.vertical & &direction.vertical);
    let down = &(&is_line.vertical & !&direction.vertical);
    let left = &(&is_line.horizontal & &direction.horizontal);
    let right = &(&is_line.horizontal & !&direction.horizontal);

    let block_after_move = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(block_after_move);

    // no turn
    if h >= 2 && w >= 2 {
        for y in 0..h {
            for x in 0..w {
                let has_vertical = if y == 0 {
                    is_line.vertical.at((y, x)).expr()
                } else if y == h - 1 {
                    is_line.vertical.at((y - 1, x)).expr()
                } else {
                    is_line.vertical.at((y - 1, x)) | is_line.vertical.at((y, x))
                };
                let has_horizontal = if x == 0 {
                    is_line.horizontal.at((y, x)).expr()
                } else if x == w - 1 {
                    is_line.horizontal.at((y, x - 1)).expr()
                } else {
                    is_line.horizontal.at((y, x - 1)) | is_line.horizontal.at((y, x))
                };

                solver.add_expr(!(has_vertical & has_horizontal));
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            let mut inbound = vec![];
            let mut outbound = vec![];

            if y > 0 {
                inbound.push(down.at((y - 1, x)));
                outbound.push(up.at((y - 1, x)));
            }
            if y < h - 1 {
                inbound.push(up.at((y, x)));
                outbound.push(down.at((y, x)));
            }
            if x > 0 {
                inbound.push(right.at((y, x - 1)));
                outbound.push(left.at((y, x - 1)));
            }
            if x < w - 1 {
                inbound.push(left.at((y, x)));
                outbound.push(right.at((y, x)));
            }

            if clues[y][x] == YajisokoCell::Block {
                solver.add_expr(!any(&inbound));
                solver.add_expr(count_true(&outbound).eq(block_after_move.at((y, x)).ite(0, 1)));
            } else {
                let n_inbound = solver.int_var(0, 1);
                let n_outbound = solver.int_var(0, 1);

                solver.add_expr(n_inbound.eq(count_true(&inbound)));
                solver.add_expr(n_outbound.eq(count_true(&outbound)));
                solver.add_expr(n_inbound.eq(n_outbound + block_after_move.at((y, x)).ite(1, 0)));
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if let YajisokoCell::Arrow((dir, n)) = clues[y][x] {
                let cells = block_after_move.pointing_cells((y, x), dir).unwrap();
                solver.add_expr((!block_after_move.at((y, x))).imp(cells.count_true().eq(n)));
            }
        }
    }
    solver
        .irrefutable_facts()
        .map(|f| (f.get(is_line), f.get(block_after_move)))
}

type Problem = Vec<Vec<YajisokoCell>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Map::new(
            HexInt,
            |clue| match clue {
                YajisokoCell::Arrow((dir, n)) => Some(
                    n * 5
                        + 10
                        + match dir {
                            Arrow::Unspecified => 0,
                            Arrow::Up => 1,
                            Arrow::Down => 2,
                            Arrow::Left => 3,
                            Arrow::Right => 4,
                        },
                ),
                _ => None,
            },
            |x| {
                if x >= 10 {
                    let dir = match (x - 10) % 5 {
                        0 => Arrow::Unspecified,
                        1 => Arrow::Up,
                        2 => Arrow::Down,
                        3 => Arrow::Left,
                        4 => Arrow::Right,
                        _ => unreachable!(),
                    };
                    let n = (x - 10) / 5;
                    Some(YajisokoCell::Arrow((dir, n)))
                } else {
                    None
                }
            },
        )),
        Box::new(Spaces::new(YajisokoCell::Empty, 'g')),
        Box::new(Dict::new(YajisokoCell::Block, ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem);
    problem_to_url_with_context_and_site(
        combinator(),
        "yajisoko",
        "https://pzprxs.vercel.app/p?",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["yajisoko"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        vec![
            vec![YajisokoCell::Arrow((Arrow::Down, 1)), YajisokoCell::Arrow((Arrow::Right, 4)), YajisokoCell::Arrow((Arrow::Down, 1)), YajisokoCell::Empty, YajisokoCell::Empty],
            vec![YajisokoCell::Block, YajisokoCell::Arrow((Arrow::Right, 3)), YajisokoCell::Empty, YajisokoCell::Block, YajisokoCell::Empty],
            vec![YajisokoCell::Empty, YajisokoCell::Block, YajisokoCell::Block, YajisokoCell::Empty, YajisokoCell::Arrow((Arrow::Left, 2))],
            vec![YajisokoCell::Block, YajisokoCell::Empty, YajisokoCell::Block, YajisokoCell::Block, YajisokoCell::Block],
        ]
    }

    #[test]
    fn test_yajisoko_problem() {
        let clues = problem_for_tests();
        let ans = solve_yajisoko(&clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let is_line_expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 1, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 1, 0, 0, 0],
                [0, 1, 1, 0, 1],
                [1, 0, 0, 1, 1],
            ]),
        };
        let has_block_expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 0, 0, 0],
            [0, 0, 1, 1, 1],
            [1, 0, 0, 1, 0],
            [0, 1, 0, 0, 0],
        ]);

        assert_eq!(ans, (is_line_expected, has_block_expected));
    }

    #[test]
    fn test_waterwalk_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?yajisoko/5/4/-11-22-11h.-1dg.h..g-17.g...";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
