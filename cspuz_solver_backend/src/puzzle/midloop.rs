use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::midloop;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (full, dots) = midloop::deserialize_problem(url).ok_or("invalid url")?;
    let ans = midloop::solve_midloop(full, &dots);
    let height = (dots.len() + 1) / 2;
    let width = (dots[0].len() + 1) / 2;
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));
    if let Some(lines) = &ans {
        board.add_lines_irrefutable_facts(lines, "green", None);
    }
    for y in 0..dots.len() {
        for x in 0..dots[0].len() {
            let kind = match dots[y][x] {
                Some(0) => ItemKind::SmallCircle,
                Some(1) if y % 2 == 0 || x % 2 == 0 => ItemKind::SmallFilledCircle,
                _ => continue,
            };
            board.push(Item {
                y: y + 1,
                x: x + 1,
                color: "black",
                kind,
            });
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
            solve("https://puzz.link/p?midloop/3/3/ifqf"),
            Board {
                kind: BoardKind::Grid, height: 3, width: 3,
                data: vec![
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::SmallFilledCircle },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::SmallFilledCircle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
