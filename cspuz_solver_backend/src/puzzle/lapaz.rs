use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::lapaz;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = lapaz::deserialize_problem(url).ok_or("invalid url")?;
    let ans = lapaz::solve_lapaz(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        check_uniqueness(&ans),
    );
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if clue >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some((is_black, _)) = &ans {
                if is_black[y][x] == Some(true) {
                    board.push(Item::cell(y, x, "green", ItemKind::Block));
                } else if is_black[y][x] == Some(false) {
                    board.push(Item::cell(y, x, "green", ItemKind::Dot));
                }
            }
        }
    }
    if let Some((_, is_connected)) = &ans {
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: if is_connected.vertical[y][x].is_some() {
                            "green"
                        } else {
                            "#cccccc"
                        },
                        kind: match is_connected.vertical[y][x] {
                            Some(true) => ItemKind::Cross,
                            Some(false) => ItemKind::BoldWall,
                            None => ItemKind::Wall,
                        },
                    });
                    if is_connected.vertical[y][x] != Some(false) {
                        board.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color: "#cccccc",
                            kind: ItemKind::Wall,
                        });
                    }
                }
                if x < width - 1 {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: if is_connected.horizontal[y][x].is_some() {
                            "green"
                        } else {
                            "#cccccc"
                        },
                        kind: match is_connected.horizontal[y][x] {
                            Some(true) => ItemKind::Cross,
                            Some(false) => ItemKind::BoldWall,
                            None => ItemKind::Wall,
                        },
                    });
                    if is_connected.horizontal[y][x] != Some(false) {
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
    } else {
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "#cccccc",
                        kind: ItemKind::Wall,
                    });
                }
                if x < width - 1 {
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
            solve("https://pzprxs.vercel.app/p?lapaz/5/4/2j1m.k1"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 4,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Text("?") },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
