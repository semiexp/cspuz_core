use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::narrowfence;

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (clues, holes) = narrowfence::deserialize_problem(url).ok_or("invalid url")?;
    let border = narrowfence::solve_narrowfence(&clues, &holes);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::ColoredGrid("#cccccc"),
        height,
        width,
        check_uniqueness(&border),
    );
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = clues[y][x] {
                if (1..=26).contains(&clue) {
                    let p = (clue - 1) as usize;
                    board.push(Item::cell(y, x, "black", ItemKind::Text(&ALPHA[p..=p])));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue - 26)));
                }
            }
        }
    }

    if let Some(holes) = holes {
        for y in 0..height {
            for x in 0..width {
                if holes[y][x] {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                }
            }
        }
    }

    board.add_borders_as_answer(border.as_ref());

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
            solve("https://pzprxs.vercel.app/p?narrow/5/4/11g1n22l"),
            Board {
                kind: BoardKind::ColoredGrid("#cccccc"),
                height: 4,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Text("B") },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Text("B") },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
