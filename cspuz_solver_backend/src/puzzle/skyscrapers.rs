use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs_puzzles::puzzles::skyscrapers;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues_up, clues_down, clues_left, clues_right, cells) =
        skyscrapers::deserialize_problem(url).ok_or("invalid url")?;
    let ans: Option<Vec<Vec<Option<i32>>>> =
        skyscrapers::solve_skyscrapers(&clues_up, &clues_down, &clues_left, &clues_right, &cells);

    let height = clues_left.len();
    let width = clues_up.len();

    let mut is_unique = Uniqueness::Unique;
    if let Some(answer) = &ans {
        for y in 0..height {
            for x in 0..width {
                if answer[y][x].is_none() || answer[y][x] == Some(-1) {
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
            board.push(Item::cell(y + 1, 0, "black", ItemKind::Num(n)));
        }
        if let Some(n) = clues_right[y] {
            board.push(Item::cell(y + 1, width + 1, "black", ItemKind::Num(n)));
        }
    }
    for x in 0..width {
        if let Some(n) = clues_up[x] {
            board.push(Item::cell(0, x + 1, "black", ItemKind::Num(n)));
        }
        if let Some(n) = clues_down[x] {
            board.push(Item::cell(height + 1, x + 1, "black", ItemKind::Num(n)));
        }
    }

    board.add_grid(1, 1, height, width);

    for y in 0..height {
        for x in 0..width {
            if let Some(ref clues) = cells {
                if let Some(n) = clues[y][x] {
                    if n >= 0 {
                        board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Num(n)));
                    } else {
                        board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Text("?")));
                    }
                } else if let Some(ans) = &ans {
                    if let Some(n) = ans[y][x] {
                        if n > 0 {
                            board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Num(n)));
                        } else {
                            board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Dot));
                        }
                    }
                }
            } else if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    if n > 0 {
                        board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Num(n)));
                    } else {
                        board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Dot));
                    }
                }
            }
        }
    }

    Ok(board)
}
