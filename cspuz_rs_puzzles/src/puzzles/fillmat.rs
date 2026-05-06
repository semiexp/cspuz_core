use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_fillmat(
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((h, w), 1, 4);
    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);
    solver.add_expr(
        num.slice((.., ..(w - 1)))
            .ne(num.slice((.., 1..)))
            .iff(&is_border.vertical),
    );
    solver.add_expr(
        num.slice((..(h - 1), ..))
            .ne(num.slice((1.., ..)))
            .iff(&is_border.horizontal),
    );

    // Same numbers cannot be diagonally adjacent. Takes care of the shape constraint
    for i in 1..=4 {
        solver.add_expr(!(num.eq(i).slice((..(h - 1), ..(w - 1))) & num.eq(i).slice((1.., 1..))));
        solver.add_expr(!(num.eq(i).slice((..(h - 1), 1..)) & num.eq(i).slice((1.., ..(w - 1)))));
    }
    graph::graph_division_2d(&mut solver, num, &is_border);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    solver.add_expr(num.at((y, x)).eq(n));
                }
            }
        }
    }

    // No 4 intersections
    for y in 1..h {
        for x in 1..w {
            let left = &is_border.horizontal.at((y - 1, x - 1));
            let right = &is_border.horizontal.at((y - 1, x));
            let up = &is_border.vertical.at((y - 1, x - 1));
            let down = &is_border.vertical.at((y, x - 1));
            solver.add_expr(!(left & right & up & down));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(&is_border))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Spaces::new(None, 'a')),
        Box::new(Dict::new(Some(-1), ".")),
        Box::new(Dict::new(Some(1), "1")),
        Box::new(Dict::new(Some(2), "2")),
        Box::new(Dict::new(Some(3), "3")),
        Box::new(Dict::new(Some(4), "4")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "fillmat", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["fillmat"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(3), None, None, Some(3), None],
            vec![None, None, None, None, None],
            vec![None, None, Some(1), None, None],
            vec![None, None, None, None, None],
            vec![None, Some(1), None, None, Some(4)],
        ]
    }

    #[test]
    fn test_fillmat_problem() {
        let problem = problem_for_tests();
        let ans = solve_fillmat(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        #[rustfmt::skip]
        let expected = graph::BoolInnerGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 1, 1],
                [0, 0, 1, 1, 0],
                [1, 0, 1, 0, 0],
                [0, 1, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 1, 0, 0],
                [1, 1, 0, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_fillmat_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?fillmat/5/5/3b3h1h1b4";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
