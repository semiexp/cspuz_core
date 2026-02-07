use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::moonsun;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = moonsun::deserialize_problem(url).ok_or("invalid url")?;
    let ans = moonsun::solve_moonsun(&borders, &clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );
    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if clues[y][x] != 0 {
                board.push(Item::cell(
                    y,
                    x,
                    "black",
                    if clues[y][x] == 1 {
                        ItemKind::Circle
                    } else {
                        ItemKind::FilledCircle
                    },
                ))
            }
        }
    }

    if let Some(is_line) = &ans {
        board.add_lines_irrefutable_facts(is_line, "green", None);
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
            solve("https://puzz.link/p?moonsun/6/6/adclai5dipkg903l916i7306"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 11, x: 9, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
