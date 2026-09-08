use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, HexInt, Map, NumSpaces, Size, Spaces, Tuple3,
};
use cspuz_rs::solver::{count_true, Solver, TRUE};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GuidearrowClue {
    Up,
    Down,
    Left,
    Right,
    Unknown, // "?"
}

pub fn solve_guidearrow(
    ty: usize,
    tx: usize,
    clues: &[Vec<Option<GuidearrowClue>>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);
    graph::active_vertices_connected_2d(&mut solver, !is_black);
    solver.add_expr(!is_black.conv2d_and((1, 2)));
    solver.add_expr(!is_black.conv2d_and((2, 1)));
    solver.add_expr(is_black.conv2d_or((2, 2)));

    let toward_right = &solver.bool_var_2d((h, w - 1));
    let toward_left = &solver.bool_var_2d((h, w - 1));
    let toward_down = &solver.bool_var_2d((h - 1, w));
    let toward_up = &solver.bool_var_2d((h - 1, w));

    for y in 0..h {
        for x in 0..(w - 1) {
            solver.add_expr(
                count_true([toward_right.at((y, x)), toward_left.at((y, x))])
                    .eq((!is_black.at((y, x)) & !is_black.at((y, x + 1))).ite(1, 0)),
            );
        }
    }
    for y in 0..(h - 1) {
        for x in 0..w {
            solver.add_expr(
                count_true([toward_down.at((y, x)), toward_up.at((y, x))])
                    .eq((!is_black.at((y, x)) & !is_black.at((y + 1, x))).ite(1, 0)),
            );
        }
    }

    solver.add_expr(!is_black.at((ty, tx)));
    for y in 0..h {
        for x in 0..w {
            let mut outgoing = vec![];
            if y > 0 {
                outgoing.push(toward_up.at((y - 1, x)));
            }
            if y < h - 1 {
                outgoing.push(toward_down.at((y, x)));
            }
            if x > 0 {
                outgoing.push(toward_left.at((y, x - 1)));
            }
            if x < w - 1 {
                outgoing.push(toward_right.at((y, x)));
            }
            let n_outgoing = count_true(outgoing);
            solver.add_expr(if (y, x) == (ty, tx) {
                n_outgoing.eq(0)
            } else {
                n_outgoing.eq((!is_black.at((y, x))).ite(1, 0))
            });

            if let Some(clue) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                match clue {
                    GuidearrowClue::Up => {
                        if y == 0 {
                            return None;
                        }
                        solver.add_expr(!is_black.at((y - 1, x)));
                        solver.add_expr(toward_up.at((y - 1, x)));
                    }
                    GuidearrowClue::Down => {
                        if y == h - 1 {
                            return None;
                        }
                        solver.add_expr(!is_black.at((y + 1, x)));
                        solver.add_expr(toward_down.at((y, x)));
                    }
                    GuidearrowClue::Left => {
                        if x == 0 {
                            return None;
                        }
                        solver.add_expr(!is_black.at((y, x - 1)));
                        solver.add_expr(toward_left.at((y, x - 1)));
                    }
                    GuidearrowClue::Right => {
                        if x == w - 1 {
                            return None;
                        }
                        solver.add_expr(!is_black.at((y, x + 1)));
                        solver.add_expr(toward_right.at((y, x)));
                    }
                    _ => (),
                }
            }
        }
    }

    let mut aux_graph = graph::Graph::new(h * w + 1);
    let mut aux_vertices = vec![];

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 {
                if x < w - 1 {
                    aux_graph.add_edge(y * w + x, (y + 1) * w + x + 1);
                }
                if x > 0 {
                    aux_graph.add_edge(y * w + x, (y + 1) * w + x - 1);
                }
            }

            if y == 0 || y == h - 1 || x == 0 || x == w - 1 {
                aux_graph.add_edge(y * w + x, h * w);
            }

            aux_vertices.push(is_black.at((y, x)).expr());
        }
    }
    aux_vertices.push(TRUE);
    graph::active_vertices_connected(&mut solver, &aux_vertices, &aux_graph);

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = (usize, usize, Vec<Vec<Option<GuidearrowClue>>>);

fn combinator() -> impl Combinator<(i32, i32, Vec<Vec<Option<GuidearrowClue>>>)> {
    Size::new(Tuple3::new(
        HexInt,
        HexInt,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Dict::new(Some(GuidearrowClue::Unknown), ".")),
            Box::new(Map::new(
                NumSpaces::new(4, 2),
                |x| match x {
                    Some(GuidearrowClue::Up) => Some(Some(1)),
                    Some(GuidearrowClue::Down) => Some(Some(2)),
                    Some(GuidearrowClue::Left) => Some(Some(3)),
                    Some(GuidearrowClue::Right) => Some(Some(4)),
                    None => Some(None),
                    _ => None,
                },
                |x| match x {
                    Some(1) => Some(Some(GuidearrowClue::Up)),
                    Some(2) => Some(Some(GuidearrowClue::Down)),
                    Some(3) => Some(Some(GuidearrowClue::Left)),
                    Some(4) => Some(Some(GuidearrowClue::Right)),
                    None => Some(None),
                    _ => None,
                },
            )),
            Box::new(Spaces::new(None, 'g')),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem.2);
    let problem = (
        problem.1 as i32 + 1,
        problem.0 as i32 + 1,
        problem.2.clone(),
    );
    problem_to_url_with_context(combinator(), "guidearrow", problem, &Context::sized(h, w))
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let res = url_to_problem(combinator(), &["guidearrow"], url)?;
    Some(((res.1 - 1) as usize, (res.0 - 1) as usize, res.2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        (0, 2,
            vec![
                vec![None, None, None, None, None, Some(GuidearrowClue::Right), None],
                vec![None, Some(GuidearrowClue::Down), None, None, None, None, None],
                vec![None, None, None, None, None, None, None],
                vec![None, None, None, Some(GuidearrowClue::Left), None, None, None],
                vec![None, None, None, None, None, Some(GuidearrowClue::Unknown), None],
                vec![None, None, None, None, None, None, None],
            ]
        )
    }

    #[test]
    fn testguidearrow_problem() {
        let (ty, tx, clues) = problem_for_tests();
        let ans = solve_guidearrow(ty, tx, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 1, 0, 1],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }
    #[test]
    fn test_guidearrow_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?guidearrow/7/6/31kecsdl.n";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
