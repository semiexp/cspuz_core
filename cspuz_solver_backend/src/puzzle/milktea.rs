use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::milktea;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let clues = milktea::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = milktea::solve_milktea(&clues).ok_or("no answer")?;

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Empty, height, width, is_unique(&is_line));

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    for y in 0..height {
        for x in 0..width {
            if clues[y][x] == 1 {
                board.push(Item::cell(y, x, "white", ItemKind::FilledCircle));
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
            } else if clues[y][x] == 2 {
                board.push(Item::cell(y, x, "black", ItemKind::FilledCircle));
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
            solve("https://pedros.works/paper-puzzle-player?W=6x5&L=b0w1w3w3b1b2b5b2b1w3w3b4&G=milk-tea"),
            Board {
                kind: BoardKind::Empty,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 1, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 9, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 3, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 1, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 9, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::FilledCircle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
