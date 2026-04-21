use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs_puzzles::puzzles::easyasabc;

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghiklmnopqrstuvwxyz";

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (range, (clues_up, clues_down, clues_left, clues_right), cells) =
        easyasabc::deserialize_problem(url).ok_or("invalid url")?;
    let ans: Option<Vec<Vec<Option<i32>>>> = easyasabc::solve_easy_as_abc(
        range,
        &clues_up,
        &clues_down,
        &clues_left,
        &clues_right,
        &cells,
    );

    let height = clues_left.len();
    let width = clues_up.len();

    let mut is_unique = Uniqueness::Unique;
    if let Some(answer) = &ans {
        for y in 0..height {
            for x in 0..width {
                if answer[y][x].is_none() || answer[y][x] == None {
                    is_unique = Uniqueness::NonUnique;
                }
            }
        }
    } else {
        is_unique = Uniqueness::NoAnswer;
    }
    let mut board = Board::new(BoardKind::Empty, height + 2, width + 2, is_unique);

    for y in 0..height {
        if let Some(n) = clues_left[y] {
            board.push(Item::cell(
                y + 1,
                0,
                "black",
                ItemKind::Text(&ALPHA[(n - 1) as usize..=(n - 1) as usize]),
            ));
        }
        if let Some(n) = clues_right[y] {
            board.push(Item::cell(
                y + 1,
                width + 1,
                "black",
                ItemKind::Text(&ALPHA[(n - 1) as usize..=(n - 1) as usize]),
            ));
        }
    }
    for x in 0..width {
        if let Some(n) = clues_up[x] {
            board.push(Item::cell(
                0,
                x + 1,
                "black",
                ItemKind::Text(&ALPHA[(n - 1) as usize..=(n - 1) as usize]),
            ));
        }
        if let Some(n) = clues_down[x] {
            board.push(Item::cell(
                height + 1,
                x + 1,
                "black",
                ItemKind::Text(&ALPHA[(n - 1) as usize..=(n - 1) as usize]),
            ));
        }
    }

    board.add_grid(1, 1, height, width);

    for y in 0..height {
        for x in 0..width {
            if let Some(clues) = &cells {
                if let Some(n) = clues[y][x] {
                    if n >= 0 {
                        board.push(Item::cell(
                            y + 1,
                            x + 1,
                            "black",
                            ItemKind::Text(&ALPHA[(n - 1) as usize..=(n - 1) as usize]),
                        ));
                    } else {
                        board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Text("?")));
                    }
                } else if let Some(ans) = &ans {
                    if let Some(n) = ans[y][x] {
                        if n > 0 {
                            board.push(Item::cell(
                                y + 1,
                                x + 1,
                                "green",
                                ItemKind::Text(&ALPHA[(n - 1) as usize..=(n - 1) as usize]),
                            ));
                        } else {
                            board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Dot));
                        }
                    }
                }
            } else if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    if n > 0 {
                        board.push(Item::cell(
                            y + 1,
                            x + 1,
                            "green",
                            ItemKind::Text(&ALPHA[(n - 1) as usize..=(n - 1) as usize]),
                        ));
                    } else {
                        board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Dot));
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
    use crate::compare_board_and_check_no_solution_case;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board_and_check_no_solution_case!(
            solve("https://puzz.link/p?easyasabc/3/3/2/12h1h1i1k2g1g"),
            Board {
                kind: BoardKind::Empty,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Text("B") },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Text("B") },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Text("A") },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Text("A") },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Text("B") },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Text("B") },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot }
                    ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
