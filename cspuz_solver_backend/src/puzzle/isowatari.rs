use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::isowatari::{self, IsowatariClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (size, problem, holes) = isowatari::deserialize_problem(url).ok_or("invalid url")?;
    let ans = isowatari::solve_isowatari(size, &problem, &holes);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));
    if let Some(ans) = &ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(is_hole) = &holes {
                    if is_hole[y][x] {
                        board.push(Item::cell(y, x, "#424242", ItemKind::Fill));
                        continue;
                    }
                }
                match problem[y][x] {
                    IsowatariClue::None => {
                        if let Some(a) = ans[y][x] {
                            board.push(Item::cell(
                                y,
                                x,
                                "green",
                                if a > 0 {
                                    ItemKind::Block
                                } else {
                                    ItemKind::Dot
                                },
                            ));
                        }
                    }
                    IsowatariClue::White => board.push(Item::cell(y, x, "black", ItemKind::Circle)),
                    IsowatariClue::Black => {
                        board.push(Item::cell(y, x, "black", ItemKind::FilledCircle))
                    }
                }
            }
        }
    } else {
        for y in 0..height {
            for x in 0..width {
                if let Some(is_hole) = &holes {
                    if is_hole[y][x] {
                        board.push(Item::cell(y, x, "#424242", ItemKind::Fill));
                        continue;
                    }
                }
                match problem[y][x] {
                    IsowatariClue::None => {}
                    IsowatariClue::White => board.push(Item::cell(y, x, "black", ItemKind::Circle)),
                    IsowatariClue::Black => {
                        board.push(Item::cell(y, x, "black", ItemKind::FilledCircle))
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
            solve("https://pzprxs.vercel.app/p?isowatari/3/3/1/90640"),
            Board {
                kind: BoardKind::Grid,
                height: 3,
                width: 3,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "#424242", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
