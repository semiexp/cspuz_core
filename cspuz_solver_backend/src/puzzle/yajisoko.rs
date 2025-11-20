use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::yajisoko::{self, YajisokoCell};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use cspuz_rs::items::Arrow;

    let problem = yajisoko::deserialize_problem(url).ok_or("invalid url")?;
    let (is_line, is_block) = yajisoko::solve_yajisoko(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        is_unique(&(&is_line, &is_block)),
    );

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                YajisokoCell::Arrow(clue) => {
                    let arrow = match clue.0 {
                        Arrow::Unspecified => None,
                        Arrow::Up => Some(ItemKind::SideArrowUp),
                        Arrow::Down => Some(ItemKind::SideArrowDown),
                        Arrow::Left => Some(ItemKind::SideArrowLeft),
                        Arrow::Right => Some(ItemKind::SideArrowRight),
                    };
                    let n = clue.1;
                    if let Some(arrow) = arrow {
                        board.push(Item::cell(y, x, "black", arrow));
                    }
                    board.push(Item::cell(
                        y,
                        x,
                        "black",
                        if n >= 0 {
                            ItemKind::Num(n)
                        } else {
                            ItemKind::Text("?")
                        },
                    ));
                }
                YajisokoCell::Block => {
                    board.push(Item::cell(y, x, "#cccccc", ItemKind::Block));
                }
                _ => (),
            }
            if is_block[y][x] == Some(true) {
                board.push(Item::cell(
                    y,
                    x,
                    "rgba(192, 255, 192, 0.8)",
                    ItemKind::Block,
                ));
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", None);

    Ok(board)
}
