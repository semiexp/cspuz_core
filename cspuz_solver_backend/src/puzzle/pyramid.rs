use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{is_unique, Uniqueness};
use cspuz_rs_puzzles::puzzles::pyramid;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (is_shaded, clues, min_value, max_value) =
        pyramid::deserialize_problem(url).ok_or("invalid url")?;
    let ans = pyramid::solve_pyramid(&is_shaded, &clues, min_value, max_value);
    let size = clues.len();

    let mut board = Board::new(
        BoardKind::Empty,
        size,
        size * 2,
        ans.as_ref()
            .map_or(Uniqueness::NoAnswer, |a| is_unique(&a.concat())),
    );

    // Fills
    for y in 0..size {
        if is_shaded[y] {
            for x in 0..=y {
                board.push(Item::cell(
                    y,
                    size - y - 1 + 2 * x,
                    "#cccccc",
                    ItemKind::Fill,
                ));
                board.push(Item::cell(
                    y,
                    size - y - 1 + 2 * x + 1,
                    "#cccccc",
                    ItemKind::Fill,
                ));
            }
        }
    }

    // Borders
    for y in 0..=size {
        let start = if y == size { 0 } else { size - y - 1 };
        let end = if y == size { size * 2 } else { size + y + 1 };

        for x in start..end {
            board.push(Item {
                y: y * 2,
                x: x * 2 + 1,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }
    for y in 0..size {
        for x in 0..=(y + 1) {
            board.push(Item {
                y: y * 2 + 1,
                x: (size - y - 1 + 2 * x) * 2,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }

    for y in 0..size {
        for x in 0..=y {
            if let Some(n) = clues[y][x] {
                board.push(Item {
                    y: 2 * y + 1,
                    x: (size - y - 1 + 2 * x + 1) * 2,
                    color: "black",
                    kind: ItemKind::Num(n),
                });
            } else if let Some(ans) = &ans {
                if let Some(n) = ans[y][x] {
                    board.push(Item {
                        y: 2 * y + 1,
                        x: (size - y - 1 + 2 * x + 1) * 2,
                        color: "green",
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
            solve("https://opt-pan.github.io/penpa-edit/?m=solve&p=vVNNb5tAEL3zK6o5byUDBjd7c9O6l9T9chVFK2StbRKjgHEXaFws57dnZsC1F1ypl1ZoH4/HwDyWedtfRmfJSoR4BAMxEC4ePjJaXjDkRTods6RMY/lKjKtynRskQnyaTMS9TovYUW7kKHBBgIfLhei5nj6z4EXOvv4q9/Vcqugg6u8n+uZEv8k9DAOQvoBw2JzaqxBPWDDFgpEPUkEAaJJaCBhhqQL/TMCHuOtR8H0uUe7rUcRqCNTOlXvEO8YJo8c4Qzei9hnfMQ4YA8YbrnnPeMt4zThkDLlmRN/jOGoYiCvsR8sVI0baItwMKPJ0XlTmXi9jkLx9grVNlS1iA7I0Vaukeb5Nk41dljxschNfvEVivHq4VL/IzYpefnbjSaepJRQ/Km3sh5eJWaa2VJrEutbG5E+WkulybQkLXeLsFOtka78p3pS2gVLbFvWj7nTLTt98cGAHvJQnvFB4uMH7+krWY1F/aMbgOI2i/oKz9lHWUxo1BSB8/p/thNJP/U1v+T6x63aWBsinyEPkSO+QNvsyv2mUz1LVMwHU5y0/TRSy/CdabXzQ9TLPFvgxCui/7VqxqFb5Y3WcWprNcccpNWidkunWKdHGKbGu0/ZT/p3Tq+jQbP/gL9PdRPg/RG/XJiw3F0OG8jFntnoxUK3eyxTqvfRQw36AUL2QIVS7MUKpnyQUe2FC7Q95ord2I0WuuqmiVr1gUavzbKnIeQE=&a=RcvBCUAxDALQXTx7+kQ7TMj+a6T5ORSEh4iZxReEGCAU/Ab9eJt3s+nLMYV5VQM="),
            Board {
                kind: BoardKind::Empty,
                height: 4,
                width: 8,
                data: vec![
                    Item { y: 1, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 3, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 5, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 9, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 11, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 5, x: 13, color: "#cccccc", kind: ItemKind::Fill },
                    Item { y: 0, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Num(4) },
                    Item { y: 5, x: 12, color: "green", kind: ItemKind::Num(6) },
                    Item { y: 7, x: 2, color: "black", kind: ItemKind::Num(5) },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 14, color: "green", kind: ItemKind::Num(5) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
