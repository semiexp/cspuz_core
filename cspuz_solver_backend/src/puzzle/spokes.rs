use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::spokes;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let clues = spokes::deserialize_problem(url).ok_or("invalid url")?;
    let ans = spokes::solve_spokes(&clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Empty,
        height - 1,
        width - 1,
        ans.as_ref()
            .map_or(Uniqueness::NoAnswer, |a| is_unique(&(&a.0, &a.1, &a.2))),
    );

    if let Some((lines, lines_dr, lines_dl)) = &ans {
        for y in 0..height {
            for x in 0..(width - 1) {
                match lines.horizontal[y][x] {
                    Some(true) => {
                        board.push(Item {
                            y: y * 2,
                            x: x * 2 + 1,
                            color: "green",
                            kind: ItemKind::Wall,
                        });
                    }
                    Some(false) => (),
                    None => {
                        board.push(Item {
                            y: y * 2,
                            x: x * 2 + 1,
                            color: "black",
                            kind: ItemKind::DottedWall,
                        });
                    }
                }
            }
        }
        for y in 0..(height - 1) {
            for x in 0..width {
                match lines.vertical[y][x] {
                    Some(true) => {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2,
                            color: "green",
                            kind: ItemKind::Wall,
                        });
                    }
                    Some(false) => (),
                    None => {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2,
                            color: "black",
                            kind: ItemKind::DottedWall,
                        });
                    }
                }
            }
        }
        for y in 0..(height - 1) {
            for x in 0..(width - 1) {
                match lines_dr[y][x] {
                    Some(true) => {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 1,
                            color: "green",
                            kind: ItemKind::Backslash,
                        });
                    }
                    Some(false) => (),
                    None => {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 1,
                            color: "black",
                            kind: ItemKind::DottedBackslash,
                        });
                    }
                }

                match lines_dl[y][x] {
                    Some(true) => {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 1,
                            color: "green",
                            kind: ItemKind::Slash,
                        });
                    }
                    Some(false) => (),
                    None => {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 1,
                            color: "black",
                            kind: ItemKind::DottedSlash,
                        });
                    }
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "white",
                    kind: ItemKind::FilledCircle,
                });
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::Circle,
                });

                if n >= 0 {
                    board.push(Item {
                        y: y * 2,
                        x: x * 2,
                        color: "black",
                        kind: ItemKind::Num(n),
                    });
                }
            }
        }
    }

    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::board::*;
    use crate::compare_board_and_check_no_solution_case;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board_and_check_no_solution_case!(
            solve("https://pedros.works/paper-puzzle-player?W=5x4&L=(3)0(2)1(2)1x1x1x1(1)1(1)2(3)1(1)1(2)1(4)1(2)1(1)1(1)2x3&G=spokes"),
            Board {
                kind: BoardKind::Empty,
                height: 3,
                width: 4,
                data: vec![
                    Item { y: 0, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 0, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Slash },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Backslash },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Slash },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Slash },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Slash },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Backslash },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Slash },
                    Item { y: 0, x: 2, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 0, x: 2, color: "black", kind: ItemKind::Circle },
                    Item { y: 0, x: 4, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 0, x: 4, color: "black", kind: ItemKind::Circle },
                    Item { y: 0, x: 4, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 0, x: 6, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 0, x: 6, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 0, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 0, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 0, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 2, x: 2, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 4, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 4, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 4, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 6, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 6, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 6, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 8, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 0, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 4, x: 0, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 0, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 4, x: 4, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 4, x: 4, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 4, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 4, x: 6, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 4, x: 6, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 6, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 4, x: 8, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 4, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 0, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 6, x: 0, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 0, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 6, x: 4, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 6, x: 4, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 4, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 6, x: 6, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 6, x: 6, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 6, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 6, x: 8, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 6, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 8, color: "black", kind: ItemKind::Num(1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
