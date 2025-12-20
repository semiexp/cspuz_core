use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::pyramid_climbers;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let clues = pyramid_climbers::deserialize_problem(url).ok_or("invalid url")?;
    let ans = pyramid_climbers::solve_pyramid_climbers(&clues).ok_or("no answer")?;
    let size = ans.len();

    let mut board = Board::new(BoardKind::Empty, size, size * 2, is_unique(&ans.concat()));

    // Clues
    for y in 0..size {
        for x in 0..=y {
            board.push(Item {
                y: 2 * y + 1,
                x: (size - y - 1 + 2 * x + 1) * 2,
                color: "black",
                kind: ItemKind::TextString(clues[y][x].clone()),
            });
        }
    }

    // Borders
    for y in 0..=size {
        let start = if y == size { 0 } else { size - y - 1 };
        let end = if y == size { size * 2 } else { size + y + 1 };

        for x in start..end {
            board.push(Item {
                y: y * 2,
                x: x * 2 + 1,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }
    for y in 0..size {
        for x in 0..=(y + 1) {
            board.push(Item {
                y: y * 2 + 1,
                x: (size - y - 1 + 2 * x) * 2,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }

    // Answers
    for y in 0..(size - 1) {
        for x in 0..=y {
            match ans[y][x * 2] {
                Some(true) => {
                    board.push(Item {
                        y: 2 * y + 1,
                        x: (size - y - 1 + 2 * x) * 2 + 2,
                        color: "green",
                        kind: ItemKind::LineTo(
                            (2 * y + 3) as i32,
                            ((size - y - 1 + 2 * x) * 2) as i32,
                        ),
                    });
                }
                Some(false) => {
                    board.push(Item {
                        y: 2 * y + 2,
                        x: (size - y - 1 + 2 * x) * 2 + 1,
                        color: "green",
                        kind: ItemKind::Cross,
                    });
                }
                None => (),
            }
            match ans[y][x * 2 + 1] {
                Some(true) => {
                    board.push(Item {
                        y: 2 * y + 1,
                        x: (size - y - 1 + 2 * x) * 2 + 2,
                        color: "green",
                        kind: ItemKind::LineTo(
                            (2 * y + 3) as i32,
                            ((size - y - 1 + 2 * x + 2) * 2) as i32,
                        ),
                    });
                }
                Some(false) => {
                    board.push(Item {
                        y: 2 * y + 2,
                        x: (size - y - 1 + 2 * x + 1) * 2 + 1,
                        color: "green",
                        kind: ItemKind::Cross,
                    });
                }
                None => (),
            }
        }
    }
    Ok(board)
}
