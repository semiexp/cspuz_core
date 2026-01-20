use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::is_unique;
use cspuz_rs_puzzles::puzzles::akari_rgb::{self, AkariRGBClue};

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = akari_rgb::deserialize_problem(url).ok_or("invalid url")?;
    let ans = akari_rgb::solve_akari_rgb(&problem).ok_or("no answer")?;

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, is_unique(&ans));
    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                AkariRGBClue::Block => {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                    continue;
                }
                AkariRGBClue::Num(n) => {
                    board.push(Item::cell(y, x, "black", ItemKind::Fill));
                    board.push(Item::cell(y, x, "white", ItemKind::Num(n)));
                    continue;
                }
                AkariRGBClue::Empty => (),
                AkariRGBClue::R => {
                    board.push(Item::cell(y, x, "#ffcccc", ItemKind::Fill));
                }
                AkariRGBClue::G => {
                    board.push(Item::cell(y, x, "#ccffcc", ItemKind::Fill));
                }
                AkariRGBClue::B => {
                    board.push(Item::cell(y, x, "#ccccff", ItemKind::Fill));
                }
                AkariRGBClue::RG => {
                    board.push(Item::cell(y, x, "#ffffcc", ItemKind::Fill));
                }
                AkariRGBClue::GB => {
                    board.push(Item::cell(y, x, "#ccffff", ItemKind::Fill));
                }
                AkariRGBClue::BR => {
                    board.push(Item::cell(y, x, "#ffccff", ItemKind::Fill));
                }
            }

            match ans[y][x] {
                None => (),
                Some(0) => {
                    board.push(Item::cell(y, x, "black", ItemKind::Dot));
                }
                Some(1) => {
                    board.push(Item::cell(y, x, "#ff0000", ItemKind::FilledCircle));
                    board.push(Item::cell(y, x, "white", ItemKind::Text("R")));
                }
                Some(2) => {
                    board.push(Item::cell(y, x, "#00ff00", ItemKind::FilledCircle));
                    board.push(Item::cell(y, x, "white", ItemKind::Text("G")));
                }
                Some(3) => {
                    board.push(Item::cell(y, x, "#0000ff", ItemKind::FilledCircle));
                    board.push(Item::cell(y, x, "white", ItemKind::Text("B")));
                }
                _ => unreachable!(),
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
            solve("https://pedros.works/paper-puzzle-player?W=6x5&L=M3C3(3)1B2z4z2R7G1Y3z1&G=akari-rgb"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Dot },
                    Item { y: 1, x: 3, color: "#ccccff", kind: ItemKind::Fill },
                    Item { y: 1, x: 3, color: "black", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "#0000ff", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 5, color: "white", kind: ItemKind::Text("B") },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "black", kind: ItemKind::Dot },
                    Item { y: 3, x: 1, color: "#ffccff", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "#0000ff", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 3, color: "white", kind: ItemKind::Text("B") },
                    Item { y: 3, x: 5, color: "black", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "black", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "#ccffcc", kind: ItemKind::Fill },
                    Item { y: 3, x: 9, color: "black", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "#00ff00", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 11, color: "white", kind: ItemKind::Text("G") },
                    Item { y: 5, x: 1, color: "#ff0000", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 1, color: "white", kind: ItemKind::Text("R") },
                    Item { y: 5, x: 3, color: "black", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "white", kind: ItemKind::Num(3) },
                    Item { y: 5, x: 5, color: "#ff0000", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 5, color: "white", kind: ItemKind::Text("R") },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "#ffcccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Dot },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::Fill },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Dot },
                    Item { y: 7, x: 3, color: "#ccffff", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "#00ff00", kind: ItemKind::FilledCircle },
                    Item { y: 7, x: 7, color: "white", kind: ItemKind::Text("G") },
                    Item { y: 7, x: 9, color: "black", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "#ffffcc", kind: ItemKind::Fill },
                    Item { y: 7, x: 11, color: "black", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "black", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "#0000ff", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 3, color: "white", kind: ItemKind::Text("B") },
                    Item { y: 9, x: 5, color: "black", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "black", kind: ItemKind::Fill },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "#ff0000", kind: ItemKind::FilledCircle },
                    Item { y: 9, x: 11, color: "white", kind: ItemKind::Text("R") },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
