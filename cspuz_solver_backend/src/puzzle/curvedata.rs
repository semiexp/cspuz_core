use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::curvedata;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (piece_id, borders, pieces) = curvedata::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = curvedata::solve_curvedata(&piece_id, &borders, &pieces).ok_or("no answer")?;

    let height = piece_id.len();
    let width = piece_id[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    if let Some(borders) = borders {
        board.add_borders(&borders, "black");
    }

    for y in 0..height {
        for x in 0..width {
            match piece_id[y][x] {
                curvedata::PieceId::None => (),
                curvedata::PieceId::Block => {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                }
                curvedata::PieceId::Piece(_) => {
                    board.push(Item::cell(y, x, "black", ItemKind::Circle));
                }
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}

pub fn enumerate(url: &str, num_max_answers: usize) -> Result<(Board, Vec<Board>), &'static str> {
    let (piece_id, borders, pieces) = curvedata::deserialize_problem(url).ok_or("invalid url")?;
    let is_line_common =
        curvedata::solve_curvedata(&piece_id, &borders, &pieces).ok_or("no answer")?;
    let answers =
        curvedata::enumerate_answers_curvedata(&piece_id, &borders, &pieces, num_max_answers);

    let height = piece_id.len();
    let width = piece_id[0].len();
    let mut board_common = Board::new(BoardKind::Grid, height, width, Uniqueness::NotApplicable);

    if let Some(borders) = borders {
        board_common.add_borders(&borders, "black");
    }

    for y in 0..height {
        for x in 0..width {
            match piece_id[y][x] {
                curvedata::PieceId::None => (),
                curvedata::PieceId::Block => {
                    board_common.push(Item::cell(y, x, "black", ItemKind::Fill));
                }
                curvedata::PieceId::Piece(_) => {
                    board_common.push(Item::cell(y, x, "black", ItemKind::Circle));
                }
            }
        }
    }
    board_common.add_lines_irrefutable_facts(&is_line_common, "green", None);

    let mut board_answers = vec![];
    for ans in answers {
        let mut board_answer =
            Board::new(BoardKind::Empty, height, width, Uniqueness::NotApplicable);
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 && is_line_common.vertical[y][x].is_none() {
                    board_answer.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "#cccccc",
                        kind: if ans.vertical[y][x] {
                            ItemKind::Line
                        } else {
                            ItemKind::Cross
                        },
                    });
                }
                if x < width - 1 && is_line_common.horizontal[y][x].is_none() {
                    board_answer.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "#cccccc",
                        kind: if ans.horizontal[y][x] {
                            ItemKind::Line
                        } else {
                            ItemKind::Cross
                        },
                    });
                }
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
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("https://puzz.link/p?curvedata/4/5/=n01o/b0100000/3/3/ec24/2/3/ba1"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 4,
                data: vec![
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
