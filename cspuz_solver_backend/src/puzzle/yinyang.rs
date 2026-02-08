use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::yinyang;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use yinyang::YinYangClue;

    let problem = yinyang::deserialize_problem(url).ok_or("invalid url")?;
    let ans = yinyang::solve_yinyang(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                YinYangClue::None => (),
                YinYangClue::White => board.push(Item::cell(y, x, "black", ItemKind::Circle)),
                YinYangClue::Black => board.push(Item::cell(y, x, "black", ItemKind::FilledCircle)),
            }
        }
    }

    if let Some(ref ans) = ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(b) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b {
                            ItemKind::FilledCircle
                        } else {
                            ItemKind::Circle
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
            solve("https://puzz.link/p?yinyang/6/6/6a166b230900"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Circle },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Circle },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Circle },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Circle },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Circle },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Circle },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Circle },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::FilledCircle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
