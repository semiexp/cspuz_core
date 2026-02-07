use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{Uniqueness, UniquenessCheckable};
use cspuz_rs_puzzles::puzzles::slicy;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let borders = slicy::deserialize_problem(url).ok_or("invalid url")?;
    let ans = slicy::solve_slicy(&borders);

    let (a, b, c, d) = borders.dims;
    let mut board = Board::new(
        BoardKind::Empty,
        (a + c - 1) * 2,
        a + b * 2 + d - 2,
        ans.as_ref()
            .map_or(Uniqueness::NoAnswer, |a| if a.flatten().to_vec().is_unique() { Uniqueness::Unique } else { Uniqueness::NonUnique }),
    );

    // Use borders.to_right to get cells, as it's available whether or not there's a solution
    for &(y, x) in borders.to_right.cells() {
        let ty = y * 2;
        let tx = if y >= a - 1 {
            x * 2 - (y - (a - 1))
        } else {
            x * 2 + (a - 1 - y)
        };

        // Render cell fills
        for dy in 0..2 {
            for dx in 0..2 {
                if let Some(ref ans) = ans {
                    // If we have a solution, render based on the solution
                    if let Some(b) = ans[(y, x)] {
                        board.push(Item {
                            y: ty * 2 + 1 + dy * 2,
                            x: tx * 2 + 1 + dx * 2,
                            color: if b { "green" } else { "#cccccc" },
                            kind: ItemKind::Fill,
                        });
                    }
                } else {
                    // If no solution, render all cells as gray
                    board.push(Item {
                        y: ty * 2 + 1 + dy * 2,
                        x: tx * 2 + 1 + dx * 2,
                        color: "white",
                        kind: ItemKind::Fill,
                    });
                }
            }
        }
    }

    // Render walls - always shown whether or not there's a solution
    for &(y, x) in borders.to_right.cells() {
        let ty = y * 2;
        let tx = if y >= a - 1 {
            x * 2 - (y - (a - 1))
        } else {
            x * 2 + (a - 1 - y)
        };

        let is_valid_coord_offset = |coord: (usize, usize), offset: (i32, i32)| -> bool {
            if let Some(ref ans) = ans {
                ans.is_valid_coord_offset(coord, offset)
            } else {
                borders.to_right.is_valid_coord_offset(coord, offset)
            }
        };

        board.push(Item {
            y: ty * 2,
            x: tx * 2 + 1,
            color: "black",
            kind: if !is_valid_coord_offset((y, x), (-1, -1))
                || borders.to_bottom_right[(y - 1, x - 1)]
            {
                ItemKind::BoldWall
            } else {
                ItemKind::DottedWall
            },
        });
        board.push(Item {
            y: ty * 2 + 4,
            x: tx * 2 + 3,
            color: "black",
            kind: if !is_valid_coord_offset((y, x), (1, 1)) || borders.to_bottom_right[(y, x)] {
                ItemKind::BoldWall
            } else {
                ItemKind::DottedWall
            },
        });
        board.push(Item {
            y: ty * 2,
            x: tx * 2 + 3,
            color: "black",
            kind: if !is_valid_coord_offset((y, x), (-1, 0)) || borders.to_bottom_left[(y - 1, x)] {
                ItemKind::BoldWall
            } else {
                ItemKind::DottedWall
            },
        });
        board.push(Item {
            y: ty * 2 + 4,
            x: tx * 2 + 1,
            color: "black",
            kind: if !is_valid_coord_offset((y, x), (1, 0)) || borders.to_bottom_left[(y, x)] {
                ItemKind::BoldWall
            } else {
                ItemKind::DottedWall
            },
        });
        for t in [1, 3] {
            board.push(Item {
                y: ty * 2 + t,
                x: tx * 2,
                color: "black",
                kind: if !is_valid_coord_offset((y, x), (0, -1)) || borders.to_right[(y, x - 1)] {
                    ItemKind::BoldWall
                } else {
                    ItemKind::DottedWall
                },
            });
            board.push(Item {
                y: ty * 2 + t,
                x: tx * 2 + 4,
                color: "black",
                kind: if !is_valid_coord_offset((y, x), (0, 1)) || borders.to_right[(y, x)] {
                    ItemKind::BoldWall
                } else {
                    ItemKind::DottedWall
                },
            });
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
            solve("https://pedros.works/paper-puzzle-player?W=3x4x5&SIE=4REUEUEUEU25UEULWLULU6RDREUERE5EUERER&G=slicy"),
            Board {
                kind: BoardKind::Empty,
                height: 16,
                width: 13,
                data: vec![
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 17, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 19, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 17, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 19, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 17, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 17, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 19, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 21, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 19, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 21, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 17, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 19, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 17, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 11, x: 19, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 15, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 15, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 13, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 15, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 15, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 13, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 13, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 15, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 15, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 13, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 17, color: "green", kind: ItemKind::Fill },
                    Item { y: 15, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 15, x: 17, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 19, color: "green", kind: ItemKind::Fill },
                    Item { y: 13, x: 21, color: "green", kind: ItemKind::Fill },
                    Item { y: 15, x: 19, color: "green", kind: ItemKind::Fill },
                    Item { y: 15, x: 21, color: "green", kind: ItemKind::Fill },
                    Item { y: 17, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 17, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 19, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 19, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 17, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 17, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 19, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 19, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 17, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 17, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 19, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 19, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 17, x: 17, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 17, x: 19, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 19, x: 17, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 19, x: 19, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 17, x: 21, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 17, x: 23, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 19, x: 21, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 19, x: 23, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 21, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 21, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 23, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 23, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 21, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 21, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 23, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 23, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 21, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 21, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 23, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 23, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 21, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 21, x: 17, color: "green", kind: ItemKind::Fill },
                    Item { y: 23, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 23, x: 17, color: "green", kind: ItemKind::Fill },
                    Item { y: 25, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 25, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 27, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 27, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 25, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 25, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 27, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 27, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 29, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 29, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 31, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 31, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 29, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 29, x: 17, color: "green", kind: ItemKind::Fill },
                    Item { y: 31, x: 15, color: "green", kind: ItemKind::Fill },
                    Item { y: 31, x: 17, color: "green", kind: ItemKind::Fill },
                    Item { y: 0, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 19, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 20, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 20, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 19, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 22, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 7, x: 22, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 19, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 9, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 20, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 11, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 20, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 23, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 8, x: 23, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 21, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 9, x: 20, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 9, x: 24, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 20, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 11, x: 24, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 15, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 11, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 15, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 15, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 15, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 15, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 19, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 21, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 21, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 19, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 22, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 15, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 15, x: 22, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 12, x: 23, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 25, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 25, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 23, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 22, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 13, x: 26, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 22, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 15, x: 26, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 19, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 19, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 19, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 19, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 11, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 11, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 17, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 17, x: 12, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 19, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 19, x: 12, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 17, x: 12, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 17, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 19, x: 12, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 19, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 19, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 17, x: 20, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 19, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 19, x: 20, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 16, x: 21, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 23, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 16, x: 23, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 20, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 17, x: 24, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 19, x: 20, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 19, x: 24, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 21, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 21, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 23, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 23, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 21, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 21, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 23, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 23, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 11, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 21, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 21, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 23, x: 10, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 23, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 21, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 21, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 23, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 23, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 20, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 20, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 19, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 21, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 21, x: 22, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 23, x: 18, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 23, x: 22, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 28, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 25, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 25, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 27, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 27, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 25, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 25, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 27, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 27, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 28, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 25, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 25, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 27, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 27, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 24, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 28, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 24, x: 19, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 28, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 25, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 25, x: 20, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 27, x: 16, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 27, x: 20, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 32, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 9, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 32, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 29, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 29, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 31, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 31, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 32, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 13, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 32, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 29, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 29, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 31, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 31, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 28, x: 15, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 32, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 28, x: 17, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 32, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 29, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 29, x: 18, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 31, x: 14, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 31, x: 18, color: "black", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::NonUnique,
            },
        );
    }
}
