use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_pzprxs, url_to_problem, Choice, Combinator, Grid, HexInt, Map, Optionalize,
    Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_lightandshadow(clues: &[Vec<Option<(i32, bool)>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let mut clue_pos = vec![];
    for y in 0..h {
        for x in 0..w {
            if let Some((n, c)) = clues[y][x] {
                clue_pos.push((y, x, n));
                solver.add_expr(is_black.at((y, x)) ^ !c);
            }
        }
    }

    let group_id = solver.int_var_2d((h, w), 1, clue_pos.len() as i32);

    for i in 1..=clue_pos.len() {
        graph::active_vertices_connected_2d(&mut solver, group_id.eq(i as i32));
    }

    solver.add_expr(
        (is_black.slice((.., ..(w - 1))) ^ !is_black.slice((.., 1..))).iff(
            group_id
                .slice((.., ..(w - 1)))
                .eq(group_id.slice((.., 1..))),
        ),
    );
    solver.add_expr(
        (is_black.slice((..(h - 1), ..)) ^ !is_black.slice((1.., ..))).iff(
            group_id
                .slice((..(h - 1), ..))
                .eq(group_id.slice((1.., ..))),
        ),
    );

    for (i, &(y, x, n)) in clue_pos.iter().enumerate() {
        solver.add_expr(group_id.at((y, x)).eq((i + 1) as i32));
        if n > 0 {
            solver.add_expr(group_id.eq((i + 1) as i32).count_true().eq(n));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

fn clue_combinator() -> impl Combinator<(i32, bool)> {
    Map::new(
        HexInt,
        |(x, y): (i32, bool)| match y {
            false => Some(2 * x),
            true => Some(2 * x + 1),
        },
        |n: i32| match n {
            i => Some((i / 2, i % 2 == 1)),
        },
    )
}

type Problem = Vec<Vec<Option<(i32, bool)>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(clue_combinator())),
        Box::new(Spaces::new(None, 'g')),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url_pzprxs(combinator(), "lightshadow", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["lightshadow"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some((2, false)), None, Some((2, false)), None],
            vec![None, None, None, None],
            vec![None, None, Some((3, false)), None],
            vec![None, Some((3, true)), None, Some((0, true))],
        ]
    }

    #[test]
    fn test_lightandshadow_problem() {
        let problem = problem_for_tests();
        let ans = solve_lightandshadow(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 0],
            [0, 1, 1, 1],
            [1, 0, 0, 1],
            [1, 1, 0, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_lightandshadow_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?lightshadow/4/4/4g4m6h7g1";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
