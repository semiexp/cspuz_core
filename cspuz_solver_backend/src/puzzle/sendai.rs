use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs::graph;
use cspuz_rs_puzzles::puzzles::sendai;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = sendai::deserialize_problem(url).ok_or("invalid url")?;
    let ans = sendai::solve_sendai(&borders, &clues).ok_or("no answer")?;
    let height = ans.horizontal.len() + 1;
    let width = ans.horizontal[0].len();
    let mut board = Board::new(BoardKind::OuterGrid, height, width, is_unique(&ans));

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if y < height - 1 {
                let mut need_default_edge = true;
                if let Some(b) = ans.horizontal[y][x] {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: if borders.horizontal[y][x] {
                            "black"
                        } else {
                            "green"
                        },
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
            if x < width - 1 {
                let mut need_default_edge = true;
                if let Some(b) = ans.vertical[y][x] {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: if borders.vertical[y][x] {
                            "black"
                        } else {
                            "green"
                        },
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

    let rooms = graph::borders_to_rooms(&borders);
    assert_eq!(rooms.len(), clues.len());
    for i in 0..rooms.len() {
        if let Some(n) = clues[i] {
            let (y, x) = rooms[i][0];
            if n >= 0 {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            }
        }
    }

    Ok(board)
}
