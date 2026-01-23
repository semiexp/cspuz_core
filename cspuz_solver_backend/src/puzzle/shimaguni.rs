use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs::graph;
use cspuz_rs_puzzles::puzzles::shimaguni;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = shimaguni::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = shimaguni::solve_shimaguni(&borders, &clues);

    let height = borders.vertical.len();
    let width = borders.vertical[0].len() + 1;
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        is_black
            .as_ref()
            .map_or(Uniqueness::NoAnswer, |b| is_unique(b)),
    );

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
            solve("https://puzz.link/p?shimaguni/6/6/6qrna9sbh1i2j3h2"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Num(2) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
