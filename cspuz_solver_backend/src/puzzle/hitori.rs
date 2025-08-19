use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::hitori;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = hitori::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = hitori::solve_hitori(&problem).ok_or("no answer")?;

    let height = is_black.len();
    let width = is_black[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_black));

    for y in 0..height {
        for x in 0..width {
            match (problem[y][x], is_black[y][x]) {
                (0, None) => (),
                (0, Some(false)) => {
                    board.push(Item::cell(y, x, "green", ItemKind::Dot));
                }
                (0, Some(true)) => {
                    board.push(Item::cell(y, x, "green", ItemKind::Fill));
                }
                (n, None) => {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                }
                (n, Some(false)) => {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                    board.push(Item::cell(y, x, "green", ItemKind::Dot));
                }
                (n, Some(true)) => {
                    board.push(Item::cell(y, x, "green", ItemKind::Fill));
                    board.push(Item::cell(y, x, "white", ItemKind::Num(n)));
                }
            }
        }
    }

    Ok(board)
}
