use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::railpool;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues, borders, holes) = railpool::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = railpool::solve_railpool(&borders, &clues, &holes);

    // Get dimensions from borders: vertical has height rows, horizontal[0] has width elements
    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&is_line));

    board.add_borders(&borders, "black");

    if let Some(holes) = &holes {
        for y in 0..height {
            for x in 0..width {
                if holes[y][x] {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let clue = &clues[y][x];
            if clue.is_empty() {
                continue;
            }

            if clue.len() <= 4 {
                let mut c = [-1, -1, -1, -1];
                for i in 0..clue.len() {
                    c[i] = if clue[i] == 0 { -2 } else { clue[i] };
                }
                board.push(Item::cell(y, x, "black", ItemKind::TapaClue(c)));
            } else {
                board.push(Item::cell(y, x, "black", ItemKind::Text("...")));
            }
        }
    }

    if let Some(is_line) = &is_line {
        board.add_lines_irrefutable_facts(is_line, "green", holes.as_ref());
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
            solve("https://puzz.link/p?railpool/7/5/0zp00o20rg8032h040gg00000020"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 7,
                data: vec![
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::TapaClue([-2, -1, -1, -1]) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::TapaClue([-2, -2, -1, -1]) },
                    Item { y: 7, x: 13, color: "black", kind: ItemKind::TapaClue([2, -2, -1, -1]) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 12, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
