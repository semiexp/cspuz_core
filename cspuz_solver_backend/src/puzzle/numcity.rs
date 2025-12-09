use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::numcity;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues, borders) = numcity::deserialize_problem(url).ok_or("invalid url")?;
    let ans = numcity::solve_numcity(&borders, &clues).ok_or("no answer")?;
    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some(n) = ans[y][x] {
                board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
            }
        }
    }

    Ok(board)
}
