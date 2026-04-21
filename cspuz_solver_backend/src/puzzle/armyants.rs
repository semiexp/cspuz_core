use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::armyants;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = armyants::deserialize_problem(url).ok_or("invalid url")?;
    let ans = armyants::solve_armyants(&borders, &clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    board.add_borders(&borders, "black");

    if let Some((movement, final_state)) = &ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(clue) = clues[y][x] {
                    if clue > 0 {
                        board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                    } else {
                        board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                    }
                } else if let Some(n) = final_state[y][x] {
                    if n > -1 {
                        board.push(Item::cell(y, x, "green", ItemKind::Num(n)));
                    }
                }
            }
        }
        board.add_lines_irrefutable_facts(movement, "green", None)
    } else {
        for y in 0..height {
            for x in 0..width {
                if let Some(clue) = clues[y][x] {
                    if clue > 0 {
                        board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                    } else {
                        board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                    }
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
            solve("https://puzz.link/p?armyants/4/4/m38dtgh1g2h43h1g2h"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 4,
                data: vec![
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
