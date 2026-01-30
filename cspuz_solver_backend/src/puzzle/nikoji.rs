use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::nikoji;

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = nikoji::deserialize_problem(url).ok_or("invalid url")?;
    let border = nikoji::solve_nikoji(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        border
            .as_ref()
            .map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if (1..=26).contains(&clue) {
                    let p = (clue - 1) as usize;
                    board.push(Item::cell(y, x, "black", ItemKind::Text(&ALPHA[p..=p])));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue - 26)));
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
            solve("https://puzz.link/p?nikoji/6/5/1i3m21g5h3i4h4g211"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Text("C") },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Text("B") },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Text("E") },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Text("C") },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Text("D") },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::Text("D") },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Text("B") },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 9, x: 11, color: "black", kind: ItemKind::Text("A") },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
