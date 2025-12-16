use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs::items::Arrow;
use cspuz_rs_puzzles::puzzles::castle_walker::{self, CastleWalkerClue, CastleWalkerSquare};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = castle_walker::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = castle_walker::solve_castle_walker(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                CastleWalkerClue::Cross => {
                    board.push(Item::cell(y, x, "black", ItemKind::Plus));
                }
                CastleWalkerClue::Cell {
                    square,
                    number,
                    arrow,
                } => {
                    let fg_color = if square == CastleWalkerSquare::Black {
                        "white"
                    } else {
                        "black"
                    };
                    match square {
                        CastleWalkerSquare::None => {}
                        CastleWalkerSquare::Black => {
                            board.push(Item::cell(y, x, "black", ItemKind::Block));
                        }
                        CastleWalkerSquare::Gray => {
                            board.push(Item::cell(y, x, "#cccccc", ItemKind::Block));
                        }
                        CastleWalkerSquare::White => {
                            board.push(Item::cell(y, x, "black", ItemKind::Circle));
                        }
                    }
                    if let Some(n) = number {
                        board.push(Item::cell(y, x, fg_color, ItemKind::Num(n)));
                    }
                    let arrow = match arrow {
                        Arrow::Unspecified => None,
                        Arrow::Up => Some(ItemKind::SideArrowUp),
                        Arrow::Down => Some(ItemKind::SideArrowDown),
                        Arrow::Left => Some(ItemKind::SideArrowLeft),
                        Arrow::Right => Some(ItemKind::SideArrowRight),
                    };
                    if let Some(arrow) = arrow {
                        board.push(Item::cell(y, x, fg_color, arrow));
                    }
                }
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
