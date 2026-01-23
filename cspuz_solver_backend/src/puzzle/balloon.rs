use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::balloon;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (color, num) = balloon::deserialize_problem(url).ok_or("invalid url")?;
    let has_line = balloon::solve_balloon(&color, &num);
    let height = num.len();
    let width = num[0].len();
    let mut board = Board::new(
        if has_line.is_some() {
            BoardKind::OuterGrid
        } else {
            BoardKind::Grid
        },
        height,
        width,
        has_line
            .as_ref()
            .map_or(Uniqueness::NoAnswer, |h| is_unique(h)),
    );

    for y in 0..height {
        for x in 0..width {
            if color[y][x] == 1 {
                board.push(Item::cell(y, x, "#eeeeee", ItemKind::Fill));
            }
            if let Some(n) = num[y][x] {
                if n == -1 {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                }
            }
        }
    }

    if let Some(has_line) = &has_line {
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 {
                    let mut need_default_edge = true;

                    if (color[y][x] ^ color[y + 1][x]) == 1 {
                        board.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color: "green",
                            kind: ItemKind::BoldWall,
                        });
                        need_default_edge = false;
                    }

                    match (color[y][x], color[y + 1][x], has_line.vertical[y][x]) {
                        (1, 1, Some(true)) | (0, 0, Some(false)) => {
                            board.push(Item {
                                y: y * 2 + 2,
                                x: x * 2 + 1,
                                color: "green",
                                kind: ItemKind::Cross,
                            });
                        }
                        (_, _, Some(true)) => {
                            board.push(Item {
                                y: y * 2 + 2,
                                x: x * 2 + 1,
                                color: "green",
                                kind: ItemKind::Line,
                            });
                        }
                        (1, 1, Some(false)) => {
                            board.push(Item {
                                y: y * 2 + 2,
                                x: x * 2 + 1,
                                color: "green",
                                kind: ItemKind::BoldWall,
                            });
                            need_default_edge = false;
                        }
                        _ => (),
                    }

                    if need_default_edge {
                        board.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color: "#cccccc",
                            kind: ItemKind::Wall,
                        });
                    }
                }
                if x < width - 1 {
                    let mut need_default_edge = true;

                    if (color[y][x] ^ color[y][x + 1]) == 1 {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 2,
                            color: "green",
                            kind: ItemKind::BoldWall,
                        });
                        need_default_edge = false;
                    }

                    match (color[y][x], color[y][x + 1], has_line.horizontal[y][x]) {
                        (1, 1, Some(true)) | (0, 0, Some(false)) => {
                            board.push(Item {
                                y: y * 2 + 1,
                                x: x * 2 + 2,
                                color: "green",
                                kind: ItemKind::Cross,
                            });
                        }
                        (_, _, Some(true)) => {
                            board.push(Item {
                                y: y * 2 + 1,
                                x: x * 2 + 2,
                                color: "green",
                                kind: ItemKind::Line,
                            });
                        }
                        (1, 1, Some(false)) => {
                            board.push(Item {
                                y: y * 2 + 1,
                                x: x * 2 + 2,
                                color: "green",
                                kind: ItemKind::BoldWall,
                            });
                            need_default_edge = false;
                        }
                        _ => (),
                    }

                    if need_default_edge {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 2,
                            color: "#cccccc",
                            kind: ItemKind::Wall,
                        });
                    }
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
            solve("https://pzprxs.vercel.app/p?balloon/a/6/5/0e7ivgj.k6n2m14g"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Text("?") },
                    Item { y: 3, x: 1, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Num(6) },
                    Item { y: 5, x: 1, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 7, x: 1, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 5, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 7, x: 9, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 7, x: 11, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 9, x: 1, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 9, x: 3, color: "#eeeeee", kind: ItemKind::Fill },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
