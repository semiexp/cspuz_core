use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs::items::Arrow;
use cspuz_rs_puzzles::puzzles::castle_walker::{self, CastleWalkerClue, CastleWalkerSquare};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = castle_walker::deserialize_problem(url).ok_or("invalid url")?;
    let ans = castle_walker::solve_castle_walker(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        ans.as_ref().map_or(Uniqueness::NoAnswer, |a| is_unique(a)),
    );

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                CastleWalkerClue::Cross => {
                    board.push(Item::cell(y, x, "black", ItemKind::Plus));
                }
                CastleWalkerClue::Cell {
                    square,
                    number,
                    arrow,
                } => {
                    let fg_color = if square == CastleWalkerSquare::Black {
                        "white"
                    } else {
                        "black"
                    };
                    match square {
                        CastleWalkerSquare::None => {}
                        CastleWalkerSquare::Black => {
                            board.push(Item::cell(y, x, "black", ItemKind::Block));
                        }
                        CastleWalkerSquare::Gray => {
                            board.push(Item::cell(y, x, "#cccccc", ItemKind::Block));
                        }
                        CastleWalkerSquare::White => {
                            board.push(Item::cell(y, x, "black", ItemKind::Circle));
                        }
                    }
                    if let Some(n) = number {
                        board.push(Item::cell(y, x, fg_color, ItemKind::Num(n)));
                    }
                    let arrow = match arrow {
                        Arrow::Unspecified => None,
                        Arrow::Up => Some(ItemKind::SideArrowUp),
                        Arrow::Down => Some(ItemKind::SideArrowDown),
                        Arrow::Left => Some(ItemKind::SideArrowLeft),
                        Arrow::Right => Some(ItemKind::SideArrowRight),
                    };
                    if let Some(arrow) = arrow {
                        board.push(Item::cell(y, x, fg_color, arrow));
                    }
                }
            }
        }
    }

    if let Some(ans) = &ans {
        board.add_lines_irrefutable_facts(ans, "green", None);
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
            solve("https://opt-pan.github.io/penpa-edit/?m=solve&p=tZTPb5swFMfv/BWTzz6AIaTllnXNLiz7lamqEEJOQhtUEncG1spR+rf3+ZkKDOywaRPy0+PDw++L7S/Vz4bLnIZw+RfUpR5cLAxxeEGAw22vdVGXefSOLpp6LyQklH5eLukdL6vcSdqq1EmIRyhhMDySvqj4BYGfOif1LTqpLErSM1U/uvSiS79HJ4ir6ERYSKKE+Bm86unZUkr8uUaZ25EAi/o14YjMmSYsA0EGQQMP29y+tfEpMeuQxVD01mlEAx8oG1E9A8xs01kAdEZVjHehfrO7m+yJOgcURC5RKsO4hkWiysf4AaOLcYYxxpprjDcYrzAGGEOsmetldpyEMdxpc83+Pofdhn0llSizqpF3fJuTCM8DfAewY3PY5NJCpRCPZXG064r7o5B596iWTa88391PlW+E3A0mf+JlaQGzmBbaFnJb2qiWhXXPpRRPFjnwem+BDa/BDNW+eLRnyo+1LaDmtkT+wAfdDt1qnB3yTHDAOWPgQljgk7qM1IKqj3CKe8ai6ivY5lOkVto1CcFDBGcAixik1116g891dmWg50K+0rYwr91Cap1G9SVK1JwS3ec9vq1TchC/QKrRoe+34rCBj0lIbznMk6rZiYemrcWDvDBy4wm5fidXp0auzoZy2+/Rcs029uWu/5Xcy/RsNsL9o19W/7/yn4z73HpNyEm7AZ5wHNBJ07V85C7gIx/phmMrAZ1wE9ChoQCNPQVwZCtgv3GWnnVoLq1q6C/damQx3arvsiR1XgE=&a=RZBRCsRQCAPv0u/8rBo9TOn9r9HsxsdCwUEzedD7fnBfUQjic+FLWSYi2tSIMQ3SlHlyokoTkTZE5T4t1lCobGhwSTkbGrW5AU3MkxO139CCbhb1Xgm6RaOXBu2WjuOKJkzqsyuavfK4ovEbqhi3SNycTuOcxu+qP/j/nhc="),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 9, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::SideArrowLeft },
                    Item { y: 3, x: 11, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::SideArrowUp },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Block },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::SideArrowLeft },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Plus },
                    Item { y: 9, x: 3, color: "black", kind: ItemKind::Plus },
                    Item { y: 9, x: 9, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::SideArrowLeft },
                    Item { y: 11, x: 1, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::SideArrowRight },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
