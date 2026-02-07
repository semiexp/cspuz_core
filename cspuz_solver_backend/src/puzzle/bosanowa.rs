use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::bosanowa;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (circles, clues) = bosanowa::deserialize_problem(url).ok_or("invalid url")?;
    let ans = bosanowa::solve_bosanowa(&circles, &clues)?;

    let height = circles.len();
    let width = circles[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = clues[y * width + x] {
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    if n > 0 {
                        board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
                    }
                } else if circles[y][x] {
                    board.push(Item::cell(y, x, "black", ItemKind::Circle));
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
            solve("https://puzz.link/p?bosanowa/6/5/jo9037g2n2n3g4j3i"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Num(3) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
