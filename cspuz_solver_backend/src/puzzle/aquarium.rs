use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::aquarium;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (region_aware, (borders, (clues_up, clues_left))) =
        aquarium::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = aquarium::solve_aquarium(region_aware, &borders, &clues_up, &clues_left);

    let height = clues_left.len();
    let width = clues_up.len();

    let mut board = Board::new(
        BoardKind::Empty,
        height + 1,
        width + 1,
        is_black.as_ref().map_or(Uniqueness::NoAnswer, |b| is_unique(b)),
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
            if let Some(is_black) = &is_black {
                if is_black[y][x] == Some(true) {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Fill));
                } else if is_black[y][x] == Some(false) {
                    board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Dot));
                }
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
            solve("https://puzz.link/p?aquarium/5/4/9lqgfk4/3h12h13"),
            Board {
                kind: BoardKind::Empty,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::Wall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::Wall },
                    Item { y: 10, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::Wall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
