use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::simpleloop;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = simpleloop::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = simpleloop::solve_simpleloop(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        is_line.as_ref().map_or(Uniqueness::NoAnswer, |l| is_unique(l)),
    );

    for y in 0..height {
        for x in 0..width {
            if problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Fill));
            }
        }
    }

    if let Some(is_line) = &is_line {
        board.add_lines_irrefutable_facts(is_line, "green", Some(&problem));
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
            solve("https://puzz.link/p?simpleloop/8/7/200200a42000"),
            Board {
                kind: BoardKind::Grid,
                height: 7,
                width: 8,
                data: vec![
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 15, color: "black", kind: ItemKind::Fill },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::Fill },
                    Item { y: 9, x: 11, color: "black", kind: ItemKind::Fill },
                    Item { y: 11, x: 7, color: "black", kind: ItemKind::Fill },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 12, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 15, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 14, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 13, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 13, x: 14, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
