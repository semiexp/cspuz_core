use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::context;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let clues = context::deserialize_problem(url).ok_or("invalid url")?;
    let is_black = context::solve_context(&clues);

    let height = is_black.as_ref().map_or(clues.len(), |b| b.len());
    let width = is_black.as_ref().map_or(clues[0].len(), |b| b[0].len());
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&is_black));

    for y in 0..height {
        for x in 0..width {
            if let Some(is_black) = &is_black {
                if let Some(b) = is_black[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b { ItemKind::Block } else { ItemKind::Dot },
                    ));
                }
            }
            if let Some(clue) = clues[y][x] {
                if clue >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
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
            solve("https://puzz.link/p?context/3/3/02h1h1g"),
            Board {
                kind: BoardKind::Grid,
                height: 3,
                width: 3,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Block },                    
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
            
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
