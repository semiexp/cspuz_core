use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::nurimaze;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = nurimaze::deserialize_problem(url).ok_or("invalid url")?;
    let ans = nurimaze::solve_nurimaze(&borders, &clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        for x in 0..width {
            match clues[y][x] {
                1 => board.push(Item::cell(y, x, "black", ItemKind::Text("S"))),
                2 => board.push(Item::cell(y, x, "black", ItemKind::Text("G"))),
                3 => board.push(Item::cell(y, x, "black", ItemKind::Circle)),
                4 => board.push(Item::cell(y, x, "black", ItemKind::Triangle)),
                _ => (),
            }

            if let Some(is_black) = &ans {
                if let Some(b) = is_black[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b { ItemKind::Fill } else { ItemKind::Dot },
                    ));
                }
            }
        }
    }

    board.add_borders(&borders, "black");

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
            solve("https://puzz.link/p?nurimaze/6/5/ervrivfppu53b481b2b"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::Triangle },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Text("S") },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Text("G") },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
