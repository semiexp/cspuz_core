use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context, Dict, Grid,
    NumSpaces, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_keywest(
    clues: &[Vec<Option<i32>>],
) -> Option<(Vec<Vec<Option<i32>>>, graph::BoolGridEdgesIrrefutableFacts)> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let cell_values = &solver.int_var_2d((h, w), 0, 4);
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_int(cell_values);
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let (is_line_flat, g) = is_line.representation();
    graph::active_vertices_connected(&mut solver, &is_line_flat, &(g.line_graph()));

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(cell_values.at((y, x)).eq(n));
            }
            solver.add_expr(
                cell_values
                    .at((y, x))
                    .eq(is_line.vertex_neighbors((y, x)).count_true()),
            );
        }
    }

    solver.add_expr(
        cell_values
            .slice((.., 1..))
            .ne(cell_values.slice((.., ..(w - 1)))),
    );
    solver.add_expr(
        cell_values
            .slice((1.., ..))
            .ne(cell_values.slice((..(h - 1), ..))),
    );

    solver
        .irrefutable_facts()
        .map(|f| (f.get(cell_values), f.get(is_line)))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(NumSpaces::new(4, 2)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem);
    problem_to_url_with_context_and_site(
        combinator(),
        "keywest",
        "https://pzprxs.vercel.app/p?",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["keywest"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![None, None, None, Some(1), None, None],
            vec![Some(0), Some(3), None, None, None, Some(2)],
            vec![None, None, Some(4), None, None, Some(1)],
            vec![None, None, None, None, None, None],
            vec![None, None, None, Some(3), None, None],
        ]
    }

    #[test]
    fn test_keywest_problem() {
        let problem = problem_for_tests();
        let ans = solve_keywest(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = (
            crate::util::tests::to_option_2d([
                [1, 2, 0, 1, 2, 1],
                [0, 3, 2, 0, 3, 2],
                [1, 2, 4, 2, 4, 1],
                [2, 3, 2, 1, 3, 2],
                [1, 2, 1, 3, 2, 1],
            ]),
            graph::GridEdges {
                horizontal: crate::util::tests::to_option_bool_2d([
                    [1, 0, 0, 1, 0],
                    [0, 1, 0, 0, 1],
                    [0, 1, 1, 1, 1],
                    [1, 1, 0, 0, 1],
                    [1, 0, 1, 1, 0],
                ]),
                vertical: crate::util::tests::to_option_bool_2d([
                    [0, 1, 0, 0, 1, 1],
                    [0, 1, 1, 0, 1, 0],
                    [1, 0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 1, 1],
                ]),
            },
        );
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_keywest_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?keywest/6/5/ib0dgcebmd";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
