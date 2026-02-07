use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::akari;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = akari::deserialize_problem(url).ok_or("invalid url")?;
    let ans = akari::solve_akari(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Fill));
                if clue >= 0 {
                    board.push(Item::cell(y, x, "white", ItemKind::Num(clue)));
                }
            } else if let Some(ans) = &ans {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if a { ItemKind::Circle } else { ItemKind::Dot },
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
            solve("https://puzz.link/p?akari/10/10/hcscl.h.idn.i.cgcndg.h.ncs.h"),
            Board {
                kind: BoardKind::Grid,
                height: 10,
                width: 10,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Circle },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "white", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 17, color: "black", kind: ItemKind::Fill },
                    Item { y: 3, x: 17, color: "white", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 19, color: "green", kind: ItemKind::Circle },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Circle },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 15, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 17, color: "green", kind: ItemKind::Circle },
                    Item { y: 5, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 9, color: "white", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Circle },
                    Item { y: 7, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Circle },
                    Item { y: 9, x: 11, color: "black", kind: ItemKind::Fill },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 15, color: "green", kind: ItemKind::Circle },
                    Item { y: 9, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 19, color: "black", kind: ItemKind::Fill },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 11, x: 1, color: "white", kind: ItemKind::Num(2) },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Circle },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "black", kind: ItemKind::Fill },
                    Item { y: 11, x: 9, color: "white", kind: ItemKind::Num(2) },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Circle },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 1, color: "green", kind: ItemKind::Circle },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 11, color: "black", kind: ItemKind::Fill },
                    Item { y: 13, x: 11, color: "white", kind: ItemKind::Num(3) },
                    Item { y: 13, x: 13, color: "green", kind: ItemKind::Circle },
                    Item { y: 13, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 19, color: "black", kind: ItemKind::Fill },
                    Item { y: 15, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 15, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 11, color: "green", kind: ItemKind::Circle },
                    Item { y: 15, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 3, color: "black", kind: ItemKind::Fill },
                    Item { y: 17, x: 3, color: "white", kind: ItemKind::Num(2) },
                    Item { y: 17, x: 5, color: "green", kind: ItemKind::Circle },
                    Item { y: 17, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 3, color: "green", kind: ItemKind::Circle },
                    Item { y: 19, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 15, color: "black", kind: ItemKind::Fill },
                    Item { y: 19, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 19, color: "green", kind: ItemKind::Circle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
