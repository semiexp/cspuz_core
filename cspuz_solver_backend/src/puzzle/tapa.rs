use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::tapa;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = tapa::deserialize_problem(url).ok_or("invalid url")?;
    let ans = tapa::solve_tapa(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::TapaClue(clue)));
            } else if let Some(a) = ans[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if a { ItemKind::Block } else { ItemKind::Dot },
                ));
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
            solve("https://puzz.link/p?tapa/6/8/q2g9g.qb0pa0ccn"),
            Board {
                kind: BoardKind::Grid,
                height: 8,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::TapaClue([2, -1, -1, -1]) },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::TapaClue([1, 1, 1, 1]) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::TapaClue([-2, -1, -1, -1]) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::TapaClue([-2, -2, -2, -1]) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 13, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 5, color: "black", kind: ItemKind::TapaClue([-2, -2, -1, -1]) },
                    Item { y: 13, x: 7, color: "black", kind: ItemKind::TapaClue([3, -2, -2, -1]) },
                    Item { y: 13, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 11, color: "green", kind: ItemKind::Block },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
