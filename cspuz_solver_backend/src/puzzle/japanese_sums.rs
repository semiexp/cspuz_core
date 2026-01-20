use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::japanese_sums;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (k, (vertical, horizontal), given_numbers) =
        japanese_sums::deserialize_problem(url).ok_or("invalid url")?;
    let num = japanese_sums::solve_japanese_sums(k, &vertical, &horizontal, &given_numbers)
        .ok_or("no answer")?;

    let height = num.len();
    let width = num[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&num));

    for y in 0..height {
        for x in 0..width {
            if let Some(given_numbers) = &given_numbers {
                if let Some(n) = given_numbers[y][x] {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                    continue;
                }
            }
            if let Some(n) = num[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if n == 0 {
                        ItemKind::Block
                    } else {
                        ItemKind::Num(n)
                    },
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
            solve("https://pzprxs.vercel.app/p?japanesesums/6/5/4/...ah5.j.1g.4g352...4.j.8go4z"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Block },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
