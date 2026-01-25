use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs_puzzles::puzzles::kouchoku;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = kouchoku::deserialize_problem(url).ok_or("invalid url")?;
    let ans = kouchoku::solve_kouchoku(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Empty,
        height,
        width,
        if let Some((_, undet_lines)) = &ans {
            if undet_lines.is_empty() {
                Uniqueness::Unique
            } else {
                Uniqueness::NonUnique
            }
        } else {
            Uniqueness::NoAnswer
        },
    );
    for y in 0..height {
        for x in 0..width {
            if y < height - 1 {
                board.push(Item {
                    y: y * 2 + 2,
                    x: x * 2 + 1,
                    color: "black",
                    kind: ItemKind::DottedLine,
                });
            }
            if x < width - 1 {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2 + 2,
                    color: "black",
                    kind: ItemKind::DottedLine,
                });
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if clue >= 0 {
                    board.push(Item::cell(y, x, "white", ItemKind::FilledCircle));
                    board.push(Item::cell(y, x, "black", ItemKind::Circle));
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue + 1)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::FilledCircle));
                }
            }
        }
    }

    if let Some((fixed_lines, undet_lines)) = ans {
        for ((x1, y1), (x2, y2)) in fixed_lines {
            board.push(Item {
                y: y1 * 2 + 1,
                x: x1 * 2 + 1,
                color: "green",
                kind: ItemKind::LineTo(y2 as i32 * 2 + 1, x2 as i32 * 2 + 1),
            });
        }
        for ((x1, y1), (x2, y2)) in undet_lines {
            board.push(Item {
                y: y1 * 2 + 1,
                x: x1 * 2 + 1,
                color: "#888888",
                kind: ItemKind::LineTo(y2 as i32 * 2 + 1, x2 as i32 * 2 + 1),
            });
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
            solve("https://puzz.link/p?kouchoku/6/6/b1.2.0c1a0.6a0c0b93.2b2"),
            Board {
                kind: BoardKind::Empty,
                height: 7,
                width: 7,
                data: vec![
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 1, x: 10, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 1, x: 12, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 2, x: 13, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 4, x: 13, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 6, x: 13, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 8, x: 13, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 10, x: 1, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 10, x: 3, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 10, x: 7, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 10, x: 9, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 10, x: 11, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 10, x: 13, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 12, x: 1, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 12, x: 3, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 12, x: 7, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 11, x: 10, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 12, x: 13, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 13, x: 2, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 13, x: 4, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 13, x: 6, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 13, x: 8, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 13, x: 10, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 13, x: 12, color: "black", kind: ItemKind::DottedLine },
                    Item { y: 1, x: 1, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 5, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 11, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 3, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 7, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 11, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 11, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 11, x: 13, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 13, x: 7, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 13, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 13, x: 7, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::LineTo(13, 7) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::LineTo(3, 11) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::LineTo(5, 1) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::LineTo(7, 7) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::LineTo(7, 3) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::LineTo(11, 13) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::LineTo(11, 13) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::LineTo(13, 7) },
                    Item { y: 1, x: 1, color: "#888888", kind: ItemKind::LineTo(1, 7) },
                    Item { y: 1, x: 1, color: "#888888", kind: ItemKind::LineTo(3, 1) },
                    Item { y: 1, x: 7, color: "#888888", kind: ItemKind::LineTo(3, 5) },
                    Item { y: 3, x: 1, color: "#888888", kind: ItemKind::LineTo(7, 3) },
                    Item { y: 3, x: 5, color: "#888888", kind: ItemKind::LineTo(5, 1) },
                    Item { y: 5, x: 1, color: "#888888", kind: ItemKind::LineTo(7, 3) },
                ],
                uniqueness: Uniqueness::NonUnique,
            },
        );
    }
}
