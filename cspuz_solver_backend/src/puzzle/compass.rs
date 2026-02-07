use crate::board::{Board, BoardKind, Compass, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::compass;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = compass::deserialize_problem(url).ok_or("invalid url")?;
    let ans = compass::solve_compass(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        check_uniqueness(&ans),
    );
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "black",
                    ItemKind::Compass(Compass {
                        up: clue.up,
                        down: clue.down,
                        left: clue.left,
                        right: clue.right,
                    }),
                ));
            }
        }
    }

    board.add_borders_as_answer(ans.as_ref());

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
            solve("https://puzz.link/p?compass/5/5/m..1.i25.1g53..i1..1m"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Compass(Compass { up: None, down: None, left: Some(1), right: None }) },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Compass(Compass { up: Some(2), down: Some(5), left: None, right: Some(1) }) },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Compass(Compass { up: Some(5), down: Some(3), left: None, right: None }) },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Compass(Compass { up: Some(1), down: None, left: None, right: Some(1) }) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
