use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::hidato;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = hidato::deserialize_problem(url).ok_or("invalid url")?;
    let answer = hidato::solve_hidato(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&answer),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = problem[y][x] {
                if n == -1 {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                }
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
            solve("https://pedros.works/paper-puzzle-player?W=4x3&L=(4)2(6)1x3(1)3(11)2&G=hidoku"),
            Board {
                kind: BoardKind::Grid,
                height: 3,
                width: 4,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(11) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(10) },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Num(6) },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Num(1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
