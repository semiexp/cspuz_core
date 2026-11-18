use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{check_uniqueness, Uniqueness};
use cspuz_rs_puzzles::puzzles::slitherlink;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (full, problem) = slitherlink::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = slitherlink::solve_slitherlink(full, &problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::DotGrid,
        height,
        width,
        check_uniqueness(&is_line),
    );

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if clue >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            }
        }
    }
    if let Some(is_line) = &is_line {
        board.add_grid_edges(is_line, "green", ItemKind::Wall, ItemKind::Cross);
    }

    Ok(board)
}

pub fn enumerate(url: &str, num_max_answers: usize) -> Result<(Board, Vec<Board>), &'static str> {
    let (full, problem) = slitherlink::deserialize_problem(url).ok_or("invalid url")?;
    let answer_common = slitherlink::solve_slitherlink(full, &problem).ok_or("no answer")?;
    let answers = slitherlink::enumerate_answers_slitherlink(full, &problem, num_max_answers);

    let height = problem.len();
    let width = problem[0].len();
    let mut board_common = Board::new(BoardKind::DotGrid, height, width, Uniqueness::NotApplicable);

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = problem[y][x] {
                board_common.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            }
        }
    }
    for y in 0..height {
        for x in 0..=width {
            if let Some(b) = answer_common.vertical[y][x] {
                board_common.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "black",
                    kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                })
            }
        }
    }
    for y in 0..=height {
        for x in 0..width {
            if let Some(b) = answer_common.horizontal[y][x] {
                board_common.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "black",
                    kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                })
            }
        }
    }

    let mut board_answers = vec![];
    for ans in answers {
        let mut board_answer =
            Board::new(BoardKind::Empty, height, width, Uniqueness::NotApplicable);
        // update board_answer according to ans
        for y in 0..height {
            for x in 0..=width {
                if answer_common.vertical[y][x].is_some() {
                    continue;
                }
                let b = ans.vertical[y][x];
                board_answer.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "green",
                    kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                });
            }
        }
        for y in 0..=height {
            for x in 0..width {
                if answer_common.horizontal[y][x].is_some() {
                    continue;
                }
                let b = ans.horizontal[y][x];
                board_answer.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "green",
                    kind: if b { ItemKind::Wall } else { ItemKind::Cross },
                });
            }
        }

        board_answers.push(board_answer);
    }

    Ok((board_common, board_answers))
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
            solve("https://puzz.link/p?slither/4/4/dgdh2c71"),
            Board {
                kind: BoardKind::DotGrid,
                height: 4,
                width: 4,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 0, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Wall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 0, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 0, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 0, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Wall },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Wall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Wall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
