use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::double_lits;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let borders = double_lits::deserialize_problem(url).ok_or("invalid url")?;
    let ans = double_lits::solve_double_lits(&borders);

    let height = borders.vertical.len();
    let width = if height > 0 {
        borders.vertical[0].len() + 1
    } else {
        0
    };
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );

    board.add_borders(&borders, "black");

    if let Some(is_black) = ans {
        for y in 0..height {
            for x in 0..width {
                if let Some(b) = is_black[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b { ItemKind::Block } else { ItemKind::Dot },
                    ));
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
            solve("https://pedros.works/paper-puzzle-player?W=6x6&SIE=19U3LLUUUURRRDRDLLDDD&G=lits&V=double"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::NonUnique,
            },
        );
    }
}
