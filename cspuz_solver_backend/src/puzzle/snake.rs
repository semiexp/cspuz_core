use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::snake;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use snake::SnakeClue;

    let (circles, (clues_up, clues_left)) = snake::deserialize_problem(url).ok_or("invalid url")?;
    let ans = snake::solve_snake(&circles, &clues_up, &clues_left);

    let height = clues_left.len();
    let width = clues_up.len();

    let mut board = Board::new(
        BoardKind::Empty,
        height + 1,
        width + 1,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        if let Some(n) = clues_left[y] {
            board.push(Item::cell(y + 1, 0, "black", ItemKind::Num(n)));
        }
    }
    for x in 0..width {
        if let Some(n) = clues_up[x] {
            board.push(Item::cell(0, x + 1, "black", ItemKind::Num(n)));
        }
    }

    board.add_grid(1, 1, height, width);

    if let Some(ans) = &ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y + 1,
                        x + 1,
                        "green",
                        if a { ItemKind::Block } else { ItemKind::Dot },
                    ));
                }
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            match circles[y][x] {
                SnakeClue::None => {}
                SnakeClue::White => board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Circle)),

                SnakeClue::Black => {
                    board.push(Item::cell(y + 1, x + 1, "black", ItemKind::FilledCircle))
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
            solve("https://puzz.link/p?snake/2/2/3i1g2g"),
            Board {
                kind: BoardKind::Empty,
                height: 3,
                width: 3,
                data: vec![
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::Wall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::FilledCircle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
