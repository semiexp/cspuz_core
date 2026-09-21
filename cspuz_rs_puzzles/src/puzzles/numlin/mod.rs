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
