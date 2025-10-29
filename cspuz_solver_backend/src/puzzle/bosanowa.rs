use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::bosanowa;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (circles, clues) = bosanowa::deserialize_problem(url).ok_or("invalid url")?;
    let ans = bosanowa::solve_bosanowa(&circles, &clues)?.ok_or("no answer")?;

    let height = circles.len();
    let width = circles[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = clues[y * width + x] {
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some(n) = ans[y][x] {
                if n > 0 {
                    board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
                }
            } else if circles[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
            }
        }
    }

    Ok(board)
}
