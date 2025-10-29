use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::hebiichigo;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use cspuz_rs::items::Arrow;

    let problem = hebiichigo::deserialize_problem(url).ok_or("invalid url")?;
    let answer = hebiichigo::solve_hebiichigo(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&answer));

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                let arrow = match clue.0 {
                    Arrow::Unspecified => None,
                    Arrow::Up => Some(ItemKind::SideArrowUp),
                    Arrow::Down => Some(ItemKind::SideArrowDown),
                    Arrow::Left => Some(ItemKind::SideArrowLeft),
                    Arrow::Right => Some(ItemKind::SideArrowRight),
                };
                let n = clue.1;
                board.push(Item::cell(y, x, "black", ItemKind::Fill));
                if n >= 0 {
                    if let Some(arrow) = arrow {
                        board.push(Item::cell(y, x, "white", arrow));
                    }
                    board.push(Item::cell(
                        y,
                        x,
                        "white",
                        if n >= 0 {
                            ItemKind::Num(n)
                        } else {
                            ItemKind::Text("?")
                        },
                    ));
                }
            } else if let Some(n) = answer[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if n > 0 {
                        ItemKind::Num(n)
                    } else {
                        ItemKind::Dot
                    },
                ));
            }
        }
    }

    Ok(board)
}
