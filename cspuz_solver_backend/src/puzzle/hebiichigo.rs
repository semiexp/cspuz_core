use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::hebiichigo;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use cspuz_rs::items::Arrow;

    let problem = hebiichigo::deserialize_problem(url).ok_or("invalid url")?;
    let answer = hebiichigo::solve_hebiichigo(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&answer));

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                let arrow = match clue.0 {
                    Arrow::Unspecified => None,
                    Arrow::Up => Some(ItemKind::SideArrowUp),
                    Arrow::Down => Some(ItemKind::SideArrowDown),
                    Arrow::Left => Some(ItemKind::SideArrowLeft),
                    Arrow::Right => Some(ItemKind::SideArrowRight),
                };
                let n = clue.1;
                board.push(Item::cell(y, x, "black", ItemKind::Fill));
                if n >= 0 {
                    if let Some(arrow) = arrow {
                        board.push(Item::cell(y, x, "white", arrow));
                    }
                    board.push(Item::cell(
                        y,
                        x,
                        "white",
                        if n >= 0 {
                            ItemKind::Num(n)
                        } else {
                            ItemKind::Text("?")
                        },
                    ));
                }
            } else if let Some(n) = answer[y][x] {
                board.push(Item::cell(
                    y,
                    x,
                    "green",
                    if n > 0 {
                        ItemKind::Num(n)
                    } else {
                        ItemKind::Dot
                    },
                ));
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
            solve("https://puzz.link/p?hebi/6/5/b31e0.a45h23g45b"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "white", kind: ItemKind::SideArrowLeft },
                    Item { y: 1, x: 5, color: "white", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "white", kind: ItemKind::SideArrowRight },
                    Item { y: 3, x: 9, color: "white", kind: ItemKind::Num(5) },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "white", kind: ItemKind::SideArrowDown },
                    Item { y: 7, x: 3, color: "white", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Num(3) },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Fill },
                    Item { y: 9, x: 7, color: "white", kind: ItemKind::SideArrowRight },
                    Item { y: 9, x: 7, color: "white", kind: ItemKind::Num(5) },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Num(5) },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Num(4) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
