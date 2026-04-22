use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::fillmat;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = fillmat::deserialize_problem(url).ok_or("invalid url")?;
    let ans = fillmat::solve_fillmat(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::ColoredGrid("#cccccc"),
        height,
        width,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = problem[y][x] {
                if n >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else if n == -2 {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                }
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
            solve("https://puzz.link/p?fillmat/3/3/c2b3b"),
            Board {
                kind: BoardKind::ColoredGrid("#cccccc"),
                height: 3,
                width: 3,
                data: vec![
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
