use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

mod propagator;

pub fn enumerate_answers_numlin(
    clues: &[Vec<Option<i32>>],
    num_max_answers: usize,
) -> Vec<graph::BoolGridEdgesModel> {
    enumerate_answers_numlin_impl(clues, num_max_answers, true)
}

pub fn enumerate_answers_numlin_impl(
    clues: &[Vec<Option<i32>>],
    num_max_answers: usize,
    use_propagator: bool,
) -> Vec<graph::BoolGridEdgesModel> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();

    let is_line = graph::GridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    for y in 0..h {
        for x in 0..w {
            if clues[y][x].is_some() {
                solver.add_expr(is_line.vertex_neighbors((y, x)).count_true().eq(1));
            } else {
                solver.add_expr(
                    is_line.vertex_neighbors((y, x)).count_true().eq(0)
                        | is_line.vertex_neighbors((y, x)).count_true().eq(2),
                );
            }
        }
    }

    // forbid trivial detour
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            solver.add_expr(is_line.cell_neighbors((y, x)).count_true().le(2));
        }
    }

    // L-shape canonization
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            if clues[y + 1][x + 1].is_none() {
                if y == h - 2 || x == w - 2 {
                    solver.add_expr(!(is_line.horizontal.at((y, x)) & is_line.vertical.at((y, x))));
                } else {
                    solver.add_expr(
                        (is_line.horizontal.at((y, x)) & is_line.vertical.at((y, x))).imp(
                            is_line.horizontal.at((y + 1, x + 1))
                                & is_line.vertical.at((y + 1, x + 1)),
                        ),
                    );
                }
            }

            if clues[y + 1][x].is_none() {
                if y == h - 2 || x == 0 {
                    solver.add_expr(
                        !(is_line.horizontal.at((y, x)) & is_line.vertical.at((y, x + 1))),
                    );
                } else {
                    solver.add_expr(
                        (is_line.horizontal.at((y, x)) & is_line.vertical.at((y, x + 1))).imp(
                            is_line.horizontal.at((y + 1, x - 1)) & is_line.vertical.at((y + 1, x)),
                        ),
                    );
                }
            }
        }
    }

    let mut max_clue = 0;
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                max_clue = max_clue.max(n);
            }
        }
    }

    let cell_clue_id = &solver.int_var_2d((h, w), 0, max_clue);
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(cell_clue_id.at((y, x)).eq(n));
            }
        }
    }

    solver.add_expr(
        is_line.horizontal.imp(
            cell_clue_id
                .slice((.., 1..))
                .eq(cell_clue_id.slice((.., ..(w - 1)))),
        ),
    );
    solver.add_expr(
        is_line.vertical.imp(
            cell_clue_id
                .slice((1.., ..))
                .eq(cell_clue_id.slice((..(h - 1), ..))),
        ),
    );

    if use_propagator {
        let mut values = vec![];
        for y in 0..h {
            for x in 0..(w - 1) {
                values.push(is_line.horizontal.at((y, x)).expr());
            }
        }
        for y in 0..(h - 1) {
            for x in 0..w {
                values.push(is_line.vertical.at((y, x)).expr());
            }
        }
        for y in 0..h {
            for x in 0..w {
                values.push(is_line.vertex_neighbors((y, x)).any());
            }
        }

        solver.add_custom_constraint(
            Box::new(propagator::PathPropagator::new(h, w, clues)),
            values,
        );
    } else {
        #[cfg(test)]
        {
            // acyclic
            let mut graph = graph::Graph::new((h - 1) * (w - 1) + 1);
            let outside = (h - 1) * (w - 1);
            let mut indicator = vec![];

            for y in 0..h {
                for x in 0..(w - 1) {
                    let v1 = if y == 0 {
                        outside
                    } else {
                        (y - 1) * (w - 1) + x
                    };
                    let v2 = if y == h - 1 { outside } else { y * (w - 1) + x };
                    graph.add_edge(v1, v2);
                    indicator.push(!is_line.horizontal.at((y, x)));
                }
            }
            for y in 0..(h - 1) {
                for x in 0..w {
                    let v1 = if x == 0 {
                        outside
                    } else {
                        y * (w - 1) + (x - 1)
                    };
                    let v2 = if x == w - 1 { outside } else { y * (w - 1) + x };
                    graph.add_edge(v1, v2);
                    indicator.push(!is_line.vertical.at((y, x)));
                }
            }
            let is_active = vec![cspuz_rs::solver::TRUE; graph.n_vertices()];
            graph::active_vertices_connected_via_active_edges(
                &mut solver,
                &is_active,
                &indicator,
                &graph,
            );

            // no detour
            let is_passed = &solver.bool_var_2d((h, w));
            for y in 0..h {
                for x in 0..w {
                    solver.add_expr(
                        is_passed
                            .at((y, x))
                            .iff(is_line.vertex_neighbors((y, x)).any()),
                    );
                }
            }
            for y in 0..h {
                for x in 0..(w - 1) {
                    solver.add_expr(
                        (!is_line.horizontal.at((y, x))
                            & is_passed.at((y, x))
                            & is_passed.at((y, x + 1)))
                        .imp(cell_clue_id.at((y, x)).ne(cell_clue_id.at((y, x + 1)))),
                    );
                }
                for x1 in 0..(w - 2) {
                    for x2 in (x1 + 2)..w {
                        solver.add_expr(
                            (is_passed.at((y, x1))
                                & is_passed.at((y, x2))
                                & !(is_passed.slice_fixed_y((y, (x1 + 1)..x2)).any()))
                            .imp(cell_clue_id.at((y, x1)).ne(cell_clue_id.at((y, x2)))),
                        );
                    }
                }
            }
            for x in 0..w {
                for y in 0..(h - 1) {
                    solver.add_expr(
                        (!is_line.vertical.at((y, x))
                            & is_passed.at((y, x))
                            & is_passed.at((y + 1, x)))
                        .imp(cell_clue_id.at((y, x)).ne(cell_clue_id.at((y + 1, x)))),
                    );
                }
                for y1 in 0..(h - 2) {
                    for y2 in (y1 + 2)..h {
                        solver.add_expr(
                            (is_passed.at((y1, x))
                                & is_passed.at((y2, x))
                                & !(is_passed.slice_fixed_x(((y1 + 1)..y2, x)).any()))
                            .imp(cell_clue_id.at((y1, x)).ne(cell_clue_id.at((y2, x)))),
                        );
                    }
                }
            }
        }

        #[cfg(not(test))]
        {
            panic!("use_propagator must be true in non-test code");
        }
    }

    solver
        .answer_iter()
        .take(num_max_answers)
        .map(|f| f.get_unwrap(&is_line))
        .collect()
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
    problem_to_url(combinator(), "numlin", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["numlin"], url)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod instance_generator;
