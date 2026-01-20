use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::the_longest;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = the_longest::deserialize_problem(url).ok_or("invalid url")?;
    let ans = the_longest::solve_the_longest(&problem).ok_or("no answer")?;

    let height = ans.vertical.len();
    let width = ans.vertical[0].len() + 1;
    let mut board = Board::new(BoardKind::DotGrid, height, width, is_unique(&ans));

    for y in 0..=height {
        for x in 0..width {
            if problem.horizontal[y][x] {
                board.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "black",
                    kind: ItemKind::BoldWall,
                });
                continue;
            }

            if y == 0 || y == height {
                board.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "green",
                    kind: ItemKind::Wall,
                });
                continue;
            }

            if let Some(b) = ans.horizontal[y - 1][x] {
                board.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "green",
                    kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                });
            }
        }
    }
    for y in 0..height {
        for x in 0..=width {
            if problem.vertical[y][x] {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::BoldWall,
                });
                continue;
            }

            if x == 0 || x == width {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "green",
                    kind: ItemKind::Wall,
                });
                continue;
            }

            if let Some(b) = ans.vertical[y][x - 1] {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "green",
                    kind: if b { ItemKind::Wall } else { ItemKind::Cross },
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
            solve("https://pedros.works/paper-puzzle-player?W=5x4&SIE=0RRR2UU7RRR2UU15UUU&G=the-longest"),
            Board {
                kind: BoardKind::DotGrid,
                height: 4,
                width: 5,
                data: vec![
                    Item { y: 0, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 0, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 9, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Wall },
                    Item { y: 1, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Wall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Wall },
                    Item { y: 1, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Wall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
