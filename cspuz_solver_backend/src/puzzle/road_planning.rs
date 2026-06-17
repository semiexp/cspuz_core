use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::road_planning;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = road_planning::deserialize_problem(url).ok_or("invalid url")?;
    let ans = road_planning::solve_road_planning(&problem);

    let height = problem.len() - 1;
    let width = problem[0].len() - 1;
    let mut board = Board::new(
        BoardKind::ColoredGrid("#cccccc"),
        height,
        width,
        check_uniqueness(&ans),
    );

    board.add_borders_as_answer(ans.as_ref());

    for y in 0..=height {
        for x in 0..=width {
            if problem[y][x] {
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "white",
                    kind: ItemKind::SmallFilledCircle,
                });
                board.push(Item {
                    y: y * 2,
                    x: x * 2,
                    color: "black",
                    kind: ItemKind::SmallCircle,
                });
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
            solve("https://opt-pan.github.io/penpa-edit/#m=solve&p=tVRRb9owEH7Pr5j8fA+xgRb81nXrXjq2DqYKWREKkJaoAXdOslZG9Lf3fIkGJuZl2hT505fPF/vsuy/lrzo1GQygD70hxMDxEWIIfIT8ckAjbp9pXhWZ/ABXdbXWBgnAtzE8pEWZRaoNSiLFBAManCVvdvKmGAOeRDv7Q+7sXKpkD/bngQ4PdCJ3iGNCTjiTOzbsMak4sGVulkU2n0wYiATYsB+UeSzO6OFleHxuncEZfRTWBQ/oeIQbOoggnOI5wfYIPxHGhAPCW4r5THhPeE3YJ7ygmEt3U1GkRFMq92CRzjOsB149K3UxL2vzkC4zJqliQNq23iwy40mF1s9FvvXj8setNllwyonZ6jEUv9BmdbL6S1oUntA0oCc1N+hJlcm999QY/eIpm7Rae8IirbBdy3X+7K+UbSs/gSr1U0yf0pPdNocz7yP2ymgoAeICBHX2SNo7sF+k1/tg77C1v0o7c53duMAVWeGkaxms9B96T/OOXTcij5GPW450htTvOPtdKjsF5jb6SJ87yjb6N+ZK39H7Um8WeBrFju6jmSnrlX6q21juWvWqyXcSyLd3yNfRJl/HAvm65I7yvW0W+qfpjpJ9U4n4r/8r/8mar63btAkaDuWA51ANeqvVO/ZCvWMkt2HXS6gG7ITqqaNQ6poKxY6vUDtjLbfqqbtcVqcGc1t1POa2OrYZ/raIvQM="),
            Board {
                kind: BoardKind::ColoredGrid("#cccccc"),
                height: 4,
                width: 5,
                data: vec![
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 2, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 0, x: 2, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 0, x: 4, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 0, x: 4, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 4, x: 4, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 4, x: 4, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 4, x: 6, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 4, x: 6, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 4, x: 8, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 4, x: 8, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 4, x: 10, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 4, x: 10, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 6, x: 0, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 6, x: 0, color: "black", kind: ItemKind::SmallCircle },
                    Item { y: 8, x: 6, color: "white", kind: ItemKind::SmallFilledCircle },
                    Item { y: 8, x: 6, color: "black", kind: ItemKind::SmallCircle },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
