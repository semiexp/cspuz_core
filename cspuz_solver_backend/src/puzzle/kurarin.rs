use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::kurarin;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = kurarin::deserialize_problem(url).ok_or("invalid url")?;
    let (is_line, is_black) = kurarin::solve_kurarin(&problem).ok_or("no answer")?;

    let height = (problem.len() + 1) / 2;
    let width = (problem[0].len() + 1) / 2;
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        is_unique(&(&is_line, &is_black)),
    );

    for y in 0..height {
        for x in 0..width {
            match is_black[y][x] {
                Some(true) => {
                    board.push(Item::cell(y, x, "green", ItemKind::Fill));
                }
                Some(false) => {
                    board.push(Item::cell(y, x, "green", ItemKind::Dot));
                }
                None => {}
            }
        }
    }

    for y in 0..height {
        for x in 0..(width - 1) {
            board.push(Item {
                y: y * 2 + 1,
                x: x * 2 + 2,
                color: "black",
                kind: ItemKind::Wall,
            });
        }
    }
    for y in 0..(height - 1) {
        for x in 0..width {
            board.push(Item {
                y: y * 2 + 2,
                x: x * 2 + 1,
                color: "black",
                kind: ItemKind::Wall,
            });
        }
    }

    let mut skip_line = vec![];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            row.push(is_black[y][x] == Some(true));
        }
        skip_line.push(row);
    }

    board.add_lines_irrefutable_facts(&is_line, "green", Some(&skip_line));

    for y in 0..(height * 2 - 1) {
        for x in 0..(width * 2 - 1) {
            if problem[y][x] == 0 {
                continue;
            }
            if problem[y][x] == 1 {
                board.push(Item {
                    y: y + 1,
                    x: x + 1,
                    color: "black",
                    kind: ItemKind::SmallFilledCircle,
                });
            } else if problem[y][x] == 2 {
                board.push(Item {
                    y: y + 1,
                    x: x + 1,
                    color: "#cccccc",
                    kind: ItemKind::SmallFilledCircle,
                });
            } else if problem[y][x] == 3 {
                board.push(Item {
                    y: y + 1,
                    x: x + 1,
                    color: "white",
                    kind: ItemKind::SmallFilledCircle,
                });
            }
            board.push(Item {
                y: y + 1,
                x: x + 1,
                color: "black",
                kind: ItemKind::SmallCircle,
            });
        }
    }

    Ok(board)
}
