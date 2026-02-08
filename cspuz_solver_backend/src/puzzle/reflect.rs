use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::reflect::{self, ReflectLinkClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = reflect::deserialize_problem(url).ok_or("invalid url")?;
    let is_line = reflect::solve_reflect_link(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&is_line));

    if let Some(is_line) = &is_line {
        board.add_lines_irrefutable_facts(is_line, "green", None);
    }
    for y in 0..height {
        for x in 0..width {
            match &problem[y][x] {
                ReflectLinkClue::None => (),
                ReflectLinkClue::UpperLeft(n) => {
                    board.push(Item::cell(y, x, "black", ItemKind::AboloUpperLeft));
                    if *n > 0 {
                        board.push(Item::cell(y, x, "white", ItemKind::NumUpperLeft(*n)));
                    }
                }
                ReflectLinkClue::UpperRight(n) => {
                    board.push(Item::cell(y, x, "black", ItemKind::AboloUpperRight));
                    if *n > 0 {
                        board.push(Item::cell(y, x, "white", ItemKind::NumUpperRight(*n)));
                    }
                }
                ReflectLinkClue::LowerLeft(n) => {
                    board.push(Item::cell(y, x, "black", ItemKind::AboloLowerLeft));
                    if *n > 0 {
                        board.push(Item::cell(y, x, "white", ItemKind::NumLowerLeft(*n)));
                    }
                }
                ReflectLinkClue::LowerRight(n) => {
                    board.push(Item::cell(y, x, "black", ItemKind::AboloLowerRight));
                    if *n > 0 {
                        board.push(Item::cell(y, x, "white", ItemKind::NumLowerRight(*n)));
                    }
                }
                ReflectLinkClue::Cross => {
                    board.push(Item::cell(y, x, "black", ItemKind::Plus));
                }
            }
        }
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
            solve("https://puzz.link/p?reflect/5/6/40f30c5d5a26a155h"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 5,
                data: vec![
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::AboloUpperLeft },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::AboloUpperRight },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Plus },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Plus },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::AboloLowerRight },
                    Item { y: 7, x: 7, color: "white", kind: ItemKind::NumLowerRight(6) },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::AboloLowerLeft },
                    Item { y: 9, x: 1, color: "white", kind: ItemKind::NumLowerLeft(5) },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::Plus },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
