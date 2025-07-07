use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs::graph::InnerGridEdges;
use cspuz_rs_puzzles::puzzles::polyominous;

const TETROMINO_NAMES: [&'static str; 5] =
    ["I", "L", "O", "S", "T"];


pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues, default_borders) =
        polyominous::deserialize_tetrominous_problem(url).ok_or("invalid url")?;
    let border = polyominous::solve_tetrominous(&clues, &default_borders).ok_or("no answer")?;

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::OuterGrid, height, width, is_unique(&border));

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "black",
                    if n >= 0 {
                        ItemKind::Text(TETROMINO_NAMES[n as usize])
                    } else {
                        ItemKind::Fill
                    },
                ));
            }
        }
    }

    let default_borders = default_borders.unwrap_or(InnerGridEdges {
        horizontal: vec![vec![false; width]; height - 1],
        vertical: vec![vec![false; width - 1]; height],
    });
    for y in 0..height {
        for x in 0..width {
            if y < height - 1 && clues[y][x] != Some(-1) && clues[y + 1][x] != Some(-1) {
                let mut need_default_edge = true;
                if default_borders.horizontal[y][x] {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "black",
                        kind: ItemKind::BoldWall,
                    });
                    need_default_edge = false;
                } else if let Some(b) = border.horizontal[y][x] {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "green",
                        kind: if b {
                            ItemKind::BoldWall
                        } else {
                            ItemKind::Cross
                        },
                    });
                    if b {
                        need_default_edge = false;
                    }
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
            if x < width - 1 && clues[y][x] != Some(-1) && clues[y][x + 1] != Some(-1) {
                let mut need_default_edge = true;
                if default_borders.vertical[y][x] {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "black",
                        kind: ItemKind::BoldWall,
                    });
                    need_default_edge = false;
                } else if let Some(b) = border.vertical[y][x] {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "green",
                        kind: if b {
                            ItemKind::BoldWall
                        } else {
                            ItemKind::Cross
                        },
                    });
                    if b {
                        need_default_edge = false;
                    }
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