use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs_puzzles::puzzles::easyasabc;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (range, (clues_up, clues_down, clues_left, clues_right, cells)) =
        easyasabc::deserialize_problem(url).ok_or("invalid url")?;
    let ans: Option<Vec<Vec<Option<i32>>>> = easyasabc::solve_easyasabc(
        range,
        &clues_up,
        &clues_down,
        &clues_left,
        &clues_right,
        cells,
    );

    let height = clues_left.len();
    let width = clues_up.len();

    let mut is_unique = Uniqueness::Unique;
    if let Some(answer) = &ans {
        for y in 0..height {
            for x in 0..width {
                if answer[y][x].is_none() || answer[y][x] == Some(-1) {
                    is_unique = Uniqueness::NonUnique;
                }
            }
        }
    } else {
        is_unique = Uniqueness::NoAnswer;
    }
    let mut board = Board::new(BoardKind::Empty, height + 2, width + 2, is_unique);

    for y in 0..height {
        if let Some(n) = clues_left[y] {
            board.push(Item::cell(y + 1, 0, "black", ItemKind::Num(n)));
        }
        if let Some(n) = clues_right[y] {
            board.push(Item::cell(y + 1, width + 1, "black", ItemKind::Num(n)));
        }
    }
    for x in 0..width {
        if let Some(n) = clues_up[x] {
            board.push(Item::cell(0, x + 1, "black", ItemKind::Num(n)));
        }
        if let Some(n) = clues_down[x] {
            board.push(Item::cell(height + 1, x + 1, "black", ItemKind::Num(n)));
        }
    }

    board.add_grid(1, 1, height, width);

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if (1..=26).contains(&clue) {
                    let p = (clue - 1) as usize;
                    board.push(Item::cell(
                        y + 1,
                        x + 1,
                        "black",
                        ItemKind::Text(&ALPHA[p..=p]),
                    ));
                } else {
                    board.push(Item::cell(y + 1, x + 1, "black", ItemKind::Num(clue - 26)));
                }
            } else if let Some(ans) = &ans {
                if let Some(a) = ans[y][x] {
                    if a == -1 {
                        board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Dot));
                    } else if (1..=26).contains(&a) {
                        let p = (a - 1) as usize;
                        board.push(Item::cell(
                            y + 1,
                            x + 1,
                            "green",
                            ItemKind::Text(&ALPHA[p..=p]),
                        ));
                    } else {
                        board.push(Item::cell(y + 1, x + 1, "green", ItemKind::Num(clue - 26)));
                    }
                }
            }
        }
    }

    Ok(board)
}
