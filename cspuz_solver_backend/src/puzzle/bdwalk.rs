use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::bdwalk::{self, CLUE_DOWN, CLUE_UP};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (start, end, clues) = bdwalk::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = bdwalk::solve_bdwalk(start, end, &clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        is_line.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );

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

    if let Some(is_line) = &is_line {
        board.add_lines_irrefutable_facts(is_line, "green", None);
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
            solve("https://puzz.link/p?bdwalk/m/5/4/1244h1h442.k30g0h"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 5,
                data: vec![
                    Item { y: 1, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::TextString("▼".to_string()) },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::TextString("S3".to_string()) },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::TextString("3".to_string()) },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::TextString("1".to_string()) },
                    Item { y: 3, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::TextString("2".to_string()) },
                    Item { y: 7, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::TextString("▲".to_string()) },
                    Item { y: 7, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::TextString("▲".to_string()) },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::TextString("G".to_string()) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
