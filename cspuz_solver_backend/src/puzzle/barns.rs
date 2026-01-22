use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::barns;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (icebarn, borders) = barns::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = barns::solve_barns(&icebarn, &borders);

    let height = icebarn.len();
    let width = icebarn[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_line.as_ref().map_or(Uniqueness::NoAnswer, |x| is_unique(x)));

    board.add_borders(&borders, "black");
    for y in 0..height {
        for x in 0..width {
            if icebarn[y][x] {
                board.push(Item::cell(y, x, "#e0e0ff", ItemKind::Fill));
            }
        }
    }
    if let Some(is_line) = &is_line {
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 {
                    if !borders.horizontal[y][x] {
                        if let Some(b) = is_line.vertical[y][x] {
                            board.push(Item {
                                y: y * 2 + 2,
                                x: x * 2 + 1,
                                color: "green",
                                kind: if b { ItemKind::Line } else { ItemKind::Cross },
                            });
                        }
                    }
                }
                if x < width - 1 {
                    if !borders.vertical[y][x] {
                        if let Some(b) = is_line.horizontal[y][x] {
                            board.push(Item {
                                y: y * 2 + 1,
                                x: x * 2 + 2,
                                color: "green",
                                kind: if b { ItemKind::Line } else { ItemKind::Cross },
                            });
                        }
                    }
                }
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
            solve("https://puzz.link/p?barns/5/6/0gce000g00000g00"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 5,
                data: vec![
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 1, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
