use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::windows;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = windows::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = windows::solve_windows(&borders, &clues).ok_or("no answer")?;
    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_black));

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if clues[y][x] == Some(0) {
                board.push(Item::cell(y, x, "black", ItemKind::Dot));
            } else if clues[y][x] == Some(1) {
                board.push(Item::cell(y, x, "black", ItemKind::Block));
            } else if let Some(b) = is_black[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if b { ItemKind::Block } else { ItemKind::Dot },
                ));
            }
        }
    }

    Ok(board)
}
