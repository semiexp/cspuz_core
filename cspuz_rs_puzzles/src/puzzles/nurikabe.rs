use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::{count_true, BoolVarArray2D, Solver};

/// `Reachable` is the default encoding. `GroupId` is kept for tests and local A/B.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NurikabeModel {
    GroupId,
    Reachable,
}

const DEFAULT_MODEL: NurikabeModel = NurikabeModel::Reachable;

pub fn solve_nurikabe(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    solve_nurikabe_with_model(clues, DEFAULT_MODEL)
}

#[doc(hidden)]
pub fn solve_nurikabe_with_model(
    clues: &[Vec<Option<i32>>],
    model: NurikabeModel,
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    add_constraints(clues, &mut solver, is_black, model);

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

pub fn enumerate_answers_nurikabe(
    clues: &[Vec<Option<i32>>],
    num_max_answers: usize,
) -> Vec<Vec<Vec<bool>>> {
    enumerate_answers_nurikabe_with_model(clues, num_max_answers, DEFAULT_MODEL)
}

#[doc(hidden)]
pub fn enumerate_answers_nurikabe_with_model(
    clues: &[Vec<Option<i32>>],
    num_max_answers: usize,
    model: NurikabeModel,
) -> Vec<Vec<Vec<bool>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    add_constraints(clues, &mut solver, is_black, model);

    solver
        .answer_iter()
        .take(num_max_answers)
        .map(|f| f.get_unwrap(is_black))
        .collect()
}

fn add_constraints(
    clues: &[Vec<Option<i32>>],
    solver: &mut Solver,
    is_black: &BoolVarArray2D,
    model: NurikabeModel,
) {
    match model {
        NurikabeModel::GroupId => add_constraints_group_id(clues, solver, is_black),
        NurikabeModel::Reachable => add_constraints_reachable(clues, solver, is_black),
    }
}

fn add_constraints_group_id(
    clues: &[Vec<Option<i32>>],
    solver: &mut Solver,
    is_black: &BoolVarArray2D,
) {
    let (h, w) = util::infer_shape(clues);

    let mut clue_pos = vec![];
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                clue_pos.push((y, x, n));
            }
        }
    }

    let group_id = solver.int_var_2d((h, w), 0, clue_pos.len() as i32);
    solver.add_expr(is_black.iff(group_id.eq(0)));

    graph::active_vertices_connected_2d(solver, is_black);
    for i in 1..=clue_pos.len() {
        graph::active_vertices_connected_2d(solver, group_id.eq(i as i32));
    }

    solver.add_expr(
        (!is_black.conv2d_or((2, 1))).imp(
            group_id
                .slice((..(h - 1), ..))
                .eq(group_id.slice((1.., ..))),
        ),
    );
    solver.add_expr(
        (!is_black.conv2d_or((1, 2))).imp(
            group_id
                .slice((.., ..(w - 1)))
                .eq(group_id.slice((.., 1..))),
        ),
    );
    solver.add_expr(!is_black.conv2d_and((2, 2)));

    for (i, &(y, x, n)) in clue_pos.iter().enumerate() {
        solver.add_expr(group_id.at((y, x)).eq((i + 1) as i32));
        if n > 0 {
            solver.add_expr(group_id.eq((i + 1) as i32).count_true().eq(n));
        }
    }
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

/// Cells that may belong to the island of clue `n` at `(y, x)`.
/// Finite `n > 0` is the open Manhattan ball `dist < n`; unknown size (`n <= 0`) is the whole board.
fn island_region(h: usize, w: usize, y: usize, x: usize, n: i32) -> Vec<(usize, usize)> {
    let mut region = Vec::new();
    for yy in 0..h {
        for xx in 0..w {
            if n <= 0 || manhattan((yy, xx), (y, x)) < n as usize {
                region.push((yy, xx));
            }
        }
    }
    region
}

