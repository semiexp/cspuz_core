use crate::puzzles::loop_common::add_full_loop_constraints;
use crate::puzzles::walk_common::{merge_walk_answers, walk_not_passing_colored_cell};
use crate::util;
use cspuz_rs::complex_constraints::walk_line_size;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, Map, MultiDigit, Optionalize, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{Solver, FALSE, TRUE};

pub fn solve_firewalk(
    full: bool,
    fire_cell: &[Vec<bool>],
    num: &[Vec<Option<i32>>],
) -> Option<(graph::BoolGridEdgesIrrefutableFacts, Vec<Vec<Option<bool>>>)> {
    let (h, w) = util::infer_shape(fire_cell);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_expr(is_line.horizontal.any() | is_line.vertical.any());
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    if full {
        add_full_loop_constraints(&mut solver, is_line, h - 1, w - 1);
    }

    for y in 0..h {
        for x in 0..w {
            if num[y][x].is_some() {
                solver.add_expr(is_line.vertex_neighbors((y, x)).any());
            }
        }
    }
    let is_inner = &solver.bool_var_2d((h - 1, w - 1));
    for y in 0..h {
        for x in 0..(w - 1) {
            let up = if y == 0 {
                FALSE
            } else {
                is_inner.at((y - 1, x)).expr()
            };

            let down = if y == h - 1 {
                FALSE
            } else {
                is_inner.at((y, x)).expr()
            };

            solver.add_expr(!(up ^ down ^ is_line.horizontal.at((y, x))));
        }
    }
    for y in 0..(h - 1) {
        for x in 0..w {
            let left = if x == 0 {
                FALSE
            } else {
                is_inner.at((y, x - 1)).expr()
            };

            let right = if x == w - 1 {
                FALSE
            } else {
                is_inner.at((y, x)).expr()
            };

            solver.add_expr(!(left ^ right ^ is_line.vertical.at((y, x))));
        }
    }

    // false:
    // + | +
    //  0
    // -   -
    //    1
    // + | +
    //
    // true:
    // + | +
    //    0
    // -   -
    //  1
    // + | +
    let fire_cell_mode = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(fire_cell_mode);

    let mut cell_ids = vec![vec![vec![]; w]; h];
    let mut last_cell_id = 0;
    for y in 0..h {
        for x in 0..w {
            cell_ids[y][x].push(last_cell_id);
            last_cell_id += 1;
            if fire_cell[y][x] {
                cell_ids[y][x].push(last_cell_id);
                last_cell_id += 1;

                solver.add_expr(
                    (!(is_line.vertex_neighbors((y, x)).any())).imp(!fire_cell_mode.at((y, x))),
                );
                if 0 < y && y < h - 1 && 0 < x && x < w - 1 {
                    solver.add_expr(
                        is_line
                            .vertex_neighbors((y, x))
                            .all()
                            .imp(fire_cell_mode.at((y, x)).iff(is_inner.at((y - 1, x - 1)))),
                    );
                }
            } else {
                solver.add_expr(!fire_cell_mode.at((y, x)));
            }
        }
    }

    let mut aux_graph = graph::Graph::new(last_cell_id);
    let mut loop_edges = vec![];
    for y in 0..h {
        for x in 0..(w - 1) {
            for i in 0..cell_ids[y][x].len() {
                for j in 0..cell_ids[y][x + 1].len() {
                    let mut condition = TRUE;

                    if cell_ids[y][x].len() == 2 {
                        if i == 0 {
                            condition = condition & fire_cell_mode.at((y, x));
                        } else {
                            condition = condition & !fire_cell_mode.at((y, x));
                        }
                    }
                    if cell_ids[y][x + 1].len() == 2 {
                        if j == 0 {
                            condition = condition & !fire_cell_mode.at((y, x + 1));
                        } else {
                            condition = condition & fire_cell_mode.at((y, x + 1));
                        }
                    }

                    aux_graph.add_edge(cell_ids[y][x][i], cell_ids[y][x + 1][j]);
                    loop_edges.push(condition & is_line.horizontal.at((y, x)));
                }
            }
        }
    }
    for y in 0..(h - 1) {
        for x in 0..w {
            for i in 0..cell_ids[y][x].len() {
                for j in 0..cell_ids[y + 1][x].len() {
                    if cell_ids[y][x].len() == 2 {
                        if i != 1 {
                            continue;
                        }
                    }
                    if cell_ids[y + 1][x].len() == 2 {
                        if j != 0 {
                            continue;
                        }
                    }

                    aux_graph.add_edge(cell_ids[y][x][i], cell_ids[y + 1][x][j]);
                    loop_edges.push(is_line.vertical.at((y, x)).expr());
                }
            }
        }
    }

    graph::active_edges_single_cycle(&mut solver, &loop_edges, &aux_graph);

    let line_size = &walk_line_size(&mut solver, &is_line, fire_cell, false);
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = num[y][x] {
                solver.add_expr(line_size.at((y, x)).eq(n));
            }
        }
    }

    let ans1 = solver
        .irrefutable_facts()
        .map(|f| (f.get(is_line), f.get(fire_cell_mode)));

    let ans2 = walk_not_passing_colored_cell(full, fire_cell, num);

    match (ans1, ans2) {
        (Some((edges, modes)), Some(edges2)) => {
            let merged_edges = merge_walk_answers(Some(edges), Some(edges2))?;
            Some((merged_edges, modes))
        }
        (Some((edges, modes)), None) => Some((edges, modes)),
        (None, Some(edges2)) => Some((edges2, vec![vec![Some(false); w]; h])),
        (None, None) => None,
    }
}

