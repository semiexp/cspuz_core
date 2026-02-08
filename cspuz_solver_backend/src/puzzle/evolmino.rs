use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::evolmino;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = evolmino::deserialize_problem(url).ok_or("invalid url")?;
    let ans = evolmino::solve_evolmino(&problem);

    let height = problem.cells.len();
    let width = problem.cells[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    for arrow in &problem.arrows {
        for i in 1..arrow.len() {
            let (y1, x1) = arrow[i - 1];
            let (y2, x2) = arrow[i];

            board.push(Item {
                y: y1 + y2 + 1,
                x: x1 + x2 + 1,
                color: "black",
                kind: ItemKind::Line,
            });
        }
    }

    for y in 0..height {
        for x in 0..width {
            if problem.cells[y][x] == evolmino::ProblemCell::Black {
                board.push(Item::cell(y, x, "black", ItemKind::Fill));
                continue;
            }
            if problem.cells[y][x] == evolmino::ProblemCell::Square {
                board.push(Item::cell(y, x, "black", ItemKind::Square));
                continue;
            }
            if let Some(is_square) = &ans {
                match is_square[y][x] {
                    Some(true) => {
                        board.push(Item::cell(y, x, "green", ItemKind::Square));
                    }
                    Some(false) => {
                        board.push(Item::cell(y, x, "green", ItemKind::Dot));
                    }
                    None => (),
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
            solve("https://puzz.link/p?evolmino/6/7/i6900910k00005zz1p0008222o"),
            Board {
                kind: BoardKind::Grid,
                height: 7,
                width: 6,
                data: vec![
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Line },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::Line },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::Line },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::Line },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::Line },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::Line },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::Line },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Square },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Square },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Square },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Square },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Square },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Square },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Square },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Square },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Square },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Square },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Square },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Square },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Square },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Square },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Square },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Square },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Square },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Square },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Square },
                    Item { y: 13, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 5, color: "green", kind: ItemKind::Square },
                    Item { y: 13, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 9, color: "black", kind: ItemKind::Fill },
                    Item { y: 13, x: 11, color: "black", kind: ItemKind::Square },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
