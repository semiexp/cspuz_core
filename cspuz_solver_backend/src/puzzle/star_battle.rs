use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::star_battle;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (star_amount, borders) = star_battle::deserialize_problem(url).ok_or("invalid url")?;
    let ans = star_battle::solve_star_battle(star_amount, &borders).ok_or("no answer")?;
    
    let height = ans.len();
    let width = ans[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if let Some(b) = ans[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if b { ItemKind::Star } else { ItemKind::Dot },
                ));
            }
        }
    }

    Ok(board)
}