use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::lits;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let borders = lits::deserialize_problem(url).ok_or("invalid url")?;
    let ans = lits::solve_lits(&borders);

    let height = borders.horizontal.len() + 1;
    let width = if borders.horizontal.is_empty() {
        0
    } else {
        borders.horizontal[0].len()
    };
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    board.add_borders(&borders, "black");

    if let Some(is_black) = &ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(b) = is_black[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b { ItemKind::Block } else { ItemKind::Dot },
                    ));
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
            solve("https://puzz.link/p?lits/10/10/08p0i3jbhmjg5j5ik048rgtr8q1e5gkf9hnu"),
            Board {
                kind: BoardKind::Grid,
                height: 10,
                width: 10,
                data: vec![
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 18, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 18, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 18, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 14, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 18, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 18, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 18, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 18, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 18, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 18, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 18, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 18, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 18, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 19, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 15, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 15, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 19, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 19, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 19, color: "green", kind: ItemKind::Block },
                    Item { y: 13, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 13, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 19, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 15, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 17, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 19, color: "green", kind: ItemKind::Block },
                    Item { y: 17, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 17, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 17, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 17, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 17, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 17, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 17, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 17, color: "green", kind: ItemKind::Block },
                    Item { y: 17, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 19, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 19, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 19, x: 15, color: "green", kind: ItemKind::Block },
                    Item { y: 19, x: 17, color: "green", kind: ItemKind::Block },
                    Item { y: 19, x: 19, color: "green", kind: ItemKind::Block },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
