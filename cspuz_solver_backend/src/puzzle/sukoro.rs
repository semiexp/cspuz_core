use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::sukoro;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = sukoro::deserialize_problem(url).ok_or("invalid url")?;
    let ans = sukoro::solve_sukoro(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = problem[y][x] {
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