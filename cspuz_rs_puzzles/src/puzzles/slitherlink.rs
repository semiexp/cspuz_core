use crate::puzzles::loop_common::add_full_loop_constraints;
use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, NumSpaces, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_slitherlink(
    full: bool,
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    add_constraints(&mut solver, full, is_line, clues);

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

pub fn enumerate_answers_slitherlink(
    full: bool,
    clues: &[Vec<Option<i32>>],
    num_max_answers: usize,
) -> Vec<graph::BoolGridEdgesModel> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    add_constraints(&mut solver, full, is_line, clues);

    solver
        .answer_iter()
        .take(num_max_answers)
        .map(|f| f.get_unwrap(is_line))
        .collect()
}

fn add_constraints(
    solver: &mut Solver,
    full: bool,
    is_line: &graph::BoolGridEdges,
    clues: &[Vec<Option<i32>>],
) {
    let (h, w) = util::infer_shape(clues);

    graph::single_cycle_grid_edges(solver, is_line);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if n < 0 {
                    continue;
                }
                solver.add_expr(is_line.cell_neighbors((y, x)).count_true().eq(n));
            }
        }
    }

    if full {
        add_full_loop_constraints(solver, is_line, h + 1, w + 1);
    }
}

type Problem = (bool, Vec<Vec<Option<i32>>>);

pub(crate) fn combinator() -> impl Combinator<Problem> {
    Tuple2::new(
        Choice::new(vec![
            Box::new(Dict::new(true, "f/")),
            Box::new(Dict::new(false, "")),
        ]),
        Grid::new(Choice::new(vec![
            Box::new(NumSpaces::new(4, 2)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "slither", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["slither", "slitherlink"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        let ret = vec![
            vec![Some(3), None, None, None],
            vec![Some(3), None, None, None],
            vec![None, Some(2), Some(2), None],
            vec![None, Some(2), None, Some(1)],
        ];
        (false, ret)
    }

    fn problem_for_tests2() -> Problem {
        let ret = vec![
            vec![None, Some(3), None],
            vec![None, None, None],
            vec![None, None, None],
        ];
        (true, ret)
    }

    #[test]
    #[rustfmt::skip]
    fn test_slitherlink_problem1() {
        let (full, problem) = problem_for_tests1();
        let ans = solve_slitherlink(full, &problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: util::tests::to_option_bool_2d([
                [1, 1, 1, 1],
                [1, 0, 1, 0],
                [1, 0, 0, 0],
                [0, 1, 0, 1],
                [1, 0, 0, 0],
            ]),
            vertical: util::tests::to_option_bool_2d([
                [1, 0, 0, 0, 1],
                [0, 1, 1, 1, 1],
                [1, 0, 1, 1, 1],
                [1, 1, 0, 0, 0],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn test_slitherlink_problem2() {
        let (full, problem) = problem_for_tests2();
        let ans = solve_slitherlink(full, &problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: util::tests::to_option_bool_2d([
                [1, 0, 1],
                [0, 1, 0],
                [0, 1, 0],
                [1, 0, 1],
        
            ]),
            vertical: util::tests::to_option_bool_2d([
                [1, 1, 1, 1],
                [1, 0, 0, 1],
                [1, 1, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_slitherlink_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://puzz.link/p?slither/4/4/dgdh2c71";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://puzz.link/p?slither/f/3/3/gdk";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
