use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, Spaces,
};
use cspuz_rs::solver::{BoolVar, Solver};

pub fn solve_school_trip(
    clues: &[Vec<Option<i32>>],
) -> Option<(
    Vec<Vec<Option<bool>>>,
    Vec<Vec<Option<bool>>>,
    graph::BoolGridEdgesIrrefutableFacts,
)> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();

    let is_black = &solver.bool_var_2d((h, w));
    let is_futon = &solver.bool_var_2d((h, w));
    let is_pillow = &solver.bool_var_2d((h, w));

    solver.add_answer_key_bool(is_black);
    solver.add_answer_key_bool(is_pillow);
    graph::active_vertices_connected_2d(&mut solver, is_black);

    // we draw a virtual line between two cells in a futon
    let connected = graph::GridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&connected.horizontal);
    solver.add_answer_key_bool(&connected.vertical);

    solver.add_expr(!(is_black.conv2d_and((2, 2))));
    solver.add_expr(is_pillow.imp(is_futon));

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                solver.add_expr(!is_futon.at((y, x)));
                solver.add_expr(!connected.vertex_neighbors((y, x)));

                if n >= 0 {
                    solver.add_expr(is_pillow.four_neighbors((y, x)).count_true().eq(n));
                }
                continue;
            }

            solver.add_expr(is_black.at((y, x)) ^ is_futon.at((y, x)));

            solver.add_expr(
                connected
                    .vertex_neighbors((y, x))
                    .count_true()
                    .eq(is_futon.at((y, x)).ite(1, 0)),
            );
            if y < h - 1 {
                solver.add_expr(is_pillow.at((y, x)).imp(!connected.vertical.at((y, x))));
            }
        }
    }

    let mut futon_condition = |cond: BoolVar, a, b| {
        let cells = is_black
            .four_neighbor_indices(a)
            .into_iter()
            .chain(is_black.four_neighbor_indices(b).into_iter())
            .filter(|&c| c != a && c != b)
            .collect::<Vec<_>>();
        solver.add_expr(cond.imp(is_black.select(cells).any()));
        solver.add_expr(cond.imp(is_pillow.at(a) ^ is_pillow.at(b)));
    };

    for y in 0..h {
        for x in 0..(w - 1) {
            futon_condition(connected.horizontal.at((y, x)), (y, x), (y, x + 1));
        }
    }
    for y in 0..(h - 1) {
        for x in 0..w {
            futon_condition(connected.vertical.at((y, x)), (y, x), (y + 1, x));
        }
    }

    solver
        .irrefutable_facts()
        .map(|f| (f.get(is_black), f.get(is_pillow), f.get(&connected)))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Dict::new(Some(0), "0")),
        Box::new(Dict::new(Some(1), "1")),
        Box::new(Dict::new(Some(2), "2")),
        Box::new(Dict::new(Some(3), "3")),
        Box::new(Dict::new(Some(4), "4")),
        Box::new(Dict::new(Some(-1), "5")),
        Box::new(Spaces::new(None, '6')),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "shugaku", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["shugaku"], url)
}

#[cfg(test)]
mod tests {
    use cspuz_rs::graph::BoolGridEdgesIrrefutableFacts;

    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(2), None, None, Some(2), None, None],
            vec![None, None, None, None, None, None],
            vec![Some(1), None, None, None, None, None],
            vec![None, None, None, Some(0), None, None],
            vec![Some(0), None, None, None, None, Some(0)],
        ]
    }

    #[test]
    fn test_school_trip_problem() {
        let problem = problem_for_tests();
        let (h, w) = util::infer_shape(&problem);
        assert_eq!((h, w), (5, 6));
        let (is_black, is_pillow, is_connected) = solve_school_trip(&problem).unwrap();
        let expected_is_black = crate::util::tests::to_option_bool_2d([
            [0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 1],
            [0, 1, 1, 0, 0, 1],
            [0, 0, 1, 0, 1, 1],
            [0, 1, 1, 1, 1, 0],
        ]);
        let expected_is_pillow = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 1, 0],
            [0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
        ]);
        let expected_connected = BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 0, 1],
                [1, 0, 0, 1, 0],
                [0, 0, 0, 1, 0],
                [1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
            ]),
        };
        assert_eq!(is_black, expected_is_black);
        assert_eq!(is_pillow, expected_is_pillow);
        assert_eq!(is_connected, expected_connected);
    }

    #[test]
    fn test_school_trip_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?shugaku/6/5/272d1d07090";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
