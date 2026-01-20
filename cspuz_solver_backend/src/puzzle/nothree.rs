use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::nothree;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = nothree::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = nothree::solve_nothree(&problem).ok_or("no answer")?;

    let height = is_black.len();
    let width = is_black[0].len();
    let mut board = Board::new(BoardKind::OuterGrid, height, width, is_unique(&is_black));

    for y in 0..height {
        for x in 0..(width - 1) {
            board.push(Item {
                y: y * 2 + 1,
                x: x * 2 + 2,
                color: "black",
                kind: ItemKind::Wall,
            });
        }
    }
    for y in 0..(height - 1) {
        for x in 0..width {
            board.push(Item {
                y: y * 2 + 2,
                x: x * 2 + 1,
                color: "black",
                kind: ItemKind::Wall,
            });
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(b) = is_black[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if b { ItemKind::Block } else { ItemKind::Dot },
                ));
            }
        }
    }

    for y in 0..(height * 2 - 1) {
        for x in 0..(width * 2 - 1) {
            if problem[y][x] {
                board.push(Item {
                    y: y + 1,
                    x: x + 1,
                    color: "white",
                    kind: ItemKind::SmallFilledCircle,
                });
                board.push(Item {
                    y: y + 1,
                    x: x + 1,
                    color: "black",
                    kind: ItemKind::SmallCircle,
                });
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
            solve("https://puzz.link/p?nothree/6/5/ger26eneq22eleq"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 2, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 2, x: 11, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 3, x: 2, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 3, x: 6, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 4, x: 11, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 6, x: 8, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 6, x: 8, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 6, x: 10, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 6, x: 10, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 7, x: 1, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 8, x: 4, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 8, x: 4, color: "black", kind: ItemKind::SmallCircle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
