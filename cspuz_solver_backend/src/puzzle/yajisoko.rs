use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::yajisoko::{self, YajisokoCell};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    use cspuz_rs::items::Arrow;

    let problem = yajisoko::deserialize_problem(url).ok_or("invalid url")?;
    let ans = yajisoko::solve_yajisoko(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        ans.as_ref()
            .map_or(Uniqueness::NoAnswer, |(is_line, is_block)| {
                is_unique(&(is_line, is_block))
            }),
    );

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                YajisokoCell::Arrow(clue) => {
                    let arrow = match clue.0 {
                        Arrow::Unspecified => None,
                        Arrow::Up => Some(ItemKind::SideArrowUp),
                        Arrow::Down => Some(ItemKind::SideArrowDown),
                        Arrow::Left => Some(ItemKind::SideArrowLeft),
                        Arrow::Right => Some(ItemKind::SideArrowRight),
                    };
                    let n = clue.1;
                    if let Some(arrow) = arrow {
                        board.push(Item::cell(y, x, "black", arrow));
                    }
                    board.push(Item::cell(
                        y,
                        x,
                        "black",
                        if n >= 0 {
                            ItemKind::Num(n)
                        } else {
                            ItemKind::Text("?")
                        },
                    ));
                }
                YajisokoCell::Block => {
                    board.push(Item::cell(y, x, "#cccccc", ItemKind::Block));
                }
                _ => (),
            }
            if let Some(ref ans) = ans {
                if ans.1[y][x] == Some(true) {
                    board.push(Item::cell(
                        y,
                        x,
                        "rgba(192, 255, 192, 0.8)",
                        ItemKind::Block,
                    ));
                }
            }
        }
    }

    if let Some(ref ans) = ans {
        board.add_lines_irrefutable_facts(&ans.0, "green", None);
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
            solve("https://pzprxs.vercel.app/p?yajisoko/5/4/-11-22-11h.-1dg.h..g-17.g..."),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 1, color: "rgba(192, 255, 192, 0.8)", kind: ItemKind::Block },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::SideArrowRight },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 1, x: 3, color: "rgba(192, 255, 192, 0.8)", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::SideArrowDown },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 3, x: 1, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::SideArrowRight },
                    Item { y: 3, x: 3, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 3, x: 5, color: "rgba(192, 255, 192, 0.8)", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 3, x: 7, color: "rgba(192, 255, 192, 0.8)", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "rgba(192, 255, 192, 0.8)", kind: ItemKind::Block },
                    Item { y: 5, x: 1, color: "rgba(192, 255, 192, 0.8)", kind: ItemKind::Block },
                    Item { y: 5, x: 3, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 5, x: 7, color: "rgba(192, 255, 192, 0.8)", kind: ItemKind::Block },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::SideArrowLeft },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 7, x: 1, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "rgba(192, 255, 192, 0.8)", kind: ItemKind::Block },
                    Item { y: 7, x: 5, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 7, x: 7, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 7, x: 9, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