type Problem = (bool, (Vec<Vec<bool>>, Vec<Vec<Option<i32>>>));

fn combinator() -> impl Combinator<Problem> {
    Tuple2::new(
        Choice::new(vec![
            Box::new(Dict::new(true, "f/")),
            Box::new(Dict::new(false, "")),
        ]),
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
        )),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1 .0.len();
    let width = problem.1 .0[0].len();
    problem_to_url_with_context_and_site(
        combinator(),
        "firewalk",
        "https://pzprxs.vercel.app/p?",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["firewalk"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        (
            false,
            (
                crate::util::tests::to_bool_2d([
                    [0, 0, 1, 0, 0, 1],
                    [0, 1, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 1, 0, 0, 0],
                ]),
                vec![
                    vec![None, Some(1), None, None, None, None],
                    vec![None, None, None, None, None, Some(8)],
                    vec![Some(3), None, Some(6), None, None, None],
                    vec![None, None, None, None, None, None],
                    vec![None, None, None, None, None, None],
                ],
            ),
        )
    }

    fn problem_for_tests2() -> Problem {
        (
            true,
            (
                crate::util::tests::to_bool_2d([[0, 0, 0], [0, 0, 0]]),
                vec![vec![None, None, None], vec![None, None, None]],
            ),
        )
    }

    #[test]
    fn test_firewalk_problem1() {
        let (full, (fire_cell, num)) = problem_for_tests1();
        let ans = solve_firewalk(full, &fire_cell, &num);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = (
            graph::GridEdges {
                horizontal: crate::util::tests::to_option_bool_2d([
                    [0, 1, 0, 0, 0],
                    [1, 1, 1, 1, 1],
                    [1, 0, 1, 1, 0],
                    [0, 0, 1, 1, 0],
                    [0, 0, 1, 1, 1],
                ]),
                vertical: crate::util::tests::to_option_bool_2d([
                    [0, 1, 1, 0, 0, 0],
                    [1, 1, 1, 0, 0, 1],
                    [0, 0, 0, 0, 1, 1],
                    [0, 0, 1, 0, 0, 1],
                ]),
            },
            crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 0, 0, 0],
                [0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0],
            ]),
        );
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_firewalk_problem2() {
        let (full, (fire_cell, num)) = problem_for_tests2();
        let ans = solve_firewalk(full, &fire_cell, &num);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = (
            graph::GridEdges {
                horizontal: crate::util::tests::to_option_bool_2d([[1, 1], [1, 1]]),
                vertical: crate::util::tests::to_option_bool_2d([[1, 0, 1]]),
            },
            crate::util::tests::to_option_bool_2d([[0, 0, 0], [0, 0, 0]]),
        );
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_firewalk_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://pzprxs.vercel.app/p?firewalk/6/5/4m0008g1o83g6u";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://pzprxs.vercel.app/p?firewalk/f/3/2/00l";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
