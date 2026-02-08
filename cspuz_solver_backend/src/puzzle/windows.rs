use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::windows;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = windows::deserialize_problem(url).ok_or("invalid url")?;
    let ans = windows::solve_windows(&borders, &clues);
    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if clues[y][x] == Some(0) {
                board.push(Item::cell(y, x, "black", ItemKind::Dot));
            } else if clues[y][x] == Some(1) {
                board.push(Item::cell(y, x, "black", ItemKind::Block));
            } else if let Some(ref ans) = ans {
                if let Some(b) = ans[y][x] {
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
            solve("https://pedros.works/kudamono/player?W=5x6&L=b2b4w14&SIE=2R2RRUU3UURR9UURRUU5UUR10R6LUU&G=windows"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 5,
                data: vec![
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 3, color: "black", kind: ItemKind::Block },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
