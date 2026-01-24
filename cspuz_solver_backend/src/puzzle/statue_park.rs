use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::statue_park::{self, StatueParkClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (problem, pieces) = statue_park::deserialize_problem(url).ok_or("invalid url")?;
    let ans = statue_park::solve_statue_park(&problem, &pieces);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        ans.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );
    if let Some(ans) = &ans {
        for y in 0..height {
            for x in 0..width {
                match problem[y][x] {
                    StatueParkClue::None => {
                        if let Some(a) = ans[y][x] {
                            board.push(Item::cell(
                                y,
                                x,
                                "green",
                                if a { ItemKind::Block } else { ItemKind::Dot },
                            ));
                        }
                    }
                    StatueParkClue::White => board.push(Item::cell(y, x, "black", ItemKind::Dot)),
                    StatueParkClue::Black => board.push(Item::cell(y, x, "black", ItemKind::Fill)),
                }
            }
        }
    } else {
        for y in 0..height {
            for x in 0..width {
                match problem[y][x] {
                    StatueParkClue::None => {}
                    StatueParkClue::White => board.push(Item::cell(y, x, "black", ItemKind::Dot)),
                    StatueParkClue::Black => board.push(Item::cell(y, x, "black", ItemKind::Fill)),
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
            solve("https://puzz.link/p?statuepark/6/5/0000591i00/2/23lg/22u"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
