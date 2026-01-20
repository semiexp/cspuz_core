use crate::board::{Board, BoardKind, Item, ItemKind};
use cspuz_rs_puzzles::puzzles::numberlink;

pub fn enumerate_answers_numberlink(
    url: &str,
    num_max_answers: usize,
) -> Result<(Board, Vec<Board>), &'static str> {
    let clues = numberlink::deserialize_problem(url).ok_or("invalid url")?;
    let answers = numberlink::enumerate_answers_numberlink(&clues, num_max_answers);
    if answers.len() == 0 {
        return Err("no answer");
    }

    let height = clues.len();
    let width = clues[0].len();

    let mut board_common = Board::new(BoardKind::Grid, height, width);
    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                board_common.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            }
        }
    }

    let mut board_answers = vec![];
    for ans in answers {
        let mut board_answer = Board::new(BoardKind::Empty, height, width);
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 && ans.vertical[y][x] {
                    board_answer.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "green",
                        kind: ItemKind::Line,
                    });
                }
                if x < width - 1 && ans.horizontal[y][x] {
                    board_answer.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "green",
                        kind: ItemKind::Line,
                    });
                }
            }
        }
        board_answers.push(board_answer);
    }

    Ok((board_common, board_answers))
}
