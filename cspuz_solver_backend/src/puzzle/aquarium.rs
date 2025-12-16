use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::aquarium;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (region_aware, (borders, (clues_up, clues_left))) =
        aquarium::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = aquarium::solve_aquarium(region_aware, &borders, &clues_up, &clues_left)
        .ok_or("no answer")?;

    let height = clues_left.len();
    let width = clues_up.len();

    let mut board = Board::new(
        BoardKind::Empty,
        height + 1,
        width + 1,
        is_unique(&is_black),
    );

    for y in 0..height {
        if let Some(n) = clues_left[y] {
            board.push(Item::cell(y + 1, 0, "black", ItemKind::Num(n)));
        }
    }
    for x in 0..width {
        if let Some(n) = clues_up[x] {
            board.push(Item::cell(0, x + 1, "black", ItemKind::Num(n)));
        }
    }

    for y in 0..height {
        for x in 0..width {
            if is_black[y][x] == Some(true) {
                board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Fill));
            } else if is_black[y][x] == Some(false) {
                board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Dot));
            }
        }
    }

    board.add_grid(1, 1, height, width);

    for y in 0..height {
        for x in 0..width {
            if y < height - 1 && borders.horizontal[y][x] {
                board.push(Item {
                    y: 2 * y + 4,
                    x: 2 * x + 3,
                    color: "black",
                    kind: ItemKind::BoldWall,
                });
            }
            if x < width - 1 && borders.vertical[y][x] {
                board.push(Item {
                    y: 2 * y + 3,
                    x: 2 * x + 4,
                    color: "black",
                    kind: ItemKind::BoldWall,
                });
            }
        }
    }

    Ok(board)
}
