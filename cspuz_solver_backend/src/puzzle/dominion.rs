use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::dominion;

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = dominion::deserialize_problem(url).ok_or("invalid url")?;
    let ans = dominion::solve_dominion(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if 1 <= clue && clue <= 26 {
                    let p = (clue - 1) as usize;
                    board.push(Item::cell(y, x, "black", ItemKind::Text(&ALPHA[p..=p])));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue - 26)));
                }
            } else if let Some(a) = ans[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if a { ItemKind::Block } else { ItemKind::Dot },
                ));
            }
        }
    }

    Ok(board)
}
