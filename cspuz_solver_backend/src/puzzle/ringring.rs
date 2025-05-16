use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::ringring;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = ringring::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = ringring::solve_ringring(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            if problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Fill));
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", Some(&problem));

    Ok(board)
}
