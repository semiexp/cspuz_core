use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::tents;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues_up, clues_left, trees) = tents::deserialize_problem(url).ok_or("invalid url")?;
    let ans = tents::solve_tents(&clues_up, &clues_left, &trees);

    let height = clues_left.len();
    let width = clues_up.len();

    let mut board = Board::new(
        BoardKind::Empty,
        height + 1,
        width + 1,
        check_uniqueness(&ans),
    );

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
            if trees[y][x] {
                board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Circle));
                continue;
            } else if let Some((_pairings, tents)) = &ans {
                if let Some(true) = tents[y][x] {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Triangle));
                } else if let Some(false) = tents[y][x] {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Dot));
                }
            }
        }
    }

    if let Some((pairings, _tents)) = &ans {
        for y in 0..height - 1 {
            for x in 0..width {
                if let Some(b) = pairings.vertical[y][x] {
                    board.push(Item::cell(
                        y * 2 + 4,
                        x * 2 + 3,
                        "green",
                        if b { ItemKind::Line } else { ItemKind::Cross },
                    ));
                }
            }
        }
        for y in 0..height {
            for x in 0..width - 1 {
                if let Some(b) = pairings.horizontal[y][x] {
                    board.push(Item::cell(
                        y * 2 + 3,
                        x * 2 + 4,
                        "green",
                        if b { ItemKind::Line } else { ItemKind::Cross },
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
            solve("https://puzz.link/p?tents/5/5/2g0h1i1j82323"),
            Board {
                kind: BoardKind::Empty,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(0) },
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
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Triangle },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Triangle },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Triangle },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Triangle },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Triangle },
                    Item { y: 11, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 23, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 23, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 23, color: "green", kind: ItemKind::Cross },
                    Item { y: 21, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 21, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 21, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 21, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 21, x: 23, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 21, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 21, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 21, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 21, color: "green", kind: ItemKind::Line },
                    Item { y: 23, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 23, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 23, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 23, x: 21, color: "green", kind: ItemKind::Cross }
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
