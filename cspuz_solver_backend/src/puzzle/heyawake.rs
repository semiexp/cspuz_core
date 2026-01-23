pub fn solve(url: &str) -> Result<crate::board::Board, &'static str> {
    crate::puzzle::heyawake_internal::solve(url, false)
}

pub fn enumerate(
    url: &str,
    num_max_answers: usize,
) -> Result<(crate::board::Board, Vec<crate::board::Board>), &'static str> {
    crate::puzzle::heyawake_internal::enumerate_answers_heyawake(url, num_max_answers)
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
            solve("https://puzz.link/p?heyawake/6/6/aa66aapv0fu0g2i3k"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                    Item { y: 2, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Block },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 11, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 11, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 11, x: 11, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(3) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
