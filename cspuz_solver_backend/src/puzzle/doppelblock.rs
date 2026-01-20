use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs_puzzles::puzzles::doppelblock;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues_up, clues_left, cells) =
        doppelblock::deserialize_problem(url).ok_or("invalid url")?;
    let answer: Vec<Vec<Option<i32>>> =
        doppelblock::solve_doppelblock(&clues_up, &clues_left, &cells).ok_or("no answer")?;

    let height = clues_left.len();
    let width = clues_up.len();

    let mut is_unique = Uniqueness::Unique;
    for y in 0..height {
        for x in 0..width {
            if answer[y][x].is_none() || answer[y][x] == Some(-1) {
                is_unique = Uniqueness::NonUnique;
            }
        }
    }
    let mut board = Board::new(BoardKind::Empty, height + 1, width + 1, is_unique);

    for y in 0..height {
        if let Some(n) = clues_left[y] {
            board.push(Item::cell(y + 1, 0, "black", ItemKind::Num(n)));
        }
    }
    for x in 0..width {
        if let Some(n) = clues_up[x] {
            board.push(Item::cell(0, x + 1, "black", ItemKind::Num(n)));
        }
    }

    board.add_grid(1, 1, height, width);

    for y in 0..height {
        for x in 0..width {
            if let Some(cells) = &cells {
                if let Some(n) = cells[y][x] {
                    board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Num(n)));
                    continue;
                }
            }

            if let Some(n) = answer[y][x] {
                if n == 0 {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Block));
                } else if n == -1 {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Circle));
                } else {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Num(n)));
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
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("https://puzz.link/p?doppelblock/5/5/61j4g35"),
            Board {
                kind: BoardKind::Empty,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(6) },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 12, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Num(1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
