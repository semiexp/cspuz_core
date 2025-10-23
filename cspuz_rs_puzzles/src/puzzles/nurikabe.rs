use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::{BoolVarArray2D, Solver};

pub fn solve_nurikabe(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    add_constraints(clues, &mut solver, is_black);

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

pub fn enumerate_answers_nurikabe(
    clues: &[Vec<Option<i32>>],
    num_max_answers: usize,
) -> Vec<Vec<Vec<bool>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    add_constraints(clues, &mut solver, is_black);

    solver
        .answer_iter()
        .take(num_max_answers)
        .map(|f| f.get_unwrap(is_black))
        .collect()
}

fn add_constraints(clues: &[Vec<Option<i32>>], solver: &mut Solver, is_black: &BoolVarArray2D) {
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
    }

    #[test]
    fn test_nurikabe_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?nurikabe/6/6/m8n8i9u";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
