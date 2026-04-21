use crate::util;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_simplegako(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((h, w), 1, (h + w) as i32);
    solver.add_answer_key_int(num);

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                if c > 0 {
                    solver.add_expr(num.at((y, x)).eq(c));
                }
            }
            solver.add_expr(
                (num.slice_fixed_x((.., x)).eq(num.at((y, x))).count_true()
                    + num.slice_fixed_y((y, ..)).eq(num.at((y, x))).count_true())
                .eq(num.at((y, x)) + 1),
            );
        }
    }

    solver.irrefutable_facts().map(|f| f.get(num))
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
    problem_to_url(combinator(), "simplegako", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["simplegako"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![None, None, None, None],
            vec![None, Some(5), None, Some(1)],
            vec![Some(2), None, Some(2), None],
            vec![None, None, None, None],
        ]
    }

    #[test]
    fn test_simplegako_problem() {
        let problem = problem_for_tests();
        let ans = solve_simplegako(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [5, 5, 5, 3],
            [5, 5, 5, 1],
            [2, 1, 2, 3],
            [5, 5, 5, 3],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_simplegako_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?simplegako/4/4/k5g12g2k";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
