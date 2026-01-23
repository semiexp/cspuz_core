use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::fillomino;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = fillomino::deserialize_problem(url).ok_or("invalid url")?;
    let ans = fillomino::solve_fillomino(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        ans.as_ref().map_or(Uniqueness::NoAnswer, |(num, border)| {
            is_unique(&(num, border))
        }),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some((num, _)) = &ans {
                if let Some(n) = num[y][x] {
                    board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if y < height - 1 {
                let mut need_default_edge = true;
                if let Some((_, border)) = &ans {
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
            if x < width - 1 {
                let mut need_default_edge = true;
                if let Some((_, border)) = &ans {
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
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("https://puzz.link/p?fillomino/5/5/g1k34g2h5h4n"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
