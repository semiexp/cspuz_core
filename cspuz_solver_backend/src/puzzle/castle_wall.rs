use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::castle_wall;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use castle_wall::Side;
    use cspuz_rs::items::Arrow;
    let problem = castle_wall::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = castle_wall::solve_castle_wall(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&is_line));

    let mut skip_line = vec![];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            row.push(problem[y][x].is_some());
        }
        skip_line.push(row);
    }
    for y in 0..height {
        for x in 0..width {
            if let Some((side, arrow)) = problem[y][x] {
                let arrow_color = match side {
                    Side::Outside => "white",
                    _ => "black",
                };
                if side == Side::Outside {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                } else if side == Side::Unspecified {
                    board.push(Item::cell(y, x, "#cccccc", ItemKind::Fill));
                }
                let n = arrow.1;
                let arrow = match arrow.0 {
                    Arrow::Unspecified => None,
                    Arrow::Up => Some(ItemKind::SideArrowUp),
                    Arrow::Down => Some(ItemKind::SideArrowDown),
                    Arrow::Left => Some(ItemKind::SideArrowLeft),
                    Arrow::Right => Some(ItemKind::SideArrowRight),
                };
                if n >= 0 {
                    if let Some(arrow) = arrow {
                        board.push(Item::cell(y, x, arrow_color, arrow));
                    }
                    board.push(Item::cell(y, x, arrow_color, ItemKind::Num(n)));
                }
            }
        }
    }

    if let Some(is_line) = is_line {
        board.add_lines_irrefutable_facts(&is_line, "green", Some(&skip_line));
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
            solve("https://puzz.link/p?castle/10/10/023b022b023v224c032f044p113c044b014w214b014b014e"),
            Board {
                kind: BoardKind::Grid,
                height: 10,
                width: 10,
                data: vec![
                    Item { y: 1, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 13, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 1, x: 13, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 19, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 19, color: "white", kind: ItemKind::SideArrowDown },
                    Item { y: 5, x: 19, color: "white", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::SideArrowLeft },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 1, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::SideArrowRight },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 11, x: 15, color: "black", kind: ItemKind::SideArrowUp },
                    Item { y: 11, x: 15, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 13, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 13, x: 3, color: "black", kind: ItemKind::SideArrowRight },
                    Item { y: 13, x: 3, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 13, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 13, x: 9, color: "black", kind: ItemKind::SideArrowUp },
                    Item { y: 13, x: 9, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 17, x: 17, color: "black", kind: ItemKind::Fill },
                    Item { y: 17, x: 17, color: "white", kind: ItemKind::SideArrowUp },
                    Item { y: 17, x: 17, color: "white", kind: ItemKind::Num(4) },
                    Item { y: 19, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 19, x: 3, color: "black", kind: ItemKind::SideArrowUp },
                    Item { y: 19, x: 3, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 19, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 19, x: 9, color: "black", kind: ItemKind::SideArrowUp },
                    Item { y: 19, x: 9, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 14, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 14, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 18, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 18, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 18, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
