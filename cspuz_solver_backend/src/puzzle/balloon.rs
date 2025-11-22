use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::balloon;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (color, num) = balloon::deserialize_problem(url).ok_or("invalid url")?;
    let has_line = balloon::solve_balloon(&color, &num).ok_or("no answer")?;
    let height = num.len();
    let width = num[0].len();
    let mut board = Board::new(BoardKind::OuterGrid, height, width, is_unique(&has_line));

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

    Ok(board)
}