fn add_constraints_reachable(
    clues: &[Vec<Option<i32>>],
    solver: &mut Solver,
    is_black: &BoolVarArray2D,
) {
    let (h, w) = util::infer_shape(clues);

    let mut clue_pos = vec![];
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                clue_pos.push((y, x, n));
            }
        }
    }

    graph::active_vertices_connected_2d(solver, is_black);
    solver.add_expr(!is_black.conv2d_and((2, 2)));

    let mut islands = Vec::with_capacity(clue_pos.len());
    let mut regions = Vec::with_capacity(clue_pos.len());
    let mut in_region = Vec::with_capacity(clue_pos.len());
    for &(y, x, n) in &clue_pos {
        islands.push(solver.bool_var_2d((h, w)));
        let region = island_region(h, w, y, x, n);
        let mut mask = vec![vec![false; w]; h];
        for &(yy, xx) in &region {
            mask[yy][xx] = true;
        }
        regions.push(region);
        in_region.push(mask);
    }

    for i in 0..clue_pos.len() {
        let island = &islands[i];
        solver.add_expr(island.imp(!is_black));

        for y in 0..h {
            for x in 0..w {
                if !in_region[i][y][x] {
                    solver.add_expr(!island.at((y, x)));
                }
            }
        }
        graph::active_vertices_connected_2d_region(solver, island, &regions[i]);

        let (y, x, n) = clue_pos[i];
        solver.add_expr(island.at((y, x)));
        for (j, &(yj, xj, _)) in clue_pos.iter().enumerate() {
            if i != j {
                solver.add_expr(!island.at((yj, xj)));
            }
        }
        if n > 0 {
            solver.add_expr(island.count_true().eq(n));
        }

        solver.add_expr(
            (island.slice((..(h - 1), ..)) & !is_black.slice((1.., ..)))
                .imp(island.slice((1.., ..))),
        );
        solver.add_expr(
            (island.slice((1.., ..)) & !is_black.slice((..(h - 1), ..)))
                .imp(island.slice((..(h - 1), ..))),
        );
        solver.add_expr(
            (island.slice((.., ..(w - 1))) & !is_black.slice((.., 1..)))
                .imp(island.slice((.., 1..))),
        );
        solver.add_expr(
            (island.slice((.., 1..)) & !is_black.slice((.., ..(w - 1))))
                .imp(island.slice((.., ..(w - 1)))),
        );
    }

    for y in 0..h {
        for x in 0..w {
            let members: Vec<_> = islands
                .iter()
                .enumerate()
                .filter(|(i, _)| in_region[*i][y][x])
                .map(|(_, island)| island.at((y, x)))
                .collect();
            if members.is_empty() {
                solver.add_expr(is_black.at((y, x)));
            } else {
                solver.add_expr(count_true(members).eq(is_black.at((y, x)).ite(0, 1)));
            }
        }
    }
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
    problem_to_url(combinator(), "nurikabe", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["nurikabe"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![None, None, None, None, None, None],
            vec![None, Some(8), None, None, None, None],
            vec![None, None, None, None, Some(8), None],
            vec![None, None, Some(9), None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
        ]
    }

    fn answers_match(problem: &[Vec<Option<i32>>]) {
        let group_id = enumerate_answers_nurikabe_with_model(problem, 3, NurikabeModel::GroupId);
        let reachable = enumerate_answers_nurikabe_with_model(problem, 3, NurikabeModel::Reachable);
        assert_eq!(group_id, reachable);
        assert_eq!(group_id.len(), 1);
    }

    #[test]
    #[rustfmt::skip]
    fn test_nurikabe_problem() {
        let problem = problem_for_tests();
        let ans = solve_nurikabe(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = vec![
            vec![Some(false), Some(false), None, Some(false), Some(false), None],
            vec![None, Some(false), None, None, None, None],
            vec![None, Some(true), Some(true), Some(true), Some(false), Some(false)],
            vec![Some(false), None, Some(false), Some(true), None, None],
            vec![Some(false), None, Some(false), None, Some(false), Some(false)],
            vec![Some(false), None, Some(false), None, Some(false), Some(false)],
        ];
        assert_eq!(ans, expected);
        assert_eq!(
            solve_nurikabe_with_model(&problem, NurikabeModel::GroupId),
            Some(expected)
        );
    }

    #[test]
    fn test_nurikabe_encodings_agree() {
        answers_match(&[
            vec![Some(1), None, Some(1)],
            vec![None, None, None],
            vec![Some(1), None, Some(1)],
        ]);
        answers_match(&[
            vec![Some(-1), None, Some(1)],
            vec![None, None, None],
            vec![Some(1), None, Some(1)],
        ]);
        assert_eq!(island_region(4, 4, 0, 0, 2).len(), 3);
        assert_eq!(island_region(3, 3, 0, 0, -1).len(), 9);
    }

    #[test]
    fn test_nurikabe_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?nurikabe/6/6/m8n8i9u";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
