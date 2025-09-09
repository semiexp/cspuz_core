use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_dominion(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let mut max_number = 0;
    let mut clue_range = vec![];
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if !clue_range.contains(&n) {
                    if n > max_number {
                        max_number = n;
                    }
                    clue_range.push(n);
                }
            }
        }
    }

    clue_range.push(0);

    let group_id = solver.int_var_2d((h, w), 0, max_number);
    solver.add_expr(is_black.iff(group_id.eq(0)));

    for i in 1..max_number + 1 {
        if clue_range.contains(&i) {
            graph::active_vertices_connected_2d(&mut solver, group_id.eq(i as i32));
        } else {
            solver.add_expr(group_id.ne(i));
        }
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if n == -1 {
                    solver.add_expr(group_id.at((y, x)).ge(1));
                } else {
                    solver.add_expr(group_id.at((y, x)).eq(n));
                }
            }
        }
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

    for y in 0..h {
        for x in 0..w {
            solver.add_expr(
                is_black
                    .at((y, x))
                    .imp(count_true(is_black.four_neighbors((y, x))).eq(1)),
            );
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
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
    problem_to_url(combinator(), "dominion", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["dominion"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(1), None, None, Some(2), None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, Some(3)],
            vec![None, None, Some(3), None, None],
            vec![Some(1), None, None, Some(-1), None],
        ]
    }

    #[test]
    fn test_dominion_problem() {
        let problem = problem_for_tests();
        let ans = solve_dominion(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 0, 1],
            [0, 1, 0, 0, 1],
            [0, 0, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_dominion_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?dominion/5/5/1h2p3h3h1h.g"; // Credits to xetto
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
