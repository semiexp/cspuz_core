use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::pyramid;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (is_shaded, clues, min_value, max_value) =
        pyramid::deserialize_problem(url).ok_or("invalid url")?;
    let ans =
        pyramid::solve_pyramid(&is_shaded, &clues, min_value, max_value).ok_or("no answer")?;
    let size = ans.len();

    let mut board = Board::new(BoardKind::Empty, size, size * 2, is_unique(&ans.concat()));

    // Fills
    for y in 0..size {
        if is_shaded[y] {
            for x in 0..=y {
                board.push(Item::cell(
                    y,
                    size - y - 1 + 2 * x,
                    "#cccccc",
                    ItemKind::Fill,
                ));
                board.push(Item::cell(
                    y,
                    size - y - 1 + 2 * x + 1,
                    "#cccccc",
                    ItemKind::Fill,
                ));
            }
        }
    }

    // Borders
    for y in 0..=size {
        let start = if y == size { 0 } else { size - y - 1 };
        let end = if y == size { size * 2 } else { size + y + 1 };

        for x in start..end {
            board.push(Item {
                y: y * 2,
                x: x * 2 + 1,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }
    for y in 0..size {
        for x in 0..=(y + 1) {
            board.push(Item {
                y: y * 2 + 1,
                x: (size - y - 1 + 2 * x) * 2,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }

    for y in 0..size {
        for x in 0..=y {
            if let Some(n) = clues[y][x] {
                board.push(Item {
                    y: 2 * y + 1,
                    x: (size - y - 1 + 2 * x + 1) * 2,
                    color: "black",
                    kind: ItemKind::Num(n),
                });
            } else if let Some(n) = ans[y][x] {
                board.push(Item {
                    y: 2 * y + 1,
                    x: (size - y - 1 + 2 * x + 1) * 2,
                    color: "green",
                    kind: ItemKind::Num(n),
                });
            }
        }
    }
    Ok(board)
}
