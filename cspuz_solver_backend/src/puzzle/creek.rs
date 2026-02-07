use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::creek;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = creek::deserialize_problem(url).ok_or("invalid url")?;
    let ans = creek::solve_creek(&problem);

    let height = problem.len() - 1;
    let width = problem[0].len() - 1;
    let mut board = Board::new(
        BoardKind::Empty,
        height,
        width,
        check_uniqueness(&ans),
    );
    if let Some(ans) = ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if a { ItemKind::Fill } else { ItemKind::Dot },
                    ));
                }
            }
        }
    }
    for y in 0..=height {
        for x in 0..=width {
            if y < height {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::Wall,
                });
            }
            if x < width {
                board.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "black",
                    kind: ItemKind::Wall,
                });
            }
        }
    }
    for y in 0..=height {
        for x in 0..=width {
            if let Some(n) = problem[y][x] {
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "white",
                    kind: ItemKind::FilledCircle,
                });
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::Circle,
                });
                if n >= 0 {
                    board.push(Item {
                        y: y * 2,
                        x: x * 2,
                        color: "black",
                        kind: ItemKind::Num(n),
                    });
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
            solve("https://puzz.link/p?creek/6/7/q2cgcj18cdm3c88cl"),
            Board {
                kind: BoardKind::Empty,
                height: 7,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 0, color: "black", kind: ItemKind::Wall },
                    Item { y: 0, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 0, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 0, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 0, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 0, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 0, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 0, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 0, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 0, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 0, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 0, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 0, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 13, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 14, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 14, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 14, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 14, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 14, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 14, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 8, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 8, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 2, x: 10, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 10, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 10, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 4, x: 4, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 4, x: 4, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 4, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 6, x: 4, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 6, x: 4, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 4, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 6, x: 6, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 6, x: 6, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 6, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 6, x: 10, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 6, x: 10, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 10, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 8, x: 2, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 8, x: 2, color: "black", kind: ItemKind::Circle },
                    Item { y: 8, x: 2, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 10, x: 8, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 10, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 10, x: 8, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 10, x: 10, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 10, x: 10, color: "black", kind: ItemKind::Circle },
                    Item { y: 10, x: 10, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 12, x: 2, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 12, x: 2, color: "black", kind: ItemKind::Circle },
                    Item { y: 12, x: 2, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 12, x: 6, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 12, x: 6, color: "black", kind: ItemKind::Circle },
                    Item { y: 12, x: 6, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 12, x: 10, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 12, x: 10, color: "black", kind: ItemKind::Circle },
                    Item { y: 12, x: 10, color: "black", kind: ItemKind::Num(2) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
