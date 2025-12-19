use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::roma;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = roma::deserialize_problem(url).ok_or("invalid url")?;
    let ans = roma::solve_roma(&borders, &clues).ok_or("no answer")?;
    let height = ans.len();
    let width = ans[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    board.add_borders(&borders, "black");

    let mut add_arrow = |y: usize, x: usize, kind: i32, color: &'static str| match kind {
        roma::CLUE_UP => {
            board.push(Item::cell(y, x, color, ItemKind::ArrowUp));
        }
        roma::CLUE_DOWN => {
            board.push(Item::cell(y, x, color, ItemKind::ArrowDown));
        }
        roma::CLUE_LEFT => {
            board.push(Item::cell(y, x, color, ItemKind::ArrowLeft));
        }
        roma::CLUE_RIGHT => {
            board.push(Item::cell(y, x, color, ItemKind::ArrowRight));
        }
        roma::CLUE_GOAL => {
            board.push(Item::cell(y, x, color, ItemKind::FilledCircle));
        }
        _ => {}
    };

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                add_arrow(y, x, n, "black");
            } else if let Some(n) = ans[y][x] {
                add_arrow(y, x, n, "green");
            }
        }
    }

    Ok(board)
}
