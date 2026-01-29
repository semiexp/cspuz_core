use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::school_trip;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = school_trip::deserialize_problem(url).ok_or("invalid url")?;
    let ans = school_trip::solve_school_trip(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        ans.as_ref().map_or(
            Uniqueness::NoAnswer,
            |(is_black, is_pillow, is_connected)| is_unique(&(is_black, is_pillow, is_connected)),
        ),
    );
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
                if clue >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                }
            } else if let Some((is_black, is_pillow, _)) = &ans {
                if is_black[y][x] == Some(true) {
                    board.push(Item::cell(y, x, "green", ItemKind::Block));
                } else if is_pillow[y][x] == Some(true) {
                    board.push(Item::cell(y, x, "green", ItemKind::Square));
                } else if is_black[y][x] == Some(false) {
                    board.push(Item::cell(y, x, "green", ItemKind::Dot));
                }
            }
        }
    }
    if let Some((is_black, _, is_connected)) = &ans {
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 {
                    let mut need_default_edge = true;
                    if (is_black[y][x] == Some(false) && problem[y][x].is_none())
                        || (is_black[y + 1][x] == Some(false) && problem[y + 1][x].is_none())
                    {
                        // If a cell is not black in the solution then either it is a number in the problem, or a futon. This checks which cells are futons
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
                        if is_connected.vertical[y][x] != Some(true) {
                            need_default_edge = false;
                        }
                    };

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
                    if (is_black[y][x] == Some(false) && problem[y][x].is_none())
                        || (is_black[y][x + 1] == Some(false) && problem[y][x + 1].is_none())
                    {
                        // If a cell is not black in the solution then either it is a number in the problem, or a futon. This checks which cells are futons
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
                        if is_connected.horizontal[y][x] != Some(true) {
                            need_default_edge = false;
                        }
                    };

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
            solve("https://puzz.link/p?shugaku/6/5/272d1d07090"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Square },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Square },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Square },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Square },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Square },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Square },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 11, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
