use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::tontonbeya;

fn num_to_item(n: i32) -> ItemKind {
    match n {
        0 => ItemKind::Circle,
        1 => ItemKind::Triangle,
        2 => ItemKind::Square,
        _ => panic!(),
    }
}

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = tontonbeya::deserialize_problem(url).ok_or("invalid url")?;
    let answer = tontonbeya::solve_tontonbeya(&borders, &clues).ok_or("no answer")?;

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&answer));
    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                board.push(Item::cell(y, x, "black", num_to_item(n)));
            } else if let Some(n) = answer[y][x] {
                board.push(Item::cell(y, x, "green", num_to_item(n)));
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
            solve("https://puzz.link/p?tontonbeya/6/5/aiqm28351oa1e3d2h1h"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Circle },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Circle },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Triangle },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Triangle },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Triangle },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Square },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Triangle },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Triangle },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Triangle },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Square },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Square },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Triangle },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Triangle },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Circle },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Triangle },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Square },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Square },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Circle },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Circle },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Triangle },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Triangle },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Triangle },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Triangle },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Circle },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Circle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
