use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::battleship::{self, BattleshipClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let ((vertical, horizontal, grid), pieces) =
        battleship::deserialize_problem(url).ok_or("invalid url")?;
    let ans = battleship::solve_battleship(&vertical, &horizontal, &grid, &pieces);

    let height = grid.len();
    let width = grid[0].len();

    let mut board = Board::new(
        BoardKind::Empty,
        height + 1,
        width + 1,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        if let Some(n) = horizontal[y] {
            board.push(Item::cell(y + 1, 0, "black", ItemKind::Num(n)));
        }
    }
    for x in 0..width {
        if let Some(n) = vertical[x] {
            board.push(Item::cell(0, x + 1, "black", ItemKind::Num(n)));
        }
    }

    board.add_grid(1, 1, height, width);

    if let Some(ans) = &ans {
        for y in 0..height {
            for x in 0..width {
                match grid[y][x] {
                    BattleshipClue::None => {
                        if let Some(a) = ans[y][x] {
                            board.push(Item::cell(
                                y + 1,
                                x + 1,
                                "green",
                                if a { ItemKind::Block } else { ItemKind::Dot },
                            ));
                        }
                    }
                    BattleshipClue::Water => {
                        board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Dot))
                    }
                    _ => board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Fill)),
                }
            }
        }
    } else {
        for y in 0..height {
            for x in 0..width {
                match grid[y][x] {
                    BattleshipClue::None => {}
                    BattleshipClue::Water => {
                        board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Dot))
                    }
                    _ => board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Fill)),
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
                kind: BoardKind::Empty,
                height: 7,
                width: 7,
                data: vec![
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 13, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 14, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 13, color: "black", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 13, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 13, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
