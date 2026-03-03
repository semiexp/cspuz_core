use crate::board::{Board, BoardKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::doubleback;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let borders = doubleback::deserialize_problem(url).ok_or("invalid url")?;
    let ans = doubleback::solve_doubleback(&borders);

    let height = borders.vertical.len();
    let width = borders.vertical[0].len() + 1;
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));
    board.add_borders(&borders, "black");

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
            solve("https://puzz.link/p?doubleback/4/4/14063o"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 4,
                data: vec![
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall }, 
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall }, 
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall }, 
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::BoldWall }, 
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall }, 
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall }, 
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall }, 
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall }, 
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line }, 
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line }, 
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line }, 
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line }, 
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line }, 
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line }, 
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line }, 
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line }, 
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line }, 
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line }, 
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line }, 
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line }, 
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line }, 
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line }, 
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line }, 
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
