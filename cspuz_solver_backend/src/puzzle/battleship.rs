use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::battleship::{self, BattleshipClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let ((vertical, horizontal), grid, pieces) = battleship::deserialize_problem(url).ok_or("invalid url")?;
    let ans = battleship::solve_battleship(&vertical, &horizontal, &grid, &pieces);

    let height = grid.len();
    let width = grid[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));
    if let Some(ans) = &ans {
        for y in 0..height {
            for x in 0..width {
                match grid[y][x] {
                    BattleshipClue::None => {
                        if let Some(a) = ans[y][x] {
                            board.push(Item::cell(
                                y,
                                x,
                                "green",
                                if a { ItemKind::Block } else { ItemKind::Dot },
                            ));
                        }
                    }
                    BattleshipClue::Water => board.push(Item::cell(y, x, "black", ItemKind::Dot)),
                    _ => board.push(Item::cell(y, x, "black", ItemKind::Fill)),
                }
            }
        }
    } else {
        for y in 0..height {
            for x in 0..width {
                match grid[y][x] {
                    BattleshipClue::None => {}
                    BattleshipClue::Water => board.push(Item::cell(y, x, "black", ItemKind::Dot)),
                    _ => board.push(Item::cell(y, x, "black", ItemKind::Fill)),
                }
            }
        }
    }

    Ok(board)
}

