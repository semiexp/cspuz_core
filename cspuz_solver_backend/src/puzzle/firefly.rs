use crate::board::{Board, BoardKind, FireflyDir, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs::items::Arrow;
use cspuz_rs_puzzles::puzzles::firefly;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = firefly::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = firefly::solve_firefly(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Empty,
        height,
        width,
        is_line.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );

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
    }

    for y in 0..height {
        for x in 0..width {
            if let Some((dir, n)) = problem[y][x] {
                let dir = match dir {
                    Arrow::Unspecified => panic!(),
                    Arrow::Up => FireflyDir::Up,
                    Arrow::Down => FireflyDir::Down,
                    Arrow::Left => FireflyDir::Left,
                    Arrow::Right => FireflyDir::Right,
                };
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::Firefly(dir, n),
                });
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
            solve("https://puzz.link/p?firefly/5/6/f1.a43b4.a42b2.a32g3.c"),
            Board {
                kind: BoardKind::Empty,
                height: 6,
                width: 5,
                data: vec![
                    Item { y: 1, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 0, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 0, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 0, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 0, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 9, x: 0, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::Firefly(FireflyDir::Up, -1) },
                    Item { y: 2, x: 6, color: "black", kind: ItemKind::Firefly(FireflyDir::Right, 3) },
                    Item { y: 4, x: 2, color: "black", kind: ItemKind::Firefly(FireflyDir::Right, -1) },
                    Item { y: 4, x: 6, color: "black", kind: ItemKind::Firefly(FireflyDir::Right, 2) },
                    Item { y: 6, x: 2, color: "black", kind: ItemKind::Firefly(FireflyDir::Down, -1) },
                    Item { y: 6, x: 6, color: "black", kind: ItemKind::Firefly(FireflyDir::Left, 2) },
                    Item { y: 10, x: 2, color: "black", kind: ItemKind::Firefly(FireflyDir::Left, -1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
