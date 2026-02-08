use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::loop_special::{self, LoopSpecialClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = loop_special::deserialize_problem(url).ok_or("invalid url")?;
    let ans = loop_special::solve_loop_special(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    if let Some(is_line) = &ans {
        board.add_lines_irrefutable_facts(is_line, "green", None);
    }

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                LoopSpecialClue::Num(n) => {
                    board.push(Item::cell(y, x, "black", ItemKind::Circle));
                    if n > 0 {
                        board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                    }
                }
                LoopSpecialClue::Empty => (),
                _ => {
                    let (up, down, left, right) = match problem[y][x] {
                        LoopSpecialClue::Cross => (true, true, true, true),
                        LoopSpecialClue::Vertical => (true, true, false, false),
                        LoopSpecialClue::Horizontal => (false, false, true, true),
                        LoopSpecialClue::UpRight => (true, false, false, true),
                        LoopSpecialClue::UpLeft => (true, false, true, false),
                        LoopSpecialClue::DownLeft => (false, true, true, false),
                        LoopSpecialClue::DownRight => (false, true, false, true),
                        _ => unreachable!(),
                    };
                    if up {
                        board.push(Item {
                            y: y * 2,
                            x: x * 2 + 1,
                            color: "black",
                            kind: ItemKind::Line,
                        });
                    }
                    if down {
                        board.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color: "black",
                            kind: ItemKind::Line,
                        });
                    }
                    if left {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2,
                            color: "black",
                            kind: ItemKind::Line,
                        });
                    }
                    if right {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 2,
                            color: "black",
                            kind: ItemKind::Line,
                        });
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
            solve("https://puzz.link/p?loopsp/6/7/1n2tln2qhomv1oku"),
            Board {
                kind: BoardKind::Grid,
                height: 7,
                width: 6,
                data: vec![
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::Line },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::Line },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Line },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 10, x: 7, color: "black", kind: ItemKind::Line },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
