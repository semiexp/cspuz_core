use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::firewalk;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (fire_cell, num) = firewalk::deserialize_problem(url).ok_or("invalid url")?;
    let (is_line, fire_cell_mode) =
        firewalk::solve_firewalk(&fire_cell, &num).ok_or("no answer")?;

    let height = fire_cell.len();
    let width = fire_cell[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        is_unique(&(&is_line, &fire_cell_mode)),
    );

    for y in 0..height {
        for x in 0..width {
            if fire_cell[y][x] {
                board.push(Item::cell(y, x, "#ffe0e0", ItemKind::Fill));
            }
        }
    }
    board.add_lines_irrefutable_facts(&is_line, "green", None);

    for y in 0..height {
        for x in 0..width {
            if fire_cell[y][x] {
                let cell_item = match fire_cell_mode[y][x] {
                    Some(true) => {
                        let ur = y > 0 && is_line.vertical[y - 1][x] == Some(true);
                        let dl = x > 0 && is_line.horizontal[y][x - 1] == Some(true);

                        match (ur, dl) {
                            (true, true) => Some(ItemKind::FirewalkCellUrDl),
                            (true, false) => Some(ItemKind::FirewalkCellUr),
                            (false, true) => Some(ItemKind::FirewalkCellDl),
                            (false, false) => None,
                        }
                    }
                    Some(false) => {
                        let ul = y > 0 && is_line.vertical[y - 1][x] == Some(true);
                        let dr = x < width - 1 && is_line.horizontal[y][x] == Some(true);

                        match (ul, dr) {
                            (true, true) => Some(ItemKind::FirewalkCellUlDr),
                            (true, false) => Some(ItemKind::FirewalkCellUl),
                            (false, true) => Some(ItemKind::FirewalkCellDr),
                            (false, false) => None,
                        }
                    }
                    None => Some(ItemKind::FirewalkCellUnknown),
                };

                if let Some(cell_item) = cell_item {
                    board.push(Item::cell(y, x, "green", cell_item));
                }
            }
            if let Some(n) = num[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
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
            solve("https://pzprxs.vercel.app/p?firewalk/6/5/4m0008g1o83g6u"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 5, color: "#ffe0e0", kind: ItemKind::Fill },
                    Item { y: 1, x: 11, color: "#ffe0e0", kind: ItemKind::Fill },
                    Item { y: 3, x: 3, color: "#ffe0e0", kind: ItemKind::Fill },
                    Item { y: 3, x: 5, color: "#ffe0e0", kind: ItemKind::Fill },
                    Item { y: 9, x: 5, color: "#ffe0e0", kind: ItemKind::Fill },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::FirewalkCellDl },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::FirewalkCellUlDr },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::FirewalkCellUrDl },
                    Item { y: 3, x: 11, color: "black", kind: ItemKind::Num(8) },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 5, color: "black", kind: ItemKind::Num(6) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::FirewalkCellUr },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
