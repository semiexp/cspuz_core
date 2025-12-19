use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context, Dict, Grid,
    HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::{Solver, FALSE};

pub fn solve_lapaz(
    clues: &[Vec<Option<i32>>],
) -> Option<(Vec<Vec<Option<bool>>>, graph::BoolGridEdgesIrrefutableFacts)> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();

    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    solver.add_expr(!(is_black.conv2d_and((2, 1))));
    solver.add_expr(!(is_black.conv2d_and((1, 2))));

    let connected = graph::GridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&connected.horizontal);
    solver.add_answer_key_bool(&connected.vertical);

    solver.add_expr(!connected.horizontal.conv2d_and((1, 2)));
    solver.add_expr(!connected.vertical.conv2d_and((2, 1)));

    for y in 0..h {
        for x in 0..w {
            let lr = &(connected.horizontal.at_offset((y, x), (0, -1), FALSE)
                | connected.horizontal.at_offset((y, x), (0, 0), FALSE));
            let ud = &(connected.vertical.at_offset((y, x), (-1, 0), FALSE)
                | connected.vertical.at_offset((y, x), (0, 0), FALSE));
            solver.add_expr(is_black.at((y, x)).imp((!lr) & (!ud)));
            solver.add_expr((!is_black.at((y, x))).imp(lr ^ ud));

            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                if n >= 0 {
                    solver.add_expr(lr.imp(is_black.slice_fixed_y((y, ..)).count_true().eq(n)));
                    solver.add_expr(ud.imp(is_black.slice_fixed_x((.., x)).count_true().eq(n)));
                }
            }
        }
    }

    solver
        .irrefutable_facts()
        .map(|f| (f.get(is_black), f.get(&connected)))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem);
    problem_to_url_with_context_and_site(
        combinator(),
        "lapaz",
        "https://pzprxs.vercel.app/p?",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["lapaz"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(2), None, None, None, None],
            vec![Some(1), None, None, None, None],
            vec![None, None, None, Some(-1), None],
            vec![None, None, None, None, Some(1)],
        ]
    }

    #[test]
    fn test_lapaz_problem() {
        let problem = problem_for_tests();
        let (is_black, is_connected) = solve_lapaz(&problem).unwrap();
        let expected_is_black = crate::util::tests::to_option_bool_2d([
            [0, 0, 1, 0, 1],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ]);
        let expected_connected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [0, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 0, 0, 1, 0],
                [1, 0, 0, 0, 1],
                [0, 0, 0, 0, 0],
            ]),
        };
        assert_eq!(is_black, expected_is_black);
        assert_eq!(is_connected, expected_connected);
    }

    #[test]
    fn test_lapaz_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?lapaz/5/4/2j1m.k1";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
