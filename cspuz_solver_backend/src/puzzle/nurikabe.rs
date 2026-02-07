use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{check_uniqueness, Uniqueness};
use cspuz_rs_puzzles::puzzles::nurikabe;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = nurikabe::deserialize_problem(url).ok_or("invalid url")?;
    let ans = nurikabe::solve_nurikabe(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some(ans) = &ans {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if a { ItemKind::Block } else { ItemKind::Dot },
                    ));
                }
            }
        }
    }

    Ok(board)
}

pub fn enumerate(url: &str, num_max_answers: usize) -> Result<(Board, Vec<Board>), &'static str> {
    let problem = nurikabe::deserialize_problem(url).ok_or("invalid url")?;
    let ans_common = nurikabe::solve_nurikabe(&problem).ok_or("no answer")?;
    let answers = nurikabe::enumerate_answers_nurikabe(&problem, num_max_answers);

    let height = problem.len();
    let width = problem[0].len();
    let mut board_common = Board::new(BoardKind::Grid, height, width, Uniqueness::NotApplicable);

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if clue > 0 {
                    board_common.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board_common.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            } else if let Some(a) = ans_common[y][x] {
                board_common.push(Item::cell(
                    y,
                    x,
                    "green",
                    if a { ItemKind::Block } else { ItemKind::Dot },
                ));
            }
        }
    }

    let mut boards = vec![];
    for ans in answers {
        let mut board_answer =
            Board::new(BoardKind::Empty, height, width, Uniqueness::NotApplicable);
        for y in 0..height {
            for x in 0..width {
                if ans_common[y][x].is_some() {
                    continue;
                }
                if let Some(clue) = problem[y][x] {
                    if clue > 0 {
                        board_answer.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                    } else {
                        board_answer.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                    }
                } else {
                    let a = ans[y][x];
                    board_answer.push(Item::cell(
                        y,
                        x,
                        "green",
                        if a { ItemKind::Block } else { ItemKind::Dot },
                    ));
                }
            }
        }
        boards.push(board_answer);
    }

    Ok((board_common, boards))
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
            solve("https://puzz.link/p?nurikabe/6/6/m8n8i9u"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(8) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Num(8) },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Num(9) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::NonUnique,
            },
        );
    }
}
