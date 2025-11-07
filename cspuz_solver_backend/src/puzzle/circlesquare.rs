use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::circlesquare::{self, CircleSquareClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = circlesquare::deserialize_problem(url).ok_or("invalid url")?;
    let ans = circlesquare::solve_circlesquare(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));
    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                CircleSquareClue::None => {
                    if let Some(a) = ans[y][x] {
                        board.push(Item::cell(
                            y,
                            x,
                            "green",
                            if a { ItemKind::Block } else { ItemKind::Dot },
                        ));
                    }
                }
                CircleSquareClue::White => board.push(Item::cell(y, x, "black", ItemKind::Dot)),
                CircleSquareClue::Black => board.push(Item::cell(y, x, "black", ItemKind::Fill)),
            }
        }
    }

    Ok(board)
}
