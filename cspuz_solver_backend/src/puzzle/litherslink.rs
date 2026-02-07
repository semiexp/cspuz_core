use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::litherslink;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = litherslink::deserialize_problem(url).ok_or("invalid url")?;
    let ans = litherslink::solve_litherslink(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::DotGrid,
        height,
        width,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            }
        }
    }
    if let Some(is_line) = &ans {
        for y in 0..height {
            for x in 0..=width {
                if let Some(b) = is_line.vertical[y][x] {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2,
                        color: "green",
                        kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                    })
                }
            }
        }
        for y in 0..=height {
            for x in 0..width {
                if let Some(b) = is_line.horizontal[y][x] {
                    board.push(Item {
                        y: y * 2,
                        x: x * 2 + 1,
                        color: "green",
                        kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                    })
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
            solve("https://puzz.link/p?lither/4/3/b8dg6d"),
            Board {
                kind: BoardKind::DotGrid,
                height: 3,
                width: 4,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 0, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 0, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
