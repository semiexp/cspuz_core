use crate::penpa_editor::{decode_penpa_editor_url, Item, PenpaEditorPuzzle};
use crate::util;
use cspuz_rs::graph;
use cspuz_rs::items::Arrow;
use cspuz_rs::solver::{count_true, Solver, FALSE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastleWalkerSquare {
    None,
    Black,
    Gray,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastleWalkerClue {
    Cross,
    Cell {
        square: CastleWalkerSquare,
        number: Option<i32>,
        arrow: Arrow,
    },
}

pub fn solve_castle_walker(
    clues: &[Vec<CastleWalkerClue>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let (is_passed, is_cross) = graph::crossable_single_cycle_grid_edges(&mut solver, is_line);

    let direction = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    let up = &(&is_line.vertical & &direction.vertical);
    let down = &(&is_line.vertical & !&direction.vertical);
    let left = &(&is_line.horizontal & &direction.horizontal);
    let right = &(&is_line.horizontal & !&direction.horizontal);

    for y in 0..h {
        for x in 0..w {
            let mut inbound = vec![];
            let mut outbound = vec![];
            if y > 0 {
                inbound.push(is_line.vertical.at((y - 1, x)) & !direction.vertical.at((y - 1, x)));
                outbound.push(up.at((y - 1, x)));
            }
            if y < h - 1 {
                inbound.push(is_line.vertical.at((y, x)) & direction.vertical.at((y, x)));
                outbound.push(down.at((y, x)));
            }
            if x > 0 {
                inbound
                    .push(is_line.horizontal.at((y, x - 1)) & !direction.horizontal.at((y, x - 1)));
                outbound.push(left.at((y, x - 1)));
            }
            if x < w - 1 {
                inbound.push(is_line.horizontal.at((y, x)) & direction.horizontal.at((y, x)));
                outbound.push(right.at((y, x)));
            }
            solver.add_expr(count_true(&inbound).eq(count_true(&outbound)));
        }
    }

    for y in 0..h {
        for x in 0..w {
            if y == 0 || y == h - 1 || x == 0 || x == w - 1 {
                solver.add_expr(!is_cross.at((y, x)));
            } else {
                solver.add_expr(is_cross.at((y, x)).imp(
                    up.at((y - 1, x)).iff(up.at((y, x)))
                        & down.at((y - 1, x)).iff(down.at((y, x)))
                        & left.at((y, x - 1)).iff(left.at((y, x)))
                        & right.at((y, x - 1)).iff(right.at((y, x))),
                ));
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            match clues[y][x] {
                CastleWalkerClue::Cross => {
                    solver.add_expr(is_cross.at((y, x)));
                }
                CastleWalkerClue::Cell {
                    square,
                    number,
                    arrow,
                } => {
                    solver.add_expr(!is_cross.at((y, x)));

                    match square {
                        CastleWalkerSquare::None => {}
                        CastleWalkerSquare::Black => {
                            solver.add_expr(
                                is_line.vertical.at_offset((y, x), (-1, 0), FALSE)
                                    ^ is_line.vertical.at_offset((y, x), (0, 0), FALSE),
                            );
                            solver.add_expr(
                                is_line.horizontal.at_offset((y, x), (0, -1), FALSE)
                                    ^ is_line.horizontal.at_offset((y, x), (0, 0), FALSE),
                            );
                        }
                        CastleWalkerSquare::Gray => {
                            solver.add_expr(is_passed.at((y, x)));
                        }
                        CastleWalkerSquare::White => {
                            solver.add_expr(
                                (is_line.vertical.at_offset((y, x), (-1, 0), FALSE)
                                    & is_line.vertical.at_offset((y, x), (0, 0), FALSE))
                                    | (is_line.horizontal.at_offset((y, x), (0, -1), FALSE)
                                        & is_line.horizontal.at_offset((y, x), (0, 0), FALSE)),
                            );
                        }
                    }

                    if let Some(number) = number {
                        let nl = is_line
                            .horizontal
                            .slice_fixed_y((y, ..x))
                            .reverse()
                            .consecutive_prefix_true();
                        let nr = is_line
                            .horizontal
                            .slice_fixed_y((y, x..))
                            .consecutive_prefix_true();
                        let nu = is_line
                            .vertical
                            .slice_fixed_x((..y, x))
                            .reverse()
                            .consecutive_prefix_true();
                        let nd = is_line
                            .vertical
                            .slice_fixed_x((y.., x))
                            .consecutive_prefix_true();
                        let count = nl + nr + nu + nd;
                        solver.add_expr(count.eq(number));
                    }

                    match arrow {
                        Arrow::Unspecified => {}
                        Arrow::Up => {
                            if y > 0 {
                                solver.add_expr(up.at((y - 1, x)));
                            } else {
                                return None;
                            }
                        }
                        Arrow::Down => {
                            if y < h - 1 {
                                solver.add_expr(down.at((y, x)));
                            } else {
                                return None;
                            }
                        }
                        Arrow::Left => {
                            if x > 0 {
                                solver.add_expr(left.at((y, x - 1)));
                            } else {
                                return None;
                            }
                        }
                        Arrow::Right => {
                            if x < w - 1 {
                                solver.add_expr(right.at((y, x)));
                            } else {
                                return None;
                            }
                        }
                    }
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = Vec<Vec<CastleWalkerClue>>;

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let decoded = decode_penpa_editor_url(url).ok()?;
    #[allow(unreachable_patterns)]
    let decoded = match decoded {
        PenpaEditorPuzzle::Square(s) => s,
        _ => return None,
    };

    let height = decoded.height();
    let width = decoded.width();

    let mut is_cross = vec![vec![false; width]; height];
    let mut square = vec![vec![CastleWalkerSquare::None; width]; height];
    let mut number = vec![vec![None; width]; height];
    let mut arrow = vec![vec![Arrow::Unspecified; width]; height];

    for y in 0..decoded.height() {
        for x in 0..decoded.width() {
            for item in decoded.get_cell(y, x) {
                if let Item::Symbol(symbol) = item {
                    if symbol.name.starts_with("square_") {
                        square[y][x] = match symbol.color_id {
                            1 => CastleWalkerSquare::White,
                            2 => CastleWalkerSquare::Black,
                            3 => CastleWalkerSquare::Gray,
                            _ => return None,
                        };
                    }
                    if symbol.name == "zL" {
                        is_cross[y][x] = true;
                    }
                }
                if let Item::Text(text) = item {
                    let parts = text.text.split('_').collect::<Vec<_>>();
                    if parts.len() != 1 && parts.len() != 2 {
                        return None;
                    }
                    let num_part = if parts[0].is_empty() {
                        None
                    } else {
                        Some(parts[0].parse().ok()?)
                    };
                    let arrow_part = if parts.len() == 2 {
                        match parts[1] {
                            "0" => Arrow::Up,
                            "1" => Arrow::Left,
                            "2" => Arrow::Right,
                            "3" => Arrow::Down,
                            _ => return None,
                        }
                    } else {
                        Arrow::Unspecified
                    };
                    number[y][x] = num_part;
                    arrow[y][x] = arrow_part;
                }
            }
        }
    }

    let mut ret = vec![vec![CastleWalkerClue::Cross; width]; height];
    for y in 0..height {
        for x in 0..width {
            if is_cross[y][x] {
                ret[y][x] = CastleWalkerClue::Cross;
            } else {
                ret[y][x] = CastleWalkerClue::Cell {
                    square: square[y][x],
                    number: number[y][x],
                    arrow: arrow[y][x],
                };
            }
        }
    }

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://puzsq.logicpuzzle.app/puzzle/166723
    fn problem_for_tests() -> Problem {
        let mut ret = vec![
            vec![
                CastleWalkerClue::Cell {
                    square: CastleWalkerSquare::None,
                    number: None,
                    arrow: Arrow::Unspecified,
                };
                6
            ];
            6
        ];

        ret[0][4] = CastleWalkerClue::Cell {
            square: CastleWalkerSquare::Gray,
            number: Some(3),
            arrow: Arrow::Left,
        };
        ret[1][5] = CastleWalkerClue::Cell {
            square: CastleWalkerSquare::Gray,
            number: None,
            arrow: Arrow::Up,
        };
        ret[2][1] = CastleWalkerClue::Cell {
            square: CastleWalkerSquare::Black,
            number: None,
            arrow: Arrow::Unspecified,
        };
        ret[2][4] = CastleWalkerClue::Cell {
            square: CastleWalkerSquare::White,
            number: None,
            arrow: Arrow::Left,
        };
        ret[3][2] = CastleWalkerClue::Cross;
        ret[4][1] = CastleWalkerClue::Cross;
        ret[4][4] = CastleWalkerClue::Cell {
            square: CastleWalkerSquare::Gray,
            number: None,
            arrow: Arrow::Left,
        };
        ret[5][0] = CastleWalkerClue::Cell {
            square: CastleWalkerSquare::Gray,
            number: Some(2),
            arrow: Arrow::Right,
        };
        ret
    }

    #[test]
    fn test_castle_walker_problem() {
        let problem = problem_for_tests();
        let ans = solve_castle_walker(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 1, 1],
                [0, 1, 0, 1, 1],
                [0, 1, 0, 1, 1],
                [0, 1, 1, 1, 0],
                [1, 1, 0, 1, 0],
                [1, 0, 0, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 0, 0, 1],
                [0, 1, 0, 1, 0, 0],
                [0, 0, 1, 0, 0, 1],
                [0, 1, 1, 0, 1, 1],
                [1, 1, 0, 1, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_castle_walker_serializer() {
        let problem = problem_for_tests();
        let url = "https://opt-pan.github.io/penpa-edit/?m=solve&p=tZTPb5swFMfv/BWTzz6AIaTllnXNLiz7lamqEEJOQhtUEncG1spR+rf3+ZkKDOywaRPy0+PDw++L7S/Vz4bLnIZw+RfUpR5cLAxxeEGAw22vdVGXefSOLpp6LyQklH5eLukdL6vcSdqq1EmIRyhhMDySvqj4BYGfOif1LTqpLErSM1U/uvSiS79HJ4ir6ERYSKKE+Bm86unZUkr8uUaZ25EAi/o14YjMmSYsA0EGQQMP29y+tfEpMeuQxVD01mlEAx8oG1E9A8xs01kAdEZVjHehfrO7m+yJOgcURC5RKsO4hkWiysf4AaOLcYYxxpprjDcYrzAGGEOsmetldpyEMdxpc83+Pofdhn0llSizqpF3fJuTCM8DfAewY3PY5NJCpRCPZXG064r7o5B596iWTa88391PlW+E3A0mf+JlaQGzmBbaFnJb2qiWhXXPpRRPFjnwem+BDa/BDNW+eLRnyo+1LaDmtkT+wAfdDt1qnB3yTHDAOWPgQljgk7qM1IKqj3CKe8ai6ivY5lOkVto1CcFDBGcAixik1116g891dmWg50K+0rYwr91Cap1G9SVK1JwS3ec9vq1TchC/QKrRoe+34rCBj0lIbznMk6rZiYemrcWDvDBy4wm5fidXp0auzoZy2+/Rcs029uWu/5Xcy/RsNsL9o19W/7/yn4z73HpNyEm7AZ5wHNBJ07V85C7gIx/phmMrAZ1wE9ChoQCNPQVwZCtgv3GWnnVoLq1q6C/damQx3arvsiR1XgE=&a=RZBRCsRQCAPv0u/8rBo9TOn9r9HsxsdCwUEzedD7fnBfUQjic+FLWSYi2tSIMQ3SlHlyokoTkTZE5T4t1lCobGhwSTkbGrW5AU3MkxO139CCbhb1Xgm6RaOXBu2WjuOKJkzqsyuavfK4ovEbqhi3SNycTuOcxu+qP/j/nhc=";
        assert_eq!(deserialize_problem(url).unwrap(), problem);
    }
}
