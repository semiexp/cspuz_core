use crate::util;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_minesweeper(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_mine = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_mine);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_mine.at((y, x)));
                if n < 0 {
                    continue;
                }
                solver.add_expr(is_mine.eight_neighbors((y, x)).count_true().eq(n));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_mine))
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
    problem_to_url(combinator(), "mines", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["mines"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        // https://puzz.link/p?mines/5/3/1g1g.j.0g1g./
        vec![
            vec![Some(1), None, Some(1), None, Some(-1)],
            vec![None, None, None, None, Some(-1)],
            vec![Some(0), None, Some(1), None, Some(-1)],
        ]
    }

    #[test]
    fn test_minesweeper_problem() {
        let problem = problem_for_tests();
        let ans = solve_minesweeper(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_minesweeper_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?mines/5/3/1g1g.j.0g1g.";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
