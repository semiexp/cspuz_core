use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::yajilin;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use cspuz_rs::items::Arrow;

    let problem = yajilin::deserialize_problem(url).ok_or("invalid url")?;
    let (is_line, is_black) = yajilin::solve_yajilin(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        is_unique(&(&is_line, &is_black)),
    );

    let mut skip_line = vec![];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            row.push(problem[y][x].is_some() || is_black[y][x] == Some(true));
        }
        skip_line.push(row);
    }
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
            } else if let Some(b) = is_black[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if b { ItemKind::Block } else { ItemKind::Dot },
                ));
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", Some(&skip_line));

    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::board::*;
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("https://puzz.link/p?yajilin/10/10/w32a41b21a21l22e30m21a12b11r20d30g"),
            Board {
                kind: BoardKind::Grid,
                height: 10,
                width: 10,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 13, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 17, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::SideArrowLeft },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::SideArrowRight },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 15, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 17, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 5, x: 17, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 19, color: "black", kind: ItemKind::SideArrowLeft },
                    Item { y: 9, x: 19, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 7, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 13, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 13, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 11, color: "black", kind: ItemKind::SideArrowUp },
                    Item { y: 13, x: 11, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 13, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 13, x: 17, color: "black", kind: ItemKind::SideArrowUp },
                    Item { y: 13, x: 17, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 13, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 15, x: 17, color: "green", kind: ItemKind::Block },
                    Item { y: 15, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 15, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 17, x: 15, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 17, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 17, x: 19, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 5, color: "black", kind: ItemKind::SideArrowLeft },
                    Item { y: 19, x: 5, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 19, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 19, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 13, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 15, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 17, color: "green", kind: ItemKind::Dot },
                    Item { y: 19, x: 19, color: "green", kind: ItemKind::Block },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 18, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 18, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 18, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 18, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 18, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 18, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 18, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 18, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 18, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 18, color: "green", kind: ItemKind::Line },
                    Item { y: 19, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 19, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 19, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 19, x: 16, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
