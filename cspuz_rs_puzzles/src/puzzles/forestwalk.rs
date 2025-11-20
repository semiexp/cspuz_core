use crate::util;
use cspuz_rs::complex_constraints::walk_line_size;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, Map, MultiDigit, Optionalize, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_forestwalk(
    forest: &[Vec<bool>],
    num: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(forest);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    // The network is conneceted
    {
        let (vertices, g) = is_line.representation();
        let line_graph = g.line_graph();

        graph::active_vertices_connected(&mut solver, &vertices, &line_graph);
    }

    let is_passed = &solver.bool_var_2d((h, w));
    let line_size = &walk_line_size(&mut solver, &is_line, forest, false);
    for y in 0..h {
        for x in 0..w {
            solver.add_expr((!is_passed.at((y, x))).imp(!(is_line.vertex_neighbors((y, x)).any())));

            if forest[y][x] {
                solver.add_expr(
                    is_passed
                        .at((y, x))
                        .imp(is_line.vertex_neighbors((y, x)).count_true().eq(3)),
                );
            } else {
                solver.add_expr(
                    is_passed
                        .at((y, x))
                        .imp(is_line.vertex_neighbors((y, x)).count_true().eq(2)),
                );

                if let Some(n) = num[y][x] {
                    if n >= 0 {
                        solver.add_expr(line_size.at((y, x)).eq(n));
                    }
                    solver.add_expr(is_passed.at((y, x)));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = (Vec<Vec<bool>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        ContextBasedGrid::new(Map::new(
            MultiDigit::new(2, 5),
            |x| Some(if x { 1 } else { 0 }),
            |x| Some(x == 1),
        )),
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem.0);
    problem_to_url_with_context_and_site(
        combinator(),
        "forestwalk",
        "https://pzprxs.vercel.app/p?",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["forestwalk"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        (
            crate::util::tests::to_bool_2d([
                [0, 0, 1, 0, 1, 0],
                [1, 0, 0, 0, 1, 0],
                [0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 1, 0],
            ]),
            vec![
                vec![Some(2), None, None, Some(3), None, None],
                vec![None, None, None, None, None, None],
                vec![None, None, None, None, Some(4), None],
                vec![None, None, None, None, None, None],
                vec![None, None, None, None, None, None],
            ],
        )
    }

    #[test]
    fn test_forestwalk_problem() {
        let (icebarn, num) = problem_for_tests();
        let ans = solve_forestwalk(&icebarn, &num);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 0, 0],
                [1, 0, 1, 0, 0],
                [0, 1, 1, 1, 1],
                [1, 1, 0, 1, 0],
                [0, 0, 1, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 1, 1, 0, 0],
                [1, 1, 0, 0, 0, 0],
                [1, 0, 0, 1, 0, 1],
                [0, 0, 1, 0, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_forestwalk_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?forestwalk/6/5/58gg1i2h3r4s";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
