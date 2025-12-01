use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::keywest;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let clues = keywest::deserialize_problem(url).ok_or("invalid url")?;
    let (num, has_line) = keywest::solve_keywest(&clues).ok_or("no answer")?;
    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Empty,
        height,
        width,
        is_unique(&(&num, &has_line)),
    );

    for y in 0..height {
        for x in 0..width {
            if y < height - 1 {
                if let Some(n) = has_line.vertical[y][x] {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "green",
                        kind: if n { ItemKind::Line } else { ItemKind::Cross },
                    });
                }
            }
            if x < width - 1 {
                if let Some(n) = has_line.horizontal[y][x] {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "green",
                        kind: if n { ItemKind::Line } else { ItemKind::Cross },
                    });
                }
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            board.push(Item::cell(y, x, "white", ItemKind::FilledCircle));
            board.push(Item::cell(y, x, "black", ItemKind::Circle));
            if let Some(n) = clues[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            } else if let Some(n) = num[y][x] {
                board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
            }
        }
    }

    Ok(board)
}
