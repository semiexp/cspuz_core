use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::tilecity;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = tilecity::deserialize_problem(url).ok_or("invalid url")?;
    let ans = tilecity::solve_tilecity(&borders, &clues).ok_or("no answer")?;
    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    for y in 0..height {
        for x in 0..width {
            if let Some(b) = ans[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if b { ItemKind::Fill } else { ItemKind::Dot },
                ));
            }
        }
    }
    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            }
        }
    }

    Ok(board)
}
