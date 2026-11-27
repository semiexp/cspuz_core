use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::ichimaga;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = ichimaga::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = ichimaga::solve_ichimaga(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Empty, height, width, check_uniqueness(&is_line));

    if let Some(is_line) = &is_line {
        for y in 0..(height - 1) {
            for x in 0..width {
                if let Some(b) = is_line.vertical[y][x] {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2,
                        color: "green",
                        kind: if b {
                            ItemKind::BoldWall
                        } else {
                            ItemKind::Cross
                        },
                    });
                    if !b {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2,
                            color: "black",
                            kind: ItemKind::DottedWall,
                        });
                    }
                } else {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2,
                        color: "black",
                        kind: ItemKind::DottedWall,
                    });
                }
            }
        }
        for y in 0..height {
            for x in 0..(width - 1) {
                if let Some(b) = is_line.horizontal[y][x] {
                    board.push(Item {
                        y: y * 2,
                        x: x * 2 + 1,
                        color: "green",
                        kind: if b {
                            ItemKind::BoldWall
                        } else {
                            ItemKind::Cross
                        },
                    });
                    if !b {
                        board.push(Item {
                            y: y * 2,
                            x: x * 2 + 1,
                            color: "black",
                            kind: ItemKind::DottedWall,
                        });
                    }
                } else {
                    board.push(Item {
                        y: y * 2,
                        x: x * 2 + 1,
                        color: "black",
                        kind: ItemKind::DottedWall,
                    });
                }
            }
        }
    } else {
        for y in 0..(height - 1) {
            for x in 0..width {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::DottedWall,
                });
            }
        }
        for y in 0..height {
            for x in 0..(width - 1) {
                board.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "black",
                    kind: ItemKind::DottedWall,
                });
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = problem[y][x] {
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
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("https://puzz.link/p?ichimaga/6/5/jdel11dgbhb"),
            Board {
                kind: BoardKind::Empty,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 0, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 0, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 0, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 0, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 0, x: 8, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 4, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 8, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 4, x: 10, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 10, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 6, x: 0, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 0, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 6, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 8, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 8, x: 6, color: "black", kind: ItemKind::Circle },
                    Item { y: 8, x: 6, color: "black", kind: ItemKind::Num(1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
