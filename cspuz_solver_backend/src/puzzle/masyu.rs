use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::masyu;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use masyu::MasyuClue;

    let problem = masyu::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = masyu::solve_masyu(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                MasyuClue::None => (),
                MasyuClue::White => board.push(Item::cell(y, x, "black", ItemKind::Circle)),
                MasyuClue::Black => board.push(Item::cell(y, x, "black", ItemKind::FilledCircle)),
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
