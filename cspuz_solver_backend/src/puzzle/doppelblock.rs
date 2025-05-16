use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs_puzzles::puzzles::doppelblock;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues_up, clues_left, cells) =
        doppelblock::deserialize_problem(url).ok_or("invalid url")?;
    let answer: Vec<Vec<Option<i32>>> =
        doppelblock::solve_doppelblock(&clues_up, &clues_left, &cells).ok_or("no answer")?;

    let height = clues_left.len();
    let width = clues_up.len();

    let mut is_unique = Uniqueness::Unique;
    for y in 0..height {
        for x in 0..width {
            if answer[y][x].is_none() || answer[y][x] == Some(-1) {
                is_unique = Uniqueness::NonUnique;
            }
        }
    }
    let mut board = Board::new(BoardKind::Empty, height + 1, width + 1, is_unique);

    for y in 0..height {
        if let Some(n) = clues_left[y] {
            board.push(Item::cell(y + 1, 0, "black", ItemKind::Num(n)));
        }
    }
    for x in 0..width {
        if let Some(n) = clues_up[x] {
            board.push(Item::cell(0, x + 1, "black", ItemKind::Num(n)));
        }
    }

    for y in 0..=height {
        for x in 0..width {
            board.push(Item {
                y: y * 2 + 2,
                x: x * 2 + 3,
                color: "black",
                kind: if y == 0 || y == height {
                    ItemKind::BoldWall
                } else {
                    ItemKind::Wall
                },
            })
        }
    }
    for y in 0..height {
        for x in 0..=width {
            board.push(Item {
                y: y * 2 + 3,
                x: x * 2 + 2,
                color: "black",
                kind: if x == 0 || x == width {
                    ItemKind::BoldWall
                } else {
                    ItemKind::Wall
                },
            })
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(cells) = &cells {
                if let Some(n) = cells[y][x] {
                    board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Num(n)));
                    continue;
                }
            }

            if let Some(n) = answer[y][x] {
                if n == 0 {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Block));
                } else if n == -1 {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Circle));
                } else {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Num(n)));
                }
            }
        }
    }

    Ok(board)
}
