use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::tetrochain_k;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = tetrochain_k::deserialize_problem(url).ok_or("invalid url")?;
    let ans = tetrochain_k::solve_tetrochain_k(&problem);
    let height = (problem.len() + 1) / 2;
    let width = (problem[0].len() + 1) / 2;
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        ans.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(ref ans) = ans {
                match ans[y][x] {
                    Some(true) => {
                        board.push(Item::cell(y, x, "green", ItemKind::Fill));
                    }
                    Some(false) => {
                        if problem[y * 2][x * 2] == 0 {
                            board.push(Item::cell(y, x, "green", ItemKind::Dot));
                        }
                    }
                    None => {}
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..(width - 1) {
            board.push(Item {
                y: y * 2 + 1,
                x: x * 2 + 2,
                color: "black",
                kind: ItemKind::Wall,
            });
        }
    }
    for y in 0..(height - 1) {
        for x in 0..width {
            board.push(Item {
                y: y * 2 + 2,
                x: x * 2 + 1,
                color: "black",
                kind: ItemKind::Wall,
            });
        }
    }

    for y in 0..(height * 2 - 1) {
        for x in 0..(width * 2 - 1) {
            if problem[y][x] == 0 {
                continue;
            }
            if problem[y][x] == 1 {
                board.push(Item {
                    y: y + 1,
                    x: x + 1,
                    color: "black",
                    kind: ItemKind::SmallFilledCircle,
                });
            } else if problem[y][x] == 2 {
                board.push(Item {
                    y: y + 1,
                    x: x + 1,
                    color: "#cccccc",
                    kind: ItemKind::SmallFilledCircle,
                });
            } else if problem[y][x] == 3 {
                board.push(Item {
                    y: y + 1,
                    x: x + 1,
                    color: "white",
                    kind: ItemKind::SmallFilledCircle,
                });
            }
            board.push(Item {
                y: y + 1,
                x: x + 1,
                color: "black",
                kind: ItemKind::SmallCircle,
            });
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
            solve("https://pzprxs.vercel.app/p?tetrochaink/7/5/m4hcg4m4g42t8h3j1j4m"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 5,
                width: 7,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 13, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 2, x: 8, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 2, x: 8, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 2, x: 12, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 2, x: 12, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 4, x: 2, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 4, x: 2, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 4, x: 6, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 4, x: 6, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 4, x: 9, color: "#cccccc", kind: ItemKind::SmallFilledCircle },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 6, x: 12, color: "#cccccc", kind: ItemKind::SmallFilledCircle },
                    Item { y: 6, x: 12, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 7, x: 6, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 8, x: 12, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 8, x: 12, color: "black", kind: ItemKind::SmallCircle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
