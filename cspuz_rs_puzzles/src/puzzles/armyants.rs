use crate::puzzles::move_common::add_movement_constraints;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, HexInt, Optionalize, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_armyants(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<(graph::BoolGridEdgesIrrefutableFacts, Vec<Vec<Option<i32>>>)> {
    let (h, w) = borders.base_shape();
    let mut solver = Solver::new();

    let mut clue_max = 0;
    let mut num_qmark = 0;
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                clue_max = clue_max.max(n);
                if n == -1 {
                    num_qmark += 1;
                }
            }
        }
    }

    let end_state = &solver.int_var_2d((h, w), -2, clue_max + num_qmark);
    solver.add_answer_key_int(end_state);
    let movement = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&movement.horizontal);
    solver.add_answer_key_bool(&movement.vertical);

    for y in 0..h {
        for x in 0..(w - 1) {
            if borders.vertical[y][x] {
                solver.add_expr(!movement.horizontal.at((y, x)));
            }
        }
    }
    for y in 0..(h - 1) {
        for x in 0..w {
            if borders.horizontal[y][x] {
                solver.add_expr(!movement.vertical.at((y, x)));
            }
        }
    }

    add_movement_constraints(
        &mut solver,
        clue_max + num_qmark,
        movement,
        clues,
        end_state,
        h,
        w,
        false,
    );

    for y in 0..h {
        for x in 0..w {
            let connected = &solver.bool_var_2d((h, w));
            solver.add_expr(
                end_state
                    .four_neighbors((y, x))
                    .ge(end_state.at((y, x)) + 1)
                    .count_true()
                    .eq(0)
                    .imp(connected.imp(end_state.ge(1))),
            );
            solver.add_expr(
                ((end_state.at((y, x)).ge(1))
                    & end_state
                        .four_neighbors((y, x))
                        .ge(end_state.at((y, x)) + 1)
                        .count_true()
                        .eq(0))
                .imp(connected.count_true().eq(end_state.at((y, x)))),
            );
            graph::active_vertices_connected_2d(&mut solver, connected);

            for nb in connected.four_neighbor_indices((y, x)) {
                solver.add_expr(
                    end_state
                        .four_neighbors((y, x))
                        .ge(end_state.at((y, x)) + 1)
                        .count_true()
                        .eq(0)
                        .imp(end_state.ge(1).at(nb).imp(connected.at(nb))),
                );
            }
            solver.add_expr(
                end_state
                    .four_neighbors((y, x))
                    .ge(end_state.at((y, x)) + 1)
                    .count_true()
                    .eq(0)
                    .imp(
                        (end_state.ge(1).slice((1.., ..)) & end_state.ge(1).slice((..(h - 1), ..)))
                            .imp(
                                connected
                                    .slice((1.., ..))
                                    .iff(connected.slice((..(h - 1), ..))),
                            ),
                    ),
            );
            solver.add_expr(
                end_state
                    .four_neighbors((y, x))
                    .ge(end_state.at((y, x)) + 1)
                    .count_true()
                    .eq(0)
                    .imp(
                        (end_state.ge(1).slice((.., 1..)) & end_state.ge(1).slice((.., ..(w - 1))))
                            .imp(
                                connected
                                    .slice((.., 1..))
                                    .iff(connected.slice((.., ..(w - 1)))),
                            ),
                    ),
            );

            solver.add_expr(
                end_state.at((y, x)).ge(2).imp(
                    end_state
                        .four_neighbors((y, x))
                        .eq(end_state.at((y, x)) - 1)
                        .count_true()
                        .eq(1),
                ),
            );
        }
    }

    solver.add_expr(end_state.ne(0));

    solver
        .irrefutable_facts()
        .map(|f| (f.get(movement), f.get(end_state)))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "armyants",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["armyants"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([[0, 1, 1, 0], [1, 1, 1, 1], [0, 1, 1, 0]]),
            vertical: crate::util::tests::to_bool_2d([[1, 0, 1], [1, 0, 0], [0, 0, 1], [1, 0, 1]]),
        };

        let clues = vec![
            vec![None, None, Some(1), None],
            vec![Some(2), None, None, Some(4)],
            vec![Some(3), None, None, Some(1)],
            vec![None, Some(2), None, None],
        ];

        (borders, clues)
    }

    #[test]
    fn test_armyants_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_armyants(&borders, &clues);
        assert!(ans.is_some());
        let (movement, final_state) = ans.unwrap();

        let expected_nums = crate::util::tests::to_option_2d([
            [2, 1, -2, -2],
            [-2, -2, 4, -2],
            [-2, -2, 3, -2],
            [-2, -2, 2, 1],
        ]);

        let expected_paths = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 0],
                [0, 0, 1],
                [1, 1, 0],
                [0, 1, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 1],
            ]),
        };
        assert_eq!(movement, expected_paths);
        assert_eq!(final_state, expected_nums);
    }

    #[test]
    fn test_armyants_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?armyants/4/4/m38dtgh1g2h43h1g2h"; // Example puzzle on puzz.link
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
