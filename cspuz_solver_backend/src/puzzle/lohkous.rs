use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::lohkous;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = lohkous::deserialize_problem(url).ok_or("invalid url")?;
    let ans = lohkous::solve_lohkous(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::OuterGrid, height, width, is_unique(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = &problem[y][x] {
                if clue.len() <= 4 {
                    let mut c = [-1, -1, -1, -1];
                    for i in 0..clue.len() {
                        c[i] = if clue[i] == -1 { -2 } else { clue[i] };
                    }
                    board.push(Item::cell(y, x, "black", ItemKind::TapaClue(c)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("...")));
                }
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            if y < height - 1 {
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
            if x < width - 1 {
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
            solve("https://puzz.link/p?lohkous/6/6/12k2a23b2k13d13a10b14d"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::TapaClue([1, 2, -1, -1]) },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::TapaClue([2, -1, -1, -1]) },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::TapaClue([2, 3, -1, -1]) },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::TapaClue([2, -1, -1, -1]) },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::TapaClue([1, 3, -1, -1]) },
                    Item { y: 9, x: 11, color: "black", kind: ItemKind::TapaClue([1, 3, -1, -1]) },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::TapaClue([1, -2, -1, -1]) },
                    Item { y: 11, x: 5, color: "black", kind: ItemKind::TapaClue([1, 4, -1, -1]) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
