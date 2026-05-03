use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::nuriloop;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = nuriloop::deserialize_problem(url).ok_or("invalid url")?;
    let ans = nuriloop::solve_nuriloop(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    if let Some(is_line) = &ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(clue) = problem[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "black",
                        if clue >= 0 {
                            ItemKind::Num(clue)
                        } else {
                            ItemKind::Text("?")
                        },
                    ));
                }
            }
        }

        board.add_lines_irrefutable_facts(is_line, "green", None);
    } else {
        for y in 0..height {
            for x in 0..width {
                if let Some(clue) = problem[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "black",
                        if clue >= 0 {
                            ItemKind::Num(clue)
                        } else {
                            ItemKind::Text("?")
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
            solve("https://pzprxs.vercel.app/p?nuriloop/3/3/l1h"),
            Board {
                kind: BoardKind::Grid,
                height: 3,
                width: 3,
                data: vec![
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
