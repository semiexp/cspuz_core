use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::kropki_pairs::{self, KropkiClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (walls, cells) = kropki_pairs::deserialize_problem(url).ok_or("invalid url")?;
    let ans = kropki_pairs::solve_kropki_pairs(&walls, &cells);

    let height = cells.len();
    let width = cells[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = cells[y][x] {
                if n > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                }
            } else if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
                }
            }
            if y < height - 1 {
                if walls.horizontal[y][x] == KropkiClue::White {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "black",
                        kind: ItemKind::SmallCircle,
                    });
                } else if walls.horizontal[y][x] == KropkiClue::Black {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "black",
                        kind: ItemKind::SmallFilledCircle,
                    });
                }
            }
            if x < width - 1 {
                if walls.vertical[y][x] == KropkiClue::White {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "black",
                        kind: ItemKind::SmallCircle,
                    });
                } else if walls.vertical[y][x] == KropkiClue::Black {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "black",
                        kind: ItemKind::SmallFilledCircle,
                    });
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
            solve("https://pedros.works/paper-puzzle-player.html?W=4x3&L=x2(3)3(1)6&L-E=w0b2b5b3w5&G=kropki-pairs"),
            Board {
                kind: BoardKind::Grid,
                height: 3,
                width: 4,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(3) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
