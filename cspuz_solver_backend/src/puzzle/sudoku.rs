use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs_puzzles::puzzles::sudoku;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = sudoku::deserialize_problem(url).ok_or("invalid url")?;
    let ans = sudoku::solve_sudoku_as_cands(&problem).ok_or("no answer")?;

    let height = ans.len();
    let width = ans[0].len();

    let mut is_unique = Uniqueness::Unique;
    for y in 0..height {
        for x in 0..width {
            if ans[y][x].iter().filter(|&&b| b).count() != 1 {
                is_unique = Uniqueness::NonUnique;
            }
        }
    }
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique);

    let (bh, bw) = match height {
        4 => (2, 2),
        6 => (2, 3),
        9 => (3, 3),
        16 => (4, 4),
        25 => (5, 5),
        _ => return Err("invalid size"),
    };

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
            } else {
                let mut cands = vec![];
                for i in 0..height {
                    if ans[y][x][i] {
                        cands.push(i as i32 + 1);
                    }
                }
                if cands.len() == 1 {
                    board.push(Item::cell(y, x, "green", ItemKind::Num(cands[0])));
                } else {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        ItemKind::SudokuCandidateSet(bw as i32, cands),
                    ));
                }
            }
        }
    }
    for x in 0..bh {
        for y in 0..height {
            board.push(Item {
                y: 2 * y + 1,
                x: 2 * x * bw,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }
    for y in 0..bw {
        for x in 0..width {
            board.push(Item {
                y: 2 * y * bh,
                x: 2 * x + 1,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }

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
            solve("https://puzz.link/p?sudoku/9/9/k8g1g7i2i99o2g3h75q19h5g4o83i4i6g4g5k"),
            Board {
                kind: BoardKind::Grid,
                height: 9,
                width: 9,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Num(8) },
                    Item { y: 1, x: 13, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 15, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 17, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(7) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 13, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 3, x: 15, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 3, x: 17, color: "black", kind: ItemKind::Num(9) },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(9) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 13, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 15, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 5, x: 17, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 7, x: 13, color: "black", kind: ItemKind::Num(7) },
                    Item { y: 7, x: 15, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 7, x: 17, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 9, x: 13, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 9, x: 15, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 9, x: 17, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 11, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 11, x: 5, color: "black", kind: ItemKind::Num(9) },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 11, x: 11, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 11, x: 13, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 11, x: 15, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 11, x: 17, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 13, x: 1, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 13, x: 3, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 13, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 13, x: 7, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 13, x: 9, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 13, x: 11, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 13, x: 13, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 13, x: 15, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 13, x: 17, color: "black", kind: ItemKind::Num(8) },
                    Item { y: 15, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 15, x: 3, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 15, x: 5, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 15, x: 7, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 15, x: 9, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 15, x: 11, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 15, x: 13, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 15, x: 15, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 15, x: 17, color: "black", kind: ItemKind::Num(6) },
                    Item { y: 17, x: 1, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 17, x: 3, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 17, x: 5, color: "green", kind: ItemKind::Num(8) },
                    Item { y: 17, x: 7, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 17, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 17, x: 11, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 17, x: 13, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 17, x: 15, color: "green", kind: ItemKind::Num(9) },
                    Item { y: 17, x: 17, color: "green", kind: ItemKind::Num(7) },
                    Item { y: 1, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 13, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 15, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 17, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 17, color: "black", kind: ItemKind::BoldWall },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
