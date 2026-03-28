use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::gokigen::{self, GOKIGEN_SLASH};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = gokigen::deserialize_problem(url).ok_or("invalid url")?;
    let ans = gokigen::solve_gokigen(&problem);

    let height = problem.len() - 1;
    let width = problem[0].len() - 1;
    let mut board = Board::new(BoardKind::Empty, height, width, check_uniqueness(&ans));
    if let Some(ans) = ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if a == GOKIGEN_SLASH {
                            ItemKind::Slash
                        } else {
                            ItemKind::Backslash
                        },
                    ));
                }
            }
        }
    }
    for y in 0..=height {
        for x in 0..=width {
            if y < height {
                board.push(Item {
                    y: y * 2 + 1,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::DottedWall,
                });
            }
            if x < width {
                board.push(Item {
                    y: y * 2,
                    x: x * 2 + 1,
                    color: "black",
                    kind: ItemKind::DottedWall,
                });
            }
        }
    }
    for y in 0..=height {
        for x in 0..=width {
            if let Some(n) = problem[y][x] {
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "white",
                    kind: ItemKind::FilledCircle,
                });
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::Circle,
                });
                if n >= 0 {
                    board.push(Item {
                        y: y * 2,
                        x: x * 2,
                        color: "black",
                        kind: ItemKind::Num(n),
                    });
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
            solve("https://puzz.link/p?gokigen/4/3/l372666bg"),
            Board {
                kind: BoardKind::Empty,
                height: 3,
                width: 4,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Backslash },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Slash },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Backslash },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Backslash },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Slash },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Slash },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Backslash },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Slash },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Slash },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Slash },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Slash },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Slash },
                    Item { y: 1, x: 0, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 2, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 0, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 0, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 0, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::DottedWall },
                    Item { y: 2, x: 2, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 2, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 2, x: 4, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 4, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 4, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 2, x: 8, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 2, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 2, x: 8, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 4, x: 0, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 4, x: 0, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 0, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 4, x: 4, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 4, x: 4, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 4, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 4, x: 8, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 4, x: 8, color: "black", kind: ItemKind::Circle },
                    Item { y: 4, x: 8, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 6, x: 2, color: "white", kind: ItemKind::FilledCircle },
                    Item { y: 6, x: 2, color: "black", kind: ItemKind::Circle },
                    Item { y: 6, x: 2, color: "black", kind: ItemKind::Num(1) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
