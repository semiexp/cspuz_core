use crate::penpa_editor::{decode_penpa_editor_url, Item, PenpaEditorPuzzle};
use crate::util;
use cspuz_rs::graph;
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_exercise(has_block: &[Vec<bool>]) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(has_block);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, is_line);

    let direction = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    let up = &(&is_line.vertical & &direction.vertical);
    let down = &(&is_line.vertical & !&direction.vertical);
    let left = &(&is_line.horizontal & &direction.horizontal);
    let right = &(&is_line.horizontal & !&direction.horizontal);

    for y in 0..h {
        for x in 0..w {
            let mut inbound = vec![];
            let mut outbound = vec![];
            if y > 0 {
                inbound.push(down.at((y - 1, x)));
                outbound.push(up.at((y - 1, x)));
            }
            if y < h - 1 {
                inbound.push(up.at((y, x)));
                outbound.push(down.at((y, x)));
            }
            if x > 0 {
                inbound.push(right.at((y, x - 1)));
                outbound.push(left.at((y, x - 1)));
            }
            if x < w - 1 {
                inbound.push(left.at((y, x)));
                outbound.push(right.at((y, x)));
            }
            solver.add_expr(count_true(&inbound).eq(is_passed.at((y, x)).ite(1, 0)));
            solver.add_expr(count_true(&outbound).eq(is_passed.at((y, x)).ite(1, 0)));
        }
    }

    for y in 0..h {
        for x in 0..w {
            if !has_block[y][x] {
                continue;
            }

            if y > 0 && (y == h - 1 || has_block[y + 1][x]) {
                solver.add_expr(!down.at((y - 1, x)));
            }
            if y < h - 1 && (y == 0 || has_block[y - 1][x]) {
                solver.add_expr(!up.at((y, x)));
            }
            if x > 0 && (x == w - 1 || has_block[y][x + 1]) {
                solver.add_expr(!right.at((y, x - 1)));
            }
            if x < w - 1 && (x == 0 || has_block[y][x - 1]) {
                solver.add_expr(!left.at((y, x)));
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if has_block[y][x] {
                continue;
            }

            let mut disturbances = vec![];

            if y >= 2 && has_block[y - 1][x] {
                disturbances.push(down.at((y - 2, x)));
            }
            if y < h - 2 && has_block[y + 1][x] {
                disturbances.push(up.at((y + 1, x)));
            }
            if x >= 2 && has_block[y][x - 1] {
                disturbances.push(right.at((y, x - 2)));
            }
            if x < w - 2 && has_block[y][x + 1] {
                disturbances.push(left.at((y, x + 1)));
            }

            solver.add_expr(count_true(disturbances).eq(is_passed.at((y, x)).ite(0, 1)));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = Vec<Vec<bool>>;

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let decoded = decode_penpa_editor_url(url).ok()?;
    #[allow(unreachable_patterns)]
    let decoded = match decoded {
        PenpaEditorPuzzle::Square(s) => s,
        _ => return None,
    };

    let mut ret = vec![vec![false; decoded.width()]; decoded.height()];
    for y in 0..decoded.height() {
        for x in 0..decoded.width() {
            for item in decoded.get_cell(y, x) {
                if let Item::Symbol(symbol) = item {
                    if symbol.name.starts_with("square_") {
                        ret[y][x] = true;
                    }
                }
            }
        }
    }

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        crate::util::tests::to_bool_2d([
            [0, 1, 1, 0, 1, 0],
            [1, 1, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0, 0],
        ])
    }

    #[test]
    fn test_exercise_problem() {
        let has_block = problem_for_tests();
        let ans = solve_exercise(&has_block);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 0, 0, 1, 0],
                [1, 0, 0, 0, 1],
                [0, 1, 1, 0, 0],
                [1, 0, 0, 1, 0],
                [0, 1, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 0, 0, 1, 1, 0],
                [1, 1, 0, 1, 0, 1],
                [1, 0, 0, 0, 0, 1],
                [0, 1, 0, 1, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_exercise_serializer() {
        let problem = problem_for_tests();
        let url = "https://opt-pan.github.io/penpa-edit/#m=solve&p=tVRNb+IwEL3nV6x8ngNxIEt9Y7tlL2z3i1WFoggZSEvUgLtO0lZG9Ld3PI5ETbyHXamKPHp5GTzPnnnUf1qpC0hhBMkYBhDjw9MUOB9DEg9pDbpnXjZVIT7ApG22SiMA+Dadwq2s6iLKuqw8yljMgHFcMctfzOyFiCSPDuanOJilyPIjmN8nOD7BX+KA8ZpiTHEhDownTGQJMCd2OWMQ51hhGGTTEJvwIBvcNwnuMApWSwM7oOgpSecU53gyMAnFzxQHFEcUZ5RzRfGG4iXFIcWUcj7au4mijHNqj3tG/4axLdgAVqtqWbf6Vq4LJqhxqB25fbtbFdqjKqUeqnLv55V3e6WL4CdLFpu7UP5K6c3Z7k+yqjzC3aBHrUu9rnyq0aX3LrVWTx6zk83WI1aywbGtt+WDv1Oxb3wBjfQlynt5Vm13OvMxYs+MVsbRMMDxhg/mQpgJmC/CswCYHzjgX4VZ2PnOGE0ONp6SOMKrE7yh7xZdOjIeIL7uMMIFQm8EzXeRmTkwW+cT/dpCtlOPKNXpsO9rtVvhYTL25jrcl7rdqPu2y6XpnTi5s4Dc5CTXQifXooBcewor17XxXeRe5EfXiMF//7m8k1ufO7MpHfQb0gHLIRu0Vsf33IV8z0e2YN9KyAbchOy5oZDqewrJnq2Q+4uz7K7n5rKqzv1lS/UsZku9dRn+axF6BQ==";
        assert_eq!(deserialize_problem(url).unwrap(), problem);
    }
}
