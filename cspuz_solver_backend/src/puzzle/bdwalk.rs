use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::bdwalk::{self, CLUE_DOWN, CLUE_UP};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (start, end, clues) = bdwalk::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = bdwalk::solve_bdwalk(start, end, &clues).ok_or("no answer")?;

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            let mut text = "".to_string();
            if (y, x) == start {
                text += "S";
            } else if (y, x) == end {
                text += "G";
            }
            if let Some(clue) = clues[y][x] {
                if clue < 0 {
                    board.push(Item::cell(y, x, "#cccccc", ItemKind::Fill));
                }
                if clue == CLUE_UP {
                    text += "▲";
                } else if clue == CLUE_DOWN {
                    text += "▼";
                } else if clue > 0 {
                    text += &clue.to_string();
                }
            }
            if !text.is_empty() {
                board.push(Item::cell(y, x, "black", ItemKind::TextString(text)));
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
