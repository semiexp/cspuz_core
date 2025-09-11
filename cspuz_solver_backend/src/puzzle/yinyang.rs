use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::yinyang;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use yinyang::YinYangClue;

    let problem = yinyang::deserialize_problem(url).ok_or("invalid url")?;
    let ans = yinyang::solve_yinyang(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                YinYangClue::None => (),
                YinYangClue::White => board.push(Item::cell(y, x, "black", ItemKind::Circle)),
                YinYangClue::Black => board.push(Item::cell(y, x, "black", ItemKind::FilledCircle)),
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(b) = ans[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if b { ItemKind::FilledCircle } else { ItemKind::Circle },
                ));
            }
        }
    }

    Ok(board)
}
