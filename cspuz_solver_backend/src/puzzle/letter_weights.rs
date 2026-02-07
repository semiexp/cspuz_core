use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::letter_weights;

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (sums, chars, nums) = letter_weights::deserialize_problem(url)?;
    let (chars, nums, ans) = letter_weights::solve_letter_weights(&sums, &chars, &nums);

    let mut board = Board::new(
        BoardKind::Grid,
        chars.len() + 1,
        nums.len() + 1,
        check_uniqueness(&ans),
    );

    for y in 0..chars.len() {
        assert!('A' <= chars[y] && chars[y] <= 'Z');
        let i = (chars[y] as u8 - b'A') as usize;
        board.push(Item::cell(y + 1, 0, "black", ItemKind::Text(&ALPHA[i..=i])));
    }

    for x in 0..nums.len() {
        board.push(Item::cell(0, x + 1, "black", ItemKind::Num(nums[x])));
    }

    if let Some(ans) = &ans {
        for y in 0..chars.len() {
            for x in 0..nums.len() {
                if let Some(clue) = ans[y][x] {
                    board.push(Item::cell(
                        y + 1,
                        x + 1,
                        "green",
                        if clue {
                            ItemKind::Circle
                        } else {
                            ItemKind::Dot
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
            solve("https://pedros.works/paper-puzzle-player?W=9x5&L=x0x1D1D1x1x1x1O1O1x1x1x1(4)1O1x1x1x1x1R1x1x1x1x1(18)1x1x1x1x1x1x1x1(11)1(3)1(1)1x1x1x1x1x1x1x1R1O1D1x1&L-MATH=p6p1e8p1p9e9&G=letter-weights"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 4,
                data: vec![
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Text("D") },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Text("O") },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Text("R") },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(11) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Circle },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Circle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
