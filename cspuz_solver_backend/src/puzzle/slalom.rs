use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::slalom;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use slalom::{SlalomBlackCellDir, SlalomCell};

    let problem = slalom::deserialize_problem_as_primitive(url).ok_or("invalid url")?;
    let (is_black, gates, origin) = slalom::parse_primitive_problem(&problem);
    let is_line = slalom::solve_slalom(origin, &is_black, &gates).ok_or("no answer")?;

    let height = is_black.len();
    let width = is_black[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&is_line));

    let (origin_y, origin_x) = origin;
    board.push(Item::cell(origin_y, origin_x, "black", ItemKind::Circle));
    board.push(Item::cell(
        origin_y,
        origin_x,
        "black",
        ItemKind::Num(gates.len() as i32),
    ));

    for y in 0..height {
        for x in 0..width {
            match problem.0[y][x] {
                SlalomCell::Black(d, n) => {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                    if n >= 0 {
                        board.push(Item::cell(y, x, "white", ItemKind::Num(n)));
                    }
                    let arrow = match d {
                        SlalomBlackCellDir::Up => ItemKind::SideArrowUp,
                        SlalomBlackCellDir::Down => ItemKind::SideArrowDown,
                        SlalomBlackCellDir::Left => ItemKind::SideArrowLeft,
                        SlalomBlackCellDir::Right => ItemKind::SideArrowRight,
                        _ => continue,
                    };
                    board.push(Item::cell(y, x, "white", arrow));
                }
                SlalomCell::Horizontal => {
                    board.push(Item::cell(y, x, "black", ItemKind::DottedHorizontalWall));
                }
                SlalomCell::Vertical => {
                    board.push(Item::cell(y, x, "black", ItemKind::DottedVerticalWall));
                }
                SlalomCell::White => (),
            }
        }
    }

    board.add_lines_irrefutable_facts(&is_line, "green", Some(&is_black));

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
            solve("https://puzz.link/p?slalom/d/10/10/h133316131f131p1333315131f1333351aj11314333h42g/51"),
            Board {
                kind: BoardKind::Grid,
                height: 10,
                width: 10,
                data: vec![
                    Item { y: 11, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 11, x: 3, color: "black", kind: ItemKind::Num(6) },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Fill },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 3, x: 13, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 3, x: 15, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 3, x: 17, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 15, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 15, color: "white", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 15, color: "white", kind: ItemKind::SideArrowUp },
                    Item { y: 7, x: 17, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 7, x: 19, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 19, color: "white", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 19, color: "white", kind: ItemKind::SideArrowLeft },
                    Item { y: 13, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 13, x: 5, color: "white", kind: ItemKind::Num(3) },
                    Item { y: 13, x: 5, color: "white", kind: ItemKind::SideArrowRight },
                    Item { y: 13, x: 7, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 13, x: 9, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 13, x: 11, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 13, x: 13, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 13, x: 15, color: "black", kind: ItemKind::Fill },
                    Item { y: 13, x: 15, color: "white", kind: ItemKind::Num(3) },
                    Item { y: 13, x: 15, color: "white", kind: ItemKind::SideArrowLeft },
                    Item { y: 15, x: 1, color: "black", kind: ItemKind::Fill },
                    Item { y: 15, x: 3, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 15, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 17, x: 11, color: "black", kind: ItemKind::Fill },
                    Item { y: 17, x: 11, color: "white", kind: ItemKind::Num(2) },
                    Item { y: 17, x: 11, color: "white", kind: ItemKind::SideArrowRight },
                    Item { y: 17, x: 13, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 17, x: 15, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 17, x: 17, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 17, x: 19, color: "black", kind: ItemKind::DottedHorizontalWall },
                    Item { y: 19, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 14, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 14, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 14, x: 17, color: "green", kind: ItemKind::Line },
                    Item { y: 14, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 16, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 16, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 18, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 18, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 18, x: 19, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 18, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 15, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 15, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 17, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 16, color: "green", kind: ItemKind::Cross },
                    Item { y: 17, x: 18, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 19, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 19, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 19, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 19, x: 16, color: "green", kind: ItemKind::Line },
                    Item { y: 19, x: 18, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
