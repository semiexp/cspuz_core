use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::guidearrow::{self, GuidearrowClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (ty, tx, clues) = guidearrow::deserialize_problem(url).ok_or("invalid url")?;
    let ans = guidearrow::solve_guidearrow(ty, tx, &clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        ans.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );
    board.push(Item::cell(ty, tx, "black", ItemKind::Circle));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = clues[y][x] {
                let kind = match clue {
                    GuidearrowClue::Up => ItemKind::ArrowUp,
                    GuidearrowClue::Down => ItemKind::ArrowDown,
                    GuidearrowClue::Left => ItemKind::ArrowLeft,
                    GuidearrowClue::Right => ItemKind::ArrowRight,
                    GuidearrowClue::Unknown => ItemKind::Text("?"),
                };
                board.push(Item::cell(y, x, "black", kind));
            }
        }
    }
    if let Some(ans) = &ans {
        for y in 0..height {
            for x in 0..width {
                if clues[y][x].is_some() || (y, x) == (ty, tx) {
                    continue;
                }
                if let Some(b) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b { ItemKind::Fill } else { ItemKind::Dot },
                    ));
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
            solve("https://puzz.link/p?guidearrow/7/6/31kecsdl.n"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 7,
                data: vec![
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::ArrowRight },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::ArrowDown },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::ArrowLeft },
                    Item { y: 9, x: 11, color: "black", kind: ItemKind::Text("?") },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 13, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Fill },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
