use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::crosswall;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = crosswall::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = crosswall::solve_crosswall(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::DotGrid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            if let Some((size, level)) = problem[y][x] {
                if size > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(size)));
                }
                if level >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::NumLowerRight(level)));
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..=width {
            if let Some(b) = is_line.vertical[y][x] {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "green",
                    kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                })
            }
        }
    }
    for y in 0..=height {
        for x in 0..width {
            if let Some(b) = is_line.horizontal[y][x] {
                board.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "green",
                    kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                })
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
            solve("https://pedros.works/paper-puzzle-player?W=5x5&L-N=(2)6(3)1(4)2(4)1(1)1(6)11&L-S=(0)9(2)3(2)6&G=crosswall"),
            Board {
                kind: BoardKind::DotGrid,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::NumLowerRight(0) },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::NumLowerRight(2) },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::NumLowerRight(2) },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Num(6) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Wall },
                    Item { y: 9, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 0, x: 9, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Wall },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
