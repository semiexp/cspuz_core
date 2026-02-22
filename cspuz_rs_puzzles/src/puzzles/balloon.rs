use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, MultiDigit, Optionalize, PrefixAndSuffix, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{count_true, IntVarArray1D, Solver};

pub fn solve_balloon(
    color: &[Vec<i32>],
    num: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(color);
    assert_eq!(util::infer_shape(num), (h, w));

    let mut solver = Solver::new();

    // connected cells within a region are also considered "connected by a line"
    let has_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&has_line.horizontal);
    solver.add_answer_key_bool(&has_line.vertical);

    let mut clues = vec![];
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = num[y][x] {
                clues.push((y, x, n));
            }
        }
    }

    // the id of the clue connected to each cell
    let region_id = &solver.int_var_2d((h, w), 0, clues.len() as i32 - 1);

    // grey (color=1) cells are divided into regions
    {
        for y in 0..h {
            for x in 0..w {
                if y < h - 1 {
                    if color[y][x] == 1 && color[y + 1][x] == 1 {
                        solver.add_expr(
                            has_line
                                .vertical
                                .at((y, x))
                                .iff(region_id.at((y, x)).eq(region_id.at((y + 1, x)))),
                        );
                    }
                }
                if x < w - 1 {
                    if color[y][x] == 1 && color[y][x + 1] == 1 {
                        solver.add_expr(
                            has_line
                                .horizontal
                                .at((y, x))
                                .iff(region_id.at((y, x)).eq(region_id.at((y, x + 1)))),
                        );
                    }
                }
            }
        }
    }

    // grey cells' region must be a rectangle
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            for cy in 0..=1 {
                for cx in 0..=1 {
                    if !(color[y + cy][x + cx] == 1
                        && color[y + (1 - cy)][x + cx] == 1
                        && color[y + cy][x + (1 - cx)] == 1)
                    {
                        continue;
                    }
                    if color[y + (1 - cy)][x + (1 - cx)] == 1 {
                        solver.add_expr(
                            (has_line.horizontal.at((y + cy, x))
                                & has_line.vertical.at((y, x + cx)))
                            .imp(
                                has_line.horizontal.at((y + (1 - cy), x))
                                    & has_line.vertical.at((y, x + (1 - cx))),
                            ),
                        );
                    } else {
                        solver.add_expr(
                            !(has_line.horizontal.at((y + cy, x))
                                & has_line.vertical.at((y, x + cx))),
                        );
                    }
                }
            }
        }
    }

    // line degree constraints
    let is_entrypoint = &solver.bool_var_2d((h, w));
    for y in 0..h {
        for x in 0..w {
            if color[y][x] == 1 {
                let mut adj = vec![];
                if y > 0 && color[y - 1][x] == 0 {
                    adj.push(has_line.vertical.at((y - 1, x)));
                }
                if y < h - 1 && color[y + 1][x] == 0 {
                    adj.push(has_line.vertical.at((y, x)));
                }
                if x > 0 && color[y][x - 1] == 0 {
                    adj.push(has_line.horizontal.at((y, x - 1)));
                }
                if x < w - 1 && color[y][x + 1] == 0 {
                    adj.push(has_line.horizontal.at((y, x)));
                }
                solver.add_expr(count_true(&adj).eq(is_entrypoint.at((y, x)).ite(1, 0)));
            } else {
                solver.add_expr(!is_entrypoint.at((y, x)));
                let deg = if num[y][x].is_some() { 1 } else { 2 };
                solver.add_expr(has_line.vertex_neighbors((y, x)).count_true().eq(deg));
            }
        }
    }

    let mut grey_cell_region_id = vec![];
    for y in 0..h {
        for x in 0..w {
            if color[y][x] == 1 {
                grey_cell_region_id.push(region_id.at((y, x)));
            }
        }
    }
    let grey_cell_region_id = IntVarArray1D::new(grey_cell_region_id);
    let region_id_flat = region_id.flatten();

    let (has_line_flat, aux_graph) = has_line.representation();
    for (i, &(y, x, n)) in clues.iter().enumerate() {
        if n != -1 {
            solver.add_expr(grey_cell_region_id.eq(i as i32).count_true().eq(n));
        }

        solver.add_expr(region_id.at((y, x)).eq(i as i32));
        solver.add_expr((is_entrypoint & region_id.eq(i as i32)).count_true().eq(1));

        graph::active_vertices_connected_via_active_edges(
            &mut solver,
            &region_id_flat.eq(i as i32),
            &has_line_flat,
            &aux_graph,
        );
    }

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 {
                solver.add_expr(
                    has_line
                        .vertical
                        .at((y, x))
                        .imp(region_id.at((y, x)).eq(region_id.at((y + 1, x)))),
                );
            }
            if x < w - 1 {
                solver.add_expr(
                    has_line
                        .horizontal
                        .at((y, x))
                        .imp(region_id.at((y, x)).eq(region_id.at((y, x + 1)))),
                );
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(has_line))
}

type Problem = (Vec<Vec<i32>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    PrefixAndSuffix::new(
        "a/",
        Size::new(Tuple2::new(
            ContextBasedGrid::new(MultiDigit::new(2, 5)),
            ContextBasedGrid::new(Choice::new(vec![
                Box::new(Optionalize::new(HexInt)),
                Box::new(Dict::new(Some(-1), ".")),
                Box::new(Spaces::new(None, 'g')),
            ])),
        )),
        "",
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem.0);
    problem_to_url_with_context_pzprxs(
        combinator(),
        "balloon",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["balloon"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        (
            vec![
                vec![0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0, 0],
                vec![1, 0, 1, 1, 1, 1],
                vec![1, 1, 0, 0, 0, 0],
            ],
            vec![
                vec![None, None, None, None, Some(-1), None],
                vec![None, None, None, None, Some(6), None],
                vec![None, None, None, None, None, None],
                vec![None, Some(2), None, None, None, None],
                vec![None, None, None, Some(1), Some(4), None],
            ],
        )
    }

    #[test]
    fn test_balloon_problem() {
        let (color, num) = problem_for_tests();
        let ans = solve_balloon(&color, &num);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 0, 1],
                [1, 1, 0, 1, 0],
                [1, 1, 0, 1, 1],
                [1, 0, 1, 1, 1],
                [0, 1, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 1, 0, 1],
                [1, 1, 1, 0, 0, 1],
                [0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_balloon_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?balloon/a/6/5/0e7ivgj.k6n2m14g";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
