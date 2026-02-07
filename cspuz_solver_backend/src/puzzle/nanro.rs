use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::nanro;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = nanro::deserialize_problem(url).ok_or("invalid url")?;
    let ans = nanro::solve_nanro(&borders, &clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = clues[y][x] {
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if n == 0 {
                            ItemKind::Dot
                        } else {
                            ItemKind::Num(n)
                        },
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
            solve("https://puzz.link/p?nanro/4/4/6r0s1oi13n1h"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 4,
                data: vec![
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
