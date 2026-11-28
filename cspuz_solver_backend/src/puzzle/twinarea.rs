use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::twinarea;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues, is_gray) = twinarea::deserialize_problem(url).ok_or("invalid url")?;
    let ans = twinarea::solve_twinarea(&borders, &clues, &is_gray);
    let (height, width) = borders.base_shape();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    for y in 0..height {
        for x in 0..width {
            if is_gray[y][x] {
                board.push(Item::cell(y, x, "#cccccc", ItemKind::Fill));
            }
            if let Some(n) = clues[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            } else if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if n == 0 {
                            ItemKind::Dot
                        } else {
                            ItemKind::Num(n)
                        },
                    ));
                }
            }
        }
    }
    board.add_borders(&borders, "black");

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
            solve("https://pzprxs.vercel.app/p?twinarea/5/4/crr0odizddhu"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 1, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
