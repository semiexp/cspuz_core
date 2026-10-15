use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::balance_loop;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = balance_loop::deserialize_problem(url).ok_or("invalid url")?;
    let ans = balance_loop::solve_balance_loop(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    for y in 0..height {
        for x in 0..width {
            if let Some((n, is_black)) = problem[y][x] {
                if is_black {
                    board.push(Item::cell(y, x, "black", ItemKind::FilledCircle));
                    board.push(Item::cell(y, x, "white", ItemKind::Num(n)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Circle));
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                }
            }
        }
    }

    if let Some(is_line) = &ans {
        board.add_lines_irrefutable_facts(is_line, "green", None);
    }

    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use cspuz_rs_puzzles::puzzles::balance_loop;

    #[test]
    fn test_solve() {
        let problem = vec![
            vec![Some((4, false)), Some((2, false)), Some((4, false))],
            vec![Some((2, false)), None, Some((2, false))],
            vec![Some((4, false)), Some((2, false)), Some((4, false))],
        ];
        let url = balance_loop::serialize_problem(&problem).unwrap();
        let board = solve(&url).unwrap();
        assert_eq!(board.height, 3);
        assert_eq!(board.width, 3);
    }
}
