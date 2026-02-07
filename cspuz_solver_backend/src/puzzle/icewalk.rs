use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::icewalk;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (icebarn, num) = icewalk::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = icewalk::solve_icewalk(&icebarn, &num);

    let height = icebarn.len();
    let width = icebarn[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&is_line),
    );

    for y in 0..height {
        for x in 0..width {
            if icebarn[y][x] {
                board.push(Item::cell(y, x, "#e0e0ff", ItemKind::Fill));
            }
            if let Some(n) = num[y][x] {
                if n >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            }
        }
    }

    if let Some(is_line) = &is_line {
        board.add_lines_irrefutable_facts(is_line, "green", None);
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
            solve("https://puzz.link/p?icewalk/6/7/g63845qg0l2h2k3p5g1k3l3"),
            Board {
                kind: BoardKind::Grid,
                height: 7,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 3, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 3, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 9, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 7, x: 9, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 9, x: 7, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 11, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 11, x: 1, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 11, x: 3, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 11, x: 7, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 11, x: 9, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 11, x: 11, color: "#e0e0ff", kind: ItemKind::Fill },
                    Item { y: 13, x: 11, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 10, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
