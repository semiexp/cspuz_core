use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::paintarea;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = paintarea::deserialize_problem(url).ok_or("invalid url")?;
    let ans = paintarea::solve_paintarea(&borders, &clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    for y in 0..height {
        for x in 0..width {
            if let Some(is_black) = &ans {
                if let Some(b) = is_black[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b { ItemKind::Fill } else { ItemKind::Dot },
                    ));
                }
            }

            if let Some(clue) = clues[y][x] {
                if clue >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            }
        }
    }

    board.add_borders(&borders, "black");

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
            solve("https://puzz.link/p?paintarea/3/3/40vgc2e"),
            Board {
                kind: BoardKind::Grid,
                height: 3,
                width: 3,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
