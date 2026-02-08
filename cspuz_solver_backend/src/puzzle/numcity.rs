use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::numcity;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues, borders) = numcity::deserialize_problem(url).ok_or("invalid url")?;
    let ans = numcity::solve_numcity(&borders, &clues);
    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
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
            solve("https://pzprxs.vercel.app/p?numcity/6/5/g1i2zg4haqa44881jg"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Num(1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
