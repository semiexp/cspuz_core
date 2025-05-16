use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::archipelago;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let clues = archipelago::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = archipelago::solve_archipelago(&clues).ok_or("no answer")?;

    let height = is_black.len();
    let width = is_black[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_black));

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Fill));
                if n >= 0 {
                    board.push(Item::cell(y, x, "white", ItemKind::Num(n)));
                }
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
