use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs_puzzles::puzzles::numlin;

pub fn enumerate(url: &str, num_max_answers: usize) -> Result<(Board, Vec<Board>), &'static str> {
    let problem = numlin::deserialize_problem(url).ok_or("invalid url")?;
    let answers = numlin::enumerate_answers_numlin(&problem, num_max_answers);

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
            }
        }
    }

    let mut boards = vec![];
    for ans in answers {
        let mut board_answer =
            Board::new(BoardKind::Empty, height, width, Uniqueness::NotApplicable);
        for y in 0..height {
            for x in 0..width {
                if x < width - 1 && ans.horizontal[y][x] {
                    board_answer.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "green",
                        kind: ItemKind::Line,
                    });
                }
                if y < height - 1 && ans.vertical[y][x] {
                    board_answer.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "green",
                        kind: ItemKind::Line,
                    });
                }
            }
        }
        boards.push(board_answer);
    }

    Ok((board_common, boards))
}
