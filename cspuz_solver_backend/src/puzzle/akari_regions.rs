use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::akari_regions;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues, has_block) =
        akari_regions::deserialize_problem(url).ok_or("invalid url")?;
    let has_light = akari_regions::solve_akari_region(&borders, &clues, &has_block);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        has_light
            .as_ref()
            .map_or(Uniqueness::NoAnswer, |h| is_unique(h)),
    );

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if has_block[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Fill));
                continue;
            }
            if let Some(has_light) = &has_light {
                if let Some(b) = has_light[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b {
                            ItemKind::FilledCircle
                        } else {
                            ItemKind::Dot
                        },
                    ));
                }
            }
            if let Some(n) = clues[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::NumUpperLeft(n)));
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
            solve("https://pedros.works/paper-puzzle-player?W=6x5&L=z7z6z8&L-N=(2)3(2)1(1)15(0)4&SIE=9UL3UU9RURR1U4U5R&G=akari-regional"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::NumUpperLeft(2) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::NumUpperLeft(1) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::NumUpperLeft(2) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::NumUpperLeft(0) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::FilledCircle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
