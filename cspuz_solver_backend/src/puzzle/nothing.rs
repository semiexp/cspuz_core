use crate::board::{Board, BoardKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::nothing;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let borders = nothing::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = nothing::solve_all_or_nothing(&borders).ok_or("no answer")?;

    let height = borders.vertical.len();
    let width = borders.vertical[0].len() + 1;
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));
    board.add_borders(&borders, "black");

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
