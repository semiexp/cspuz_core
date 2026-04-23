use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_pzprxs, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize,
    Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_nuriloop(clues: &[Vec<Option<i32>>]) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);
    let is_passed = &graph::single_cycle_grid_edges(&mut solver, is_line);

    let mut clue_pos = vec![];
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                clue_pos.push((y, x, n));
            }
        }
    }

    let group_id = solver.int_var_2d((h, w), 0, clue_pos.len() as i32);

    solver.add_expr(is_passed.iff(group_id.eq(0)));

    for i in 1..=clue_pos.len() {
        graph::active_vertices_connected_2d(&mut solver, group_id.eq(i as i32));
    }

    for (i, &(y, x, n)) in clue_pos.iter().enumerate() {
        solver.add_expr(group_id.at((y, x)).eq((i + 1) as i32));
        if n > 0 {
            solver.add_expr(group_id.eq((i + 1) as i32).count_true().eq(n));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
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
    problem_to_url_pzprxs(combinator(), "nuriloop", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["nuriloop"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(4), None, None, None, None],
            vec![None, None, None, Some(2), None],
            vec![None, None, None, None, None],
            vec![None, Some(1), None, None, None],
            vec![None, None, None, None, None],
        ]
    }

    #[test]
    fn test_nuriloop_problem() {
        let problem = problem_for_tests();
        let ans = solve_nuriloop(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected_edges = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 1],
                [0, 0, 0, 0],
                [1, 1, 0, 0],
                [0, 0, 1, 0],
                [1, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 0, 1],
                [0, 0, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 0, 1, 1, 1],
            ]),
        };
        assert_eq!(ans, expected_edges);
    }

    #[test]
    fn test_nurilooop_serializer() {
        {
            let problem = problem_for_tests();
            let url = "https://pzprxs.vercel.app/p?nuriloop/5/5/4m2m1n";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
