use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs::graph;
use cspuz_rs_puzzles::puzzles::stostone;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = stostone::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = stostone::solve_stostone(&borders, &clues);

    let height = borders.horizontal.len() + 1;
    let width = borders.horizontal[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&is_black));

    board.add_borders(&borders, "black");

    if let Some(is_black) = &is_black {
        for y in 0..height {
            for x in 0..width {
                if let Some(b) = is_black[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b { ItemKind::Block } else { ItemKind::Dot },
                    ));
                }
            }
        }
    }
    let rooms = graph::borders_to_rooms(&borders);
    assert_eq!(rooms.len(), clues.len());
    for i in 0..rooms.len() {
        if let Some(n) = clues[i] {
            let (y, x) = rooms[i][0];
            if n >= 0 {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            } else {
                board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
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
            solve("https://puzz.link/p?stostone/6/6/222ac4vg1ve831h3g23"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Num(3) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
