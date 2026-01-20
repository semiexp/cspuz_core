use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::roma;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = roma::deserialize_problem(url).ok_or("invalid url")?;
    let ans = roma::solve_roma(&borders, &clues).ok_or("no answer")?;
    let height = ans.len();
    let width = ans[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));

    board.add_borders(&borders, "black");

    let mut add_arrow = |y: usize, x: usize, kind: i32, color: &'static str| match kind {
        roma::CLUE_UP => {
            board.push(Item::cell(y, x, color, ItemKind::ArrowUp));
        }
        roma::CLUE_DOWN => {
            board.push(Item::cell(y, x, color, ItemKind::ArrowDown));
        }
        roma::CLUE_LEFT => {
            board.push(Item::cell(y, x, color, ItemKind::ArrowLeft));
        }
        roma::CLUE_RIGHT => {
            board.push(Item::cell(y, x, color, ItemKind::ArrowRight));
        }
        roma::CLUE_GOAL => {
            board.push(Item::cell(y, x, color, ItemKind::FilledCircle));
        }
        _ => {}
    };

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                add_arrow(y, x, n, "black");
            } else if let Some(n) = ans[y][x] {
                add_arrow(y, x, n, "green");
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
            solve("https://puzz.link/p?roma/5/5/tvv2bi1vc4b3a522b2c13a4b4a"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::ArrowRight },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::ArrowRight },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::ArrowDown },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::ArrowRight },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::ArrowDown },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::ArrowUp },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::ArrowLeft },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::ArrowRight },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::ArrowDown },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::ArrowDown },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::ArrowUp },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::ArrowLeft },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::ArrowDown },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::ArrowLeft },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::ArrowRight },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::ArrowRight },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::ArrowUp },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::ArrowLeft },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::ArrowUp },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::ArrowRight },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::ArrowUp },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::ArrowLeft },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::ArrowRight },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::ArrowUp },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
