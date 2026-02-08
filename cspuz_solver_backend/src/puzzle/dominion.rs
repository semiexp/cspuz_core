use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::dominion;

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = dominion::deserialize_problem(url).ok_or("invalid url")?;
    let ans = dominion::solve_dominion(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if (1..=26).contains(&clue) {
                    let p = (clue - 1) as usize;
                    board.push(Item::cell(y, x, "black", ItemKind::Text(&ALPHA[p..=p])));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue - 26)));
                }
            } else if let Some(ans) = &ans {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if a { ItemKind::Block } else { ItemKind::Dot },
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
            solve("https://puzz.link/p?dominion/5/5/1h2p3h3h1h.g"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Text("B") },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Text("C") },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Text("C") },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Num(-27) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
