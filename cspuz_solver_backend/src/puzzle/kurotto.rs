use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::kurotto;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = kurotto::deserialize_problem(url).ok_or("invalid url")?;
    let ans = kurotto::solve_kurotto(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                }
            } else if let Some(a) = ans[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if a { ItemKind::Fill } else { ItemKind::Dot },
                ));
            }
        }
    }

    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::board::*;
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("https://puzz.link/p?kurotto/6/6/3gah.m.i9.iam8h3g2"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(10) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::Num(9) },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Num(10) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Num(8) },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 11, x: 7, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 11, x: 11, color: "black", kind: ItemKind::Num(2) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
