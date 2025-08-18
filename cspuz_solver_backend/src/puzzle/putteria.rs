use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::putteria;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = putteria::deserialize_problem(url).ok_or("invalid url")?;
    let ans = putteria::solve_putteria(&borders, &clues).ok_or("no answer")?;

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "black",
                    if n == -2 { ItemKind::Dot } else { ItemKind::Num(n)},
                ));
            } else if let Some(n) = ans[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if n == -2 { ItemKind::Dot } else { ItemKind::Num(n)},
                ));     
            }
        }
    }

    Ok(board)
}