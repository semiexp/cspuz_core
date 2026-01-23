use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::slashpack;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = slashpack::deserialize_problem(url).ok_or("invalid url")?;
    let ans = slashpack::solve_slashpack(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        ans.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some(ans) = &ans {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        match a {
                            slashpack::SLASHPACK_EMPTY => ItemKind::Dot,
                            slashpack::SLASHPACK_SLASH => ItemKind::Slash,
                            slashpack::SLASHPACK_BACKSLASH => ItemKind::Backslash,
                            _ => panic!(),
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
            solve("https://puzz.link/p?slashpack/6/6/h1j1m1i2j.o.g"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Backslash },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Backslash },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Slash },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Backslash },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Slash },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Slash },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Backslash },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Backslash },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Backslash },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Text("?") },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Backslash },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Slash },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Slash },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Slash },
                    Item { y: 11, x: 9, color: "black", kind: ItemKind::Text("?") },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
