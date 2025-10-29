use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::geradeweg;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = geradeweg::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = geradeweg::solve_geradeweg(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                }
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
