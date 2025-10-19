use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::school_trip;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = school_trip::deserialize_problem(url).ok_or("invalid url")?;
    let (is_black, is_pillow, is_connected) = school_trip::solve_school_trip(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::OuterGrid, height, width, is_unique(&(&is_black, &is_pillow, &is_connected)));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
                if clue >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                }
            } else {
                if is_black[y][x] == Some(true) {
                    board.push(Item::cell(y, x, "green", ItemKind::Block));
                } else if is_pillow[y][x] == Some(true) {
                    board.push(Item::cell(y, x, "green", ItemKind::Square));
                } else if is_black[y][x] == Some(false) {
                    board.push(Item::cell(y, x, "green", ItemKind::Dot));
                }
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            if y < height - 1 { 
                if (is_black[y][x] == Some(false) && problem[y][x] == None) || (is_black[y + 1][x] == Some(false) && problem[y + 1][x] == None) { 
                    // If a cell is not black in the solution then either it is a number in the problem, or a futon. This checks which cells are futons                                                                                                                                        
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: if is_connected.vertical[y][x].is_some() {
                            "green"
                        } else {
                            "#cccccc"
                        },
                        kind: 
                            match is_connected.vertical[y][x] {
                            Some(true) => ItemKind::Cross,
                            Some(false) => ItemKind::BoldWall,
                            None => ItemKind::Wall,
                            }
                        },
                    )
                };
            }
            if x < width - 1 {
                if (is_black[y][x] == Some(false) && problem[y][x] == None) || (is_black[y][x + 1] == Some(false) && problem[y][x + 1] == None) { 
                    // If a cell is not black in the solution then either it is a number in the problem, or a futon. This checks which cells are futons                                                                                                                                        
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: if is_connected.horizontal[y][x].is_some() {
                            "green"
                        } else {
                            "#cccccc"
                        },
                        kind: 
                            match is_connected.horizontal[y][x] {
                            Some(true) => ItemKind::Cross,
                            Some(false) => ItemKind::BoldWall,
                            None => ItemKind::Wall,
                            }
                        },
                    )
                };
            }
        }
    }

    Ok(board)
}
