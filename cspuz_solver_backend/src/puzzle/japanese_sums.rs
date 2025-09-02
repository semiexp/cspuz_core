use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::japanese_sums;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (k, (vertical, horizontal)) = japanese_sums::deserialize_problem(url).ok_or("invalid url")?;
    let num = japanese_sums::solve_japanese_sums(k, &vertical, &horizontal).ok_or("no answer")?;

    let height = num.len();
    let width = num[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&num));

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = num[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if n == 0 { ItemKind::Block } else { ItemKind::Num(n) },
                ));
            }
        }
    }

    Ok(board)
}
