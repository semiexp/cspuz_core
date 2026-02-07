use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs::graph::InnerGridEdges;
use cspuz_rs_puzzles::puzzles::polyominous;

const PENTOMINO_NAMES: [&str; 12] = ["F", "I", "L", "N", "P", "T", "U", "V", "W", "X", "Y", "Z"];

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues, default_borders) =
        polyominous::deserialize_pentominous_problem(url).ok_or("invalid url")?;
    let ans = polyominous::solve_pentominous(&clues, &default_borders);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "black",
                    if n >= 0 {
                        ItemKind::Text(PENTOMINO_NAMES[n as usize])
                    } else if n == -1 {
                        ItemKind::Text("?")
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
            if y < height - 1 && clues[y][x] != Some(-2) && clues[y + 1][x] != Some(-2) {
                let mut need_default_edge = true;
                if default_borders.horizontal[y][x] {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "black",
                        kind: ItemKind::BoldWall,
                    });
                    need_default_edge = false;
                } else if let Some(border) = &ans {
                    if let Some(b) = border.horizontal[y][x] {
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
            if x < width - 1 && clues[y][x] != Some(-2) && clues[y][x + 1] != Some(-2) {
                let mut need_default_edge = true;
                if default_borders.vertical[y][x] {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "black",
                        kind: ItemKind::BoldWall,
                    });
                    need_default_edge = false;
                } else if let Some(border) = &ans {
                    if let Some(b) = border.vertical[y][x] {
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
            solve("https://puzz.link/p?pentominous/5/5/72zi"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Text("V") },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Text("L") },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
