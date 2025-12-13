use crate::puzzles::walk_common::{merge_walk_answers, walk_not_passing_colored_cell};
use crate::util;
use cspuz_rs::complex_constraints::walk_line_size;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, HexInt, Map, MultiDigit, Optionalize, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_waterwalk(
    water: &[Vec<bool>],
    num: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(water);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_expr(is_line.horizontal.any() | is_line.vertical.any());
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, &is_line);
    let line_size = &walk_line_size(&mut solver, &is_line, water, true);
    for y in 0..h {
        for x in 0..w {
            solver.add_expr((!is_passed.at((y, x))).imp(!(is_line.vertex_neighbors((y, x)).any())));

            if water[y][x] {
                solver.add_expr(is_passed.at((y, x)).imp(line_size.at((y, x)).le(2)));
            } else {
                if let Some(n) = num[y][x] {
                    if n >= 0 {
                        solver.add_expr(line_size.at((y, x)).eq(n));
                    }
                    solver.add_expr(is_passed.at((y, x)));
                }
            }
        }
    }

    let ans1 = solver.irrefutable_facts().map(|f| f.get(is_line));
    let ans2 = walk_not_passing_colored_cell(water, num);
    merge_walk_answers(ans1, ans2)
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
    problem_to_url_with_context(
        combinator(),
        "waterwalk",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["waterwalk"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        (
            crate::util::tests::to_bool_2d([
                [0, 0, 1, 1, 1, 0],
                [1, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 1, 0],
                [1, 0, 0, 1, 0, 0],
                [0, 0, 0, 1, 0, 0],
            ]),
            vec![
                vec![Some(2), None, None, None, None, None],
                vec![None, Some(3), None, None, Some(1), None],
                vec![None, None, None, None, None, None],
                vec![None, None, None, None, Some(1), None],
                vec![Some(3), None, None, None, None, None],
            ],
        )
    }

    #[test]
    fn test_waterwalk_problem() {
        let (water, num) = problem_for_tests();
        let ans = solve_waterwalk(&water, &num);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 0, 1],
                [1, 1, 0, 0, 0],
                [0, 1, 0, 0, 0],
                [1, 0, 0, 1, 0],
                [1, 1, 1, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 1, 1, 1],
                [0, 0, 1, 1, 1, 1],
                [0, 1, 0, 1, 1, 1],
                [1, 0, 0, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_waterwalk_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?waterwalk/6/5/786a842l3h1q1g3k";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
