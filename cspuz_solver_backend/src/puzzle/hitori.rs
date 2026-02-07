use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::hitori;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = hitori::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = hitori::solve_hitori(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&is_black),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(is_black) = &is_black {
                match (problem[y][x], is_black[y][x]) {
                    (0, None) => (),
                    (0, Some(false)) => {
                        board.push(Item::cell(y, x, "green", ItemKind::Dot));
                    }
                    (0, Some(true)) => {
                        board.push(Item::cell(y, x, "green", ItemKind::Fill));
                    }
                    (n, None) => {
                        board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                    }
                    (n, Some(false)) => {
                        board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                        board.push(Item::cell(y, x, "green", ItemKind::Dot));
                    }
                    (n, Some(true)) => {
                        board.push(Item::cell(y, x, "green", ItemKind::Fill));
                        board.push(Item::cell(y, x, "white", ItemKind::Num(n)));
                    }
                }
            } else {
                // No solution: show only clues
                if problem[y][x] != 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(problem[y][x])));
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
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("https://puzz.link/p?hitori/6/5/111.45.2..453.3.1..2....3.3.1."),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 1, color: "white", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "white", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 11, color: "white", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "white", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "white", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "white", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 1, color: "white", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 9, color: "white", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
