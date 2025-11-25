use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::energywalk;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (forest, num) = energywalk::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = energywalk::solve_energywalk(&forest, &num).ok_or("no answer")?;
    let height = forest.len();
    let width = forest[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            if forest[y][x] {
                board.push(Item::cell(y, x, "#f9f9d0", ItemKind::Fill));
            }
            if let Some(n) = num[y][x] {
                if n >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
