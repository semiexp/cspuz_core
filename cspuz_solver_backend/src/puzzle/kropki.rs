use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::kropki::{self, KropkiClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = kropki::deserialize_problem(url).ok_or("invalid url")?;
    let ans = kropki::solve_kropki(&problem);

    let height = problem.horizontal.len() + 1;
    let width = problem.vertical[0].len() + 1;
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        ans.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
                }
            }
            if y < height - 1 {
                if problem.horizontal[y][x] == KropkiClue::White {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "black",
                        kind: ItemKind::SmallCircle,
                    });
                } else if problem.horizontal[y][x] == KropkiClue::Black {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "black",
                        kind: ItemKind::SmallFilledCircle,
                    });
                }
            }
            if x < width - 1 {
                if problem.vertical[y][x] == KropkiClue::White {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "black",
                        kind: ItemKind::SmallCircle,
                    });
                } else if problem.vertical[y][x] == KropkiClue::Black {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "black",
                        kind: ItemKind::SmallFilledCircle,
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
            solve("https://puzz.link/p?kropki/5/5/da05f05304410i"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(2) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
