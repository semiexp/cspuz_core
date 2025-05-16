use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::creek;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = creek::deserialize_problem(url).ok_or("invalid url")?;
    let ans = creek::solve_creek(&problem).ok_or("no answer")?;

    let height = ans.len();
    let width = ans[0].len();
    let mut board = Board::new(BoardKind::Empty, height, width, is_unique(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some(a) = ans[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if a { ItemKind::Fill } else { ItemKind::Dot },
                ));
            }
        }
    }
    for y in 0..=height {
        for x in 0..=width {
            if y < height {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::Wall,
                });
            }
            if x < width {
                board.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "black",
                    kind: ItemKind::Wall,
                });
            }
        }
    }
    for y in 0..=height {
        for x in 0..=width {
            if let Some(n) = problem[y][x] {
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "white",
                    kind: ItemKind::FilledCircle,
                });
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::Circle,
                });
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::Num(n),
                });
            }
        }
    }

    Ok(board)
}
