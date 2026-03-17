use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::lightandshadow;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = lightandshadow::deserialize_problem(url).ok_or("invalid url")?;
    let ans = lightandshadow::solve_lightandshadow(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some((clue, state)) = problem[y][x] {
                if !state {
                    if clue > 0 {
                        board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                    } else {
                        board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                    }
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                    if clue > 0 {
                        board.push(Item::cell(y, x, "white", ItemKind::Num(clue)));
                    } else {
                        board.push(Item::cell(y, x, "white", ItemKind::Text("?")));
                    }
                }
            } else if let Some(ans) = &ans {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if a { ItemKind::Block } else { ItemKind::Dot },
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
            solve("https://pzprxs.vercel.app/p?lightshadow/2/2/05h"),
            Board {
                kind: BoardKind::Grid,
                height: 2,
                width: 2,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Text("?") },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Fill },
                    Item { y: 1, x: 3, color: "white", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Block },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
