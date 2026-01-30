use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::dbchoco;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (color, num) = dbchoco::deserialize_problem(url).ok_or("invalid url")?;
    let ans = dbchoco::solve_doublechoco(&color, &num);

    let height = num.len();
    let width = num[0].len();
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        ans.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );

    for y in 0..height {
        for x in 0..width {
            if color[y][x] == 1 {
                board.push(Item::cell(y, x, "#cccccc", ItemKind::Fill));
            }
            if let Some(n) = num[y][x] {
                if n == -1 {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
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
            solve("https://puzz.link/p?dbchoco/6/6/pu9hgpe05zu"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 1, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "#cccccc", kind: ItemKind::Wall },
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
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
