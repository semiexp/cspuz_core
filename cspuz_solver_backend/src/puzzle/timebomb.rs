use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::timebomb;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = timebomb::deserialize_problem(url).ok_or("invalid url")?;
    let ans = timebomb::solve_timebomb(&problem);

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
                if clue >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Circle));
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else if clue == -1 {
                    board.push(Item::cell(y, x, "black", ItemKind::Circle));
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                } else if clue == -2 {
                    board.push(Item::cell(y, x, "black", ItemKind::FilledCircle));
                }
            } else if let Some((ref has_number, ref num)) = ans {
                if let Some(n) = num[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if n == -1 {
                            ItemKind::Dot
                        } else {
                            ItemKind::Num(n)
                        },
                    ));
                } else if has_number[y][x] == Some(true) {
                    board.push(Item::cell(y, x, "green", ItemKind::Text("?")));
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
            solve("https://pzprxs.vercel.app/p?timebomb/6/5/5j0h0h0.k0g01g2g0j"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Num(0) },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Text("?") },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(0) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Num(0) },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 11, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(0) },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Num(1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
