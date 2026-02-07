use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{Uniqueness, UniquenessCheckable};
use cspuz_rs_puzzles::puzzles::cross_border_parity_loop::{self, CBPLCell};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (cells, clues_black, clues_white, borders) =
        cross_border_parity_loop::deserialize_problem(url).ok_or("invalid url")?;
    let result = cross_border_parity_loop::solve_cross_border_parity_loop(
        &cells,
        &clues_black,
        &clues_white,
        &borders,
    );

    let height = cells.len();
    let width = cells[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        result
            .as_ref()
            .map(|(is_line, cell_state)| if (is_line, cell_state).is_unique() { Uniqueness::Unique } else { Uniqueness::NonUnique })
            .unwrap_or(Uniqueness::NoAnswer),
    );

    let mut is_skip = vec![vec![false; width]; height];
    for y in 0..height {
        for x in 0..width {
            if cells[y][x] == CBPLCell::Blocked {
                is_skip[y][x] = true;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(ref result) = result {
                if let Some(n) = result.1[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        match n {
                            0 => "#cccccc",
                            1 => "#ffcccc",
                            2 => "#ccccff",
                            _ => unreachable!(),
                        },
                        ItemKind::Fill,
                    ));
                }
            }

            match cells[y][x] {
                CBPLCell::Empty => (),
                CBPLCell::Blocked => board.push(Item::cell(y, x, "black", ItemKind::Fill)),
                CBPLCell::BlackCircle => {
                    board.push(Item::cell(y, x, "#ff0000", ItemKind::SmallFilledCircle))
                }
                CBPLCell::WhiteCircle => {
                    board.push(Item::cell(y, x, "#0000ff", ItemKind::SmallCircle))
                }
            }

            if let Some(n) = clues_black[y][x] {
                board.push(Item::cell(y, x, "#ff0000", ItemKind::NumLowerRight(n)));
            }
            if let Some(n) = clues_white[y][x] {
                board.push(Item::cell(y, x, "#0000ff", ItemKind::NumUpperLeft(n)));
            }
        }
    }

    board.add_borders(&borders, "black");

    if let Some((is_line, _)) = result {
        board.add_lines_irrefutable_facts(&is_line, "green", Some(&is_skip));
    }

    Ok(board)
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
            solve("https://pedros.works/paper-puzzle-player?W=6x5&LI-N=(1)8(3)8&LI-S=(2)2(1)14&L=x4w4x4w2b6w3b3&SIE=3RRUU9UU8RRR4UUUU1RR10DLU&G=cross-border-parity-loop"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 1, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "#0000ff", kind: ItemKind::SmallCircle },
                    Item { y: 1, x: 7, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "#ccccff", kind: ItemKind::Fill },
                    Item { y: 1, x: 11, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "#ccccff", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "#0000ff", kind: ItemKind::SmallCircle },
                    Item { y: 3, x: 3, color: "#0000ff", kind: ItemKind::NumUpperLeft(1) },
                    Item { y: 3, x: 5, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "#0000ff", kind: ItemKind::SmallCircle },
                    Item { y: 3, x: 11, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "#ff0000", kind: ItemKind::NumLowerRight(2) },
                    Item { y: 5, x: 3, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "#ccccff", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 11, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "#ccccff", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "#ff0000", kind: ItemKind::NumLowerRight(1) },
                    Item { y: 7, x: 7, color: "#0000ff", kind: ItemKind::NumUpperLeft(3) },
                    Item { y: 7, x: 9, color: "#ccccff", kind: ItemKind::Fill },
                    Item { y: 7, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 11, color: "#ff0000", kind: ItemKind::SmallFilledCircle },
                    Item { y: 9, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "#ccccff", kind: ItemKind::Fill },
                    Item { y: 9, x: 7, color: "#ccccff", kind: ItemKind::Fill },
                    Item { y: 9, x: 9, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 9, color: "#ff0000", kind: ItemKind::SmallFilledCircle },
                    Item { y: 9, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
