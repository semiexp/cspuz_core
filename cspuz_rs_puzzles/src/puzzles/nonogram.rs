use cspuz_rs::complex_constraints::japanese;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Combinator, Context, HexInt, OutsideSequences,
    Size,
};
use cspuz_rs::solver::{BoolVarArray1D, Solver};

pub fn solve_nonogram(
    clue_vertical: &[Option<Vec<i32>>],
    clue_horizontal: &[Option<Vec<i32>>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let h = clue_horizontal.len();
    let w = clue_vertical.len();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let mut add_constraint = |target: BoolVarArray1D, clue: &Option<Vec<i32>>| {
        if let Some(clue) = clue {
            let group_id = japanese(&mut solver, &target, &vec![false; clue.len()]);

            for i in 0..clue.len() {
                solver.add_expr((&target & (group_id.eq(i as i32))).count_true().eq(clue[i]));
            }
        } else {
            solver.add_expr(!target);
        }
    };

    for y in 0..h {
        add_constraint(is_black.slice_fixed_y((y, ..)), &clue_horizontal[y]);
    }
    for x in 0..w {
        add_constraint(is_black.slice_fixed_x((.., x)), &clue_vertical[x]);
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = (Vec<Option<Vec<i32>>>, Vec<Option<Vec<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(OutsideSequences::new(HexInt))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1.len();
    let width = problem.0.len();
    problem_to_url_with_context(
        combinator(),
        "nonogram",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["nonogram"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        let clue_vertical = vec![
            Some(vec![1, 3]),
            Some(vec![2, 1]),
            Some(vec![1, 3]),
            Some(vec![1, 1, 1]),
            Some(vec![2]),
            Some(vec![5]),
        ];
        let clue_horizontal = vec![
            None,
            Some(vec![2, 3]),
            Some(vec![2, 2]),
            Some(vec![1, 1]),
            Some(vec![1, 2, 1]),
            Some(vec![1, 1, 1]),
            Some(vec![3]),
        ];
        (clue_vertical, clue_horizontal)
    }

    #[test]
    fn test_nonogram_problem() {
        let problem = problem_for_tests();
        let ans = solve_nonogram(&problem.0, &problem.1);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 0, 0, 0, 0, 0],
            [1, 1, 0, 1, 1, 1],
            [0, 1, 1, 0, 1, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 0, 1, 1, 0, 1],
            [1, 0, 1, 0, 0, 1],
            [0, 1, 1, 1, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_nonogram_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?nonogram/6/7/31h12h31h111g2i5l32g22g11g1211113h";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
