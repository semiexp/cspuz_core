use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::battleship::{self, BattleshipClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let ((vertical, horizontal, grid), pieces) =
        battleship::deserialize_problem(url).ok_or("invalid url")?;
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

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::board::*;
    use crate::compare_board_and_check_no_solution_case;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board_and_check_no_solution_case!(
            solve("https://puzz.link/p?battleship/6/6/g12h2g30g3gk0r3w//c"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Block},
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
