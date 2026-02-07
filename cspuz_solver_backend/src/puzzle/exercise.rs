use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::exercise;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let has_block = exercise::deserialize_problem(url).ok_or("invalid url")?;
    let ans = exercise::solve_exercise(&has_block);

    let height = has_block.len();
    let width = has_block[0].len();
    let mut board = Board::new(
        BoardKind::Grid,
        height,
        width,
        check_uniqueness(&ans),
    );

    for y in 0..height {
        for x in 0..width {
            if has_block[y][x] {
                board.push(Item::cell(y, x, "#cccccc", ItemKind::Block));
            }
        }
    }

    if let Some(is_line) = ans {
        board.add_lines_irrefutable_facts(&is_line, "green", None);
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
            solve("https://opt-pan.github.io/penpa-edit/#m=solve&p=tVRNb+IwEL3nV6x8ngNxIEt9Y7tlL2z3i1WFoggZSEvUgLtO0lZG9Ld3PI5ETbyHXamKPHp5GTzPnnnUf1qpC0hhBMkYBhDjw9MUOB9DEg9pDbpnXjZVIT7ApG22SiMA+Dadwq2s6iLKuqw8yljMgHFcMctfzOyFiCSPDuanOJilyPIjmN8nOD7BX+KA8ZpiTHEhDownTGQJMCd2OWMQ51hhGGTTEJvwIBvcNwnuMApWSwM7oOgpSecU53gyMAnFzxQHFEcUZ5RzRfGG4iXFIcWUcj7au4mijHNqj3tG/4axLdgAVqtqWbf6Vq4LJqhxqB25fbtbFdqjKqUeqnLv55V3e6WL4CdLFpu7UP5K6c3Z7k+yqjzC3aBHrUu9rnyq0aX3LrVWTx6zk83WI1aywbGtt+WDv1Oxb3wBjfQlynt5Vm13OvMxYs+MVsbRMMDxhg/mQpgJmC/CswCYHzjgX4VZ2PnOGE0ONp6SOMKrE7yh7xZdOjIeIL7uMMIFQm8EzXeRmTkwW+cT/dpCtlOPKNXpsO9rtVvhYTL25jrcl7rdqPu2y6XpnTi5s4Dc5CTXQifXooBcewor17XxXeRe5EfXiMF//7m8k1ufO7MpHfQb0gHLIRu0Vsf33IV8z0e2YN9KyAbchOy5oZDqewrJnq2Q+4uz7K7n5rKqzv1lS/UsZku9dRn+axF6BQ=="),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 6,
                data: vec![
                    Item { y: 1, x: 3, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 1, x: 5, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 1, x: 9, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 3, x: 1, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 3, x: 3, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 3, x: 9, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 7, x: 5, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 9, x: 3, color: "#cccccc", kind: ItemKind::Block },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Line },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
