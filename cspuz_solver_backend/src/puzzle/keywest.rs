use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::keywest;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let clues = keywest::deserialize_problem(url).ok_or("invalid url")?;
    let ans = keywest::solve_keywest(&clues);
    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Empty,
        height,
        width,
        ans.as_ref()
            .map_or(Uniqueness::NoAnswer, |a| is_unique(&(&a.0, &a.1))),
    );

    if let Some((num, has_line)) = &ans {
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 {
                    if let Some(n) = has_line.vertical[y][x] {
                        board.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color: "green",
                            kind: if n { ItemKind::Line } else { ItemKind::Cross },
                        });
                    }
                }
                if x < width - 1 {
                    if let Some(n) = has_line.horizontal[y][x] {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 2,
                            color: "green",
                            kind: if n { ItemKind::Line } else { ItemKind::Cross },
                        });
                    }
                }
            }
        }
        for y in 0..height {
            for x in 0..width {
                board.push(Item::cell(y, x, "white", ItemKind::FilledCircle));
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
                if let Some(n) = clues[y][x] {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else if let Some(n) = num[y][x] {
                    board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
                }
            }
        }
    } else {
        for y in 0..height {
            for x in 0..width {
                board.push(Item::cell(y, x, "white", ItemKind::FilledCircle));
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
                if let Some(n) = clues[y][x] {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
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
            solve("https://pzprxs.vercel.app/p?keywest/6/5/ib0dgcebmd"),
            Board {
                kind: BoardKind::Empty,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 1, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 3, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 5, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(0) },
                    Item { y: 1, x: 7, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 9, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 11, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 1, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 3, x: 3, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 5, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 7, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(0) },
                    Item { y: 3, x: 9, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 11, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 1, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 3, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 5, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 5, x: 7, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 9, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 5, x: 11, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 1, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 3, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 5, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 7, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 9, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 11, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 1, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 3, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 5, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 7, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 9, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 11, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Num(1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
