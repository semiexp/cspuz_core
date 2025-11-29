use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::exercise;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let has_block = exercise::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = exercise::solve_exercise(&has_block).ok_or("no answer")?;

    let height = has_block.len();
    let width = has_block[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            if has_block[y][x] {
                board.push(Item::cell(y, x, "#cccccc", ItemKind::Block));
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
