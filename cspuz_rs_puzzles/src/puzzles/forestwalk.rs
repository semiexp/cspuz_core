use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, HexInt, Map, MultiDigit, Optionalize, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{count_true, BoolVar, Solver};

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

    let rank = &solver.int_var_2d((h, w), 0, (h * w - 1) as i32);
    let size = &solver.int_var_2d((h, w), 0, (h * w - 1) as i32);
    solver.add_expr(rank.le(size));

    for y in 0..h {
        for x in 0..w {
            solver.add_expr((!is_passed.at((y, x))).imp(!(is_line.vertex_neighbors((y, x)).any())));

            if forest[y][x] {
                solver.add_expr(
                    is_passed
                        .at((y, x))
                        .imp(is_line.vertex_neighbors((y, x)).count_true().eq(3)),
                );
                solver.add_expr(rank.at((y, x)).eq(0));
                solver.add_expr(size.at((y, x)).eq(0));
            } else {
                solver.add_expr(
                    is_passed
                        .at((y, x))
                        .imp(is_line.vertex_neighbors((y, x)).count_true().eq(2)),
                );

                if let Some(n) = num[y][x] {
                    solver.add_expr(size.at((y, x)).eq(n - 1));
                    solver.add_expr(is_passed.at((y, x)));
                }

                let mut lower = vec![];
                let mut upper = vec![];

                let mut check_neighbor = |e: &BoolVar, y2: usize, x2: usize| {
                    if forest[y2][x2] {
                        solver.add_expr(
                            e.imp(rank.at((y, x)).eq(0) | rank.at((y, x)).eq(size.at((y, x)))),
                        );
                        lower.push(e & rank.at((y, x)).eq(0));
                        upper.push(e & rank.at((y, x)).eq(size.at((y, x))));
                    } else {
                        solver.add_expr(e.imp(
                            rank.at((y2, x2)).eq(rank.at((y, x)) + 1)
                                | rank.at((y2, x2)).eq(rank.at((y, x)) - 1),
                        ));
                        solver.add_expr(e.imp(size.at((y2, x2)).eq(size.at((y, x)))));
                        lower.push(e & rank.at((y2, x2)).eq(rank.at((y, x)) - 1));
                        upper.push(e & rank.at((y2, x2)).eq(rank.at((y, x)) + 1));
                    }
                };

                if y > 0 {
                    check_neighbor(&is_line.vertical.at((y - 1, x)), y - 1, x);
                }
                if y < h - 1 {
                    check_neighbor(&is_line.vertical.at((y, x)), y + 1, x);
                }
                if x > 0 {
                    check_neighbor(&is_line.horizontal.at((y, x - 1)), y, x - 1);
                }
                if x < w - 1 {
                    check_neighbor(&is_line.horizontal.at((y, x)), y, x + 1);
                }

                // NOTE: size[(y, x)] can be 0
                solver.add_expr(is_passed.at((y, x)).imp(count_true(&lower).ge(1)));
                solver.add_expr(is_passed.at((y, x)).imp(count_true(&upper).ge(1)));
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
