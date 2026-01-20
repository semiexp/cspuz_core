use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::tricklayer;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = tricklayer::deserialize_problem(url).ok_or("invalid url")?;
    let ans = tricklayer::solve_tricklayer(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::OuterGrid, height, width, is_unique(&ans));
    for y in 0..height {
        for x in 0..width {
            if problem[y][x] {
                board.push(Item::cell(y, x, "#cccccc", ItemKind::Fill));
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            if y < height - 1 && !problem[y][x] && !problem[y + 1][x] {
                let mut need_default_edge = true;
                if let Some(b) = ans.horizontal[y][x] {
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
            if x < width - 1 && !problem[y][x] && !problem[y][x + 1] {
                let mut need_default_edge = true;
                if let Some(b) = ans.vertical[y][x] {
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
            solve("https://pedros.works/paper-puzzle-player?W=5x4&L=x1x2x8x7x1&G=tricklayer"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 4,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
