use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::mintonette;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = mintonette::deserialize_problem(url).ok_or("invalid url")?;
    let ans = mintonette::solve_mintonette(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
                if clue > -1 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                }
            }
        }
    }

    if let Some(ans) = ans {
        board.add_lines_irrefutable_facts(&ans, "green", None);
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
            solve("https://pzprxs.vercel.app/p?mintonette/4/4/h..n12.2"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 4,
                data: vec![
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Circle }, 
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Circle }, 
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Circle }, 
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Num(1) }, 
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Circle }, 
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(2) }, 
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Circle }, 
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Circle }, 
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(2) }, 
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line }, 
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line }, 
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line }, 
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line }, 
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line }, 
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Line }, 
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line }, 
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line }, 
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line }, 
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line }, 
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line }, 
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line }, 
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Line }, 
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
