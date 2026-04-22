use crate::util;
use cspuz_rs::different_shape::DifferentShape;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Choice, Choice2, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, Map, MultiDigit, Optionalize, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{any, Solver};

pub fn solve_narrowfence(
    clues: &[Vec<Option<i32>>],
    holes: &Option<Vec<Vec<bool>>>,
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);

    let mut max_clue = 0;
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                max_clue = max_clue.max(n);
            }
        }
    }
    if max_clue == 0 {
        return None;
    }

    let mut clue_groups = vec![vec![]; (max_clue + 1) as usize];
    let mut id = 0;
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                clue_groups[n as usize].push((y, x, id));
                id += 1;
            }
        }
    }

    let block_id = &solver.int_var_2d((h, w), if holes.is_some() { -1 } else { 0 }, id - 1);
    solver.add_expr(
        block_id
            .slice((.., ..(w - 1)))
            .ne(block_id.slice((.., 1..)))
            .iff(&is_border.vertical),
    );
    solver.add_expr(
        block_id
            .slice((..(h - 1), ..))
            .ne(block_id.slice((1.., ..)))
            .iff(&is_border.horizontal),
    );

    if let Some(holes) = holes {
        for y in 0..h {
            for x in 0..w {
                if holes[y][x] {
                    solver.add_expr(block_id.at((y, x)).eq(-1));
                } else {
                    solver.add_expr(block_id.at((y, x)).ne(-1));
                }
            }
        }
    }

    for i in 0..=max_clue as usize {
        for &(y, x, id) in &clue_groups[i] {
            solver.add_expr(block_id.at((y, x)).eq(id));
        }
    }

    for i in 0..=max_clue as usize {
        if clue_groups[i].is_empty() {
            continue;
        }

        let mut indicators = vec![];
        for &(_, _, id) in &clue_groups[i] {
            indicators.push(block_id.eq(id));
        }

        for ind in &indicators {
            graph::active_vertices_connected_2d(&mut solver, ind);
        }

        // do not allow 2x2 blocks in the same clue group
        {
            let mut in_group = indicators[0].clone();
            for j in 1..indicators.len() {
                in_group = in_group | indicators[j].clone();
            }
            solver.add_expr(!(in_group.conv2d_and((2, 2))));
        }

        // not rectangles
        {
            for ind in &indicators {
                let mut cand = vec![];
                for y in 0..(h - 1) {
                    for x in 0..(w - 1) {
                        cand.push(ind.slice((y..(y + 2), x..(x + 2))).count_true().eq(3));
                    }
                }
                solver.add_expr(any(cand));
            }
        }

        for u in 0..indicators.len() {
            for v in (u + 1)..indicators.len() {
                let mut c = vec![];
                for y in 0..h {
                    for x in 0..w {
                        c.push(indicators[u].at((y, x)));
                    }
                }
                for y in 0..h {
                    for x in 0..w {
                        c.push(indicators[v].at((y, x)));
                    }
                }

                #[cfg(not(test))]
                {
                    solver.add_custom_constraint(Box::new(DifferentShape::new(h, w)), c);
                }

                #[cfg(test)]
                {
                    solver.add_custom_constraint(
                        Box::new(util::tests::ReasonVerifier::new(
                            DifferentShape::new(h, w),
                            DifferentShape::new(h, w),
                        )),
                        c,
                    );
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(&is_border))
}

type Problem = (Vec<Vec<Option<i32>>>, Option<Vec<Vec<bool>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
        Choice2::new(
            Optionalize::new(ContextBasedGrid::new(Map::new(
                MultiDigit::new(2, 5),
                |x: bool| Some(if x { 1 } else { 0 }),
                |n: i32| Some(n == 1),
            ))),
            Dict::new(None, ""),
        ),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.len();
    let width = problem.0[0].len();

    problem_to_url_with_context_pzprxs(
        combinator(),
        "narrow",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["narrow"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        (
            vec![
                vec![Some(1), Some(1), None, Some(1), None],
                vec![None, None, None, None, None],
                vec![None, None, Some(2), Some(2), None],
                vec![None, None, None, None, None],
            ],
            None,
        )
    }

    #[test]
    fn test_narrowfence_problem() {
        let (clues, is_hole) = problem_for_tests();
        let ans = solve_narrowfence(&clues, &is_hole);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolInnerGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 1, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 1, 0],
                [1, 1, 1, 1],
                [1, 0, 1, 1],
                [0, 0, 1, 0],
            ]),
        };

        assert_eq!(ans, expected);
    }

    #[test]
    fn test_narrowfence_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?narrow/5/4/11g1n22l";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
