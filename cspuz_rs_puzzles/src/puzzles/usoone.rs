use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, NumSpaces, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_usoone(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<(Vec<Vec<Option<bool>>>, Vec<Vec<Option<bool>>>)> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    let is_lie = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);
    solver.add_answer_key_bool(is_lie);

    graph::active_vertices_connected_2d(&mut solver, !is_black);
    solver.add_expr(!is_black.conv2d_and((1, 2)));
    solver.add_expr(!is_black.conv2d_and((2, 1)));

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                if n >= 0 {
                    solver.add_expr(
                        is_lie
                            .at((y, x))
                            .iff(is_black.four_neighbors((y, x)).count_true().ne(n)),
                    );
                }
            } else {
                solver.add_expr(!is_lie.at((y, x)));
            }
        }
    }

    for room in graph::borders_to_rooms(borders) {
        solver.add_expr(is_lie.select(&room).count_true().eq(1));
    }

    solver
        .irrefutable_facts()
        .map(|f| (f.get(is_black), f.get(is_lie)))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(NumSpaces::new(4, 2)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (height, width) = problem.0.base_shape();
    problem_to_url_with_context(
        combinator(),
        "usoone",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["usoone"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 1, 0, 0, 0],
                [1, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 0],
                [0, 0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 0, 1, 0, 0],
                [1, 0, 1, 0, 0],
                [0, 0, 1, 0, 1],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ]),
        };
        let clues = vec![
            vec![Some(2), None, Some(2), None, None, None],
            vec![None, None, Some(2), Some(0), None, None],
            vec![None, Some(3), None, None, None, Some(2)],
            vec![None, None, Some(2), None, None, None],
            vec![None, None, None, Some(3), None, None],
        ];
        (borders, clues)
    }

    #[test]
    fn test_usoone_problem() {
        let (borders, clues) = problem_for_tests();
        let expected_black = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 1],
            [1, 0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0, 1],
            [0, 0, 0, 0, 0, 0],
        ]);
        let expected_lie = crate::util::tests::to_option_bool_2d([
            [1, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0],
        ]);
        assert_eq!(
            solve_usoone(&borders, &clues),
            Some((expected_black, expected_lie))
        );
    }

    #[test]
    fn test_usoone_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?usoone/6/5/4k544c8fo07ci2agdgccjd";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
