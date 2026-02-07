use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::kakuro;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = kakuro::deserialize_problem(url).ok_or("invalid url")?;
    let answer = kakuro::solve_kakuro(&problem);

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
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "#cccccc", ItemKind::Fill));
                board.push(Item::cell(y, x, "black", ItemKind::Backslash));

                if let Some(n) = clue.down {
                    if n > 0 {
                        board.push(Item::cell(y, x, "black", ItemKind::NumLowerLeft(n)));
                    }
                }
                if let Some(n) = clue.right {
                    if n > 0 {
                        board.push(Item::cell(y, x, "black", ItemKind::NumUpperRight(n)));
                    }
                }
            } else if let Some(answer) = &answer {
                if let Some(n) = answer[y][x] {
                    board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
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
            solve("https://puzz.link/p?kakuro/6/5/Dclh4t9fl3-p-gl-alJeC3BgG"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 7,
                data: vec![
                    Item { y: 1, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Backslash },
                    Item { y: 1, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Backslash },
                    Item { y: 1, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Backslash },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::NumLowerLeft(29) },
                    Item { y: 1, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Backslash },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::NumLowerLeft(14) },
                    Item { y: 1, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Backslash },
                    Item { y: 1, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Backslash },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::NumLowerLeft(22) },
                    Item { y: 1, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 13, color: "black", kind: ItemKind::Backslash },
                    Item { y: 1, x: 13, color: "black", kind: ItemKind::NumLowerLeft(3) },
                    Item { y: 3, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Backslash },
                    Item { y: 3, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Backslash },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::NumLowerLeft(23) },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::NumUpperRight(12) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 3, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Backslash },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::NumLowerLeft(17) },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::NumUpperRight(4) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 13, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Backslash },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::NumUpperRight(21) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 13, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Backslash },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::NumUpperRight(16) },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 7, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Backslash },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::NumLowerLeft(9) },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::NumUpperRight(15) },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 7, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 13, color: "black", kind: ItemKind::Backslash },
                    Item { y: 7, x: 13, color: "black", kind: ItemKind::NumLowerLeft(3) },
                    Item { y: 9, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Backslash },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::NumUpperRight(26) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 11, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Backslash },
                    Item { y: 11, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 3, color: "black", kind: ItemKind::Backslash },
                    Item { y: 11, x: 3, color: "black", kind: ItemKind::NumUpperRight(16) },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 11, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 9, color: "black", kind: ItemKind::Backslash },
                    Item { y: 11, x: 9, color: "black", kind: ItemKind::NumUpperRight(10) },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Num(2) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
