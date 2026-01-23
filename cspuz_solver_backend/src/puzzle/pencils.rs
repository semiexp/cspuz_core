use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::pencils::{self, PencilsAnswer, PencilsClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = pencils::deserialize_problem(url).ok_or("invalid url")?;
    let ans = pencils::solve_pencils(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::OuterGrid,
        height,
        width,
        ans.as_ref()
            .map_or(Uniqueness::NoAnswer, |(cell, line, _)| {
                is_unique(&(cell, line))
            }),
    );

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                PencilsClue::Num(-1) => board.push(Item::cell(y, x, "black", ItemKind::Text("?"))),
                PencilsClue::Num(n) => board.push(Item::cell(y, x, "black", ItemKind::Num(n))),
                PencilsClue::Left => board.push(Item::cell(y, x, "black", ItemKind::PencilLeft)),
                PencilsClue::Right => board.push(Item::cell(y, x, "black", ItemKind::PencilRight)),
                PencilsClue::Up => board.push(Item::cell(y, x, "black", ItemKind::PencilUp)),
                PencilsClue::Down => board.push(Item::cell(y, x, "black", ItemKind::PencilDown)),
                _ => {
                    if let Some((cell, _, _)) = &ans {
                        if let Some(c) = cell[y][x] {
                            match c {
                                PencilsAnswer::Left => {
                                    board.push(Item::cell(y, x, "green", ItemKind::PencilLeft))
                                }
                                PencilsAnswer::Right => {
                                    board.push(Item::cell(y, x, "green", ItemKind::PencilRight))
                                }
                                PencilsAnswer::Up => {
                                    board.push(Item::cell(y, x, "green", ItemKind::PencilUp))
                                }
                                PencilsAnswer::Down => {
                                    board.push(Item::cell(y, x, "green", ItemKind::PencilDown))
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if y < height - 1 {
                let mut need_default_edge = true;
                if let Some((_, line, border)) = &ans {
                    let kind = match (border.horizontal[y][x], line.vertical[y][x]) {
                        (Some(true), _) => Some(ItemKind::BoldWall),
                        (_, Some(true)) => Some(ItemKind::Line),
                        (Some(false), Some(false)) => Some(ItemKind::Cross),
                        _ => None,
                    };
                    if kind == Some(ItemKind::BoldWall) {
                        need_default_edge = false;
                    }
                    if let Some(kind) = kind {
                        board.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color: "green",
                            kind,
                        })
                    }
                }
                if need_default_edge {
                    board.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "#cccccc",
                        kind: ItemKind::Wall,
                    });
                }
            }
            if x < width - 1 {
                let mut need_default_edge = true;
                if let Some((_, line, border)) = &ans {
                    let kind = match (border.vertical[y][x], line.horizontal[y][x]) {
                        (Some(true), _) => Some(ItemKind::BoldWall),
                        (_, Some(true)) => Some(ItemKind::Line),
                        (Some(false), Some(false)) => Some(ItemKind::Cross),
                        _ => None,
                    };
                    if kind == Some(ItemKind::BoldWall) {
                        need_default_edge = false;
                    }
                    if let Some(kind) = kind {
                        board.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 2,
                            color: "green",
                            kind,
                        })
                    }
                }
                if need_default_edge {
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "#cccccc",
                        kind: ItemKind::Wall,
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
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("https://puzz.link/p?pencils/6/6/kgin3p2oil2njkimjk"),
            Board {
                kind: BoardKind::OuterGrid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::PencilDown },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::PencilRight },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::PencilDown },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::PencilLeft },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::PencilRight },
                    Item { y: 7, x: 11, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::PencilLeft },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::PencilRight },
                    Item { y: 11, x: 9, color: "black", kind: ItemKind::PencilLeft },
                    Item { y: 2, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 1, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 9, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 11, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 4, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 8, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 10, color: "#cccccc", kind: ItemKind::Wall },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
