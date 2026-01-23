use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::soulmates;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = soulmates::deserialize_problem(url).ok_or("invalid url")?;
    let answer = soulmates::solve_soulmates(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        answer
            .as_ref()
            .map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            } else if let Some(answer) = &answer {
                if let Some(n) = answer[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if n == 0 {
                            ItemKind::Dot
                        } else {
                            ItemKind::Num(n)
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
            solve("https://pedros.works/paper-puzzle-player?W=4x4&L=(3)0(10)3(1)4&G=soulmates"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 4,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(10) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(10) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
