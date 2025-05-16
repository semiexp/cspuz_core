use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::forestwalk;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (forest, num) = forestwalk::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = forestwalk::solve_forestwalk(&forest, &num).ok_or("no answer")?;

    let height = forest.len();
    let width = forest[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            if forest[y][x] {
                board.push(Item::cell(y, x, "#e0ffe0", ItemKind::Fill));
            }
            if let Some(n) = num[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
