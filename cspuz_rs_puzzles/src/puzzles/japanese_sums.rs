use cspuz_rs::complex_constraints::japanese;
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, DecInt, Dict, HexInt, Optionalize, OutsideSequences, PrefixAndSuffix, Size,
    Spaces, Tuple3,
};
use cspuz_rs::solver::{IntVarArray1D, Solver};

pub fn solve_japanese_sums(
    k: i32,
    clue_vertical: &[Option<Vec<i32>>],
    clue_horizontal: &[Option<Vec<i32>>],
    given_numbers: &Option<Vec<Vec<Option<i32>>>>,
) -> Option<Vec<Vec<Option<i32>>>> {
    let h = clue_horizontal.len();
    let w = clue_vertical.len();

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((h, w), 0, k);
    solver.add_answer_key_int(num);

    let mut add_constraint = |target: IntVarArray1D, clue: &Option<Vec<i32>>| {
        for i in 1..=k {
            solver.add_expr(target.eq(i).count_true().le(1));
        }
        solver.add_expr(target.eq(0).count_true().ge(target.len() as i32 - k));

        if let Some(clue) = clue {
            let is_present = target.ne(0);
            let group_id = japanese(&mut solver, &is_present, &vec![false; clue.len()]);

            for i in 0..clue.len() {
                if clue[i] >= 0 {
                    solver.add_expr(group_id.eq(i as i32).ite(&target, 0).sum().eq(clue[i]));
                }
            }
        }
    };

    for y in 0..h {
        add_constraint(num.slice_fixed_y((y, ..)), &clue_horizontal[y]);
    }
    for x in 0..w {
        add_constraint(num.slice_fixed_x((.., x)), &clue_vertical[x]);
    }
    if let Some(given_numbers) = given_numbers {
        for y in 0..h {
            for x in 0..w {
                if let Some(n) = given_numbers[y][x] {
                    solver.add_expr(num.at((y, x)).eq(n));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(num))
}

type Problem = (
    i32,
    (Vec<Option<Vec<i32>>>, Vec<Option<Vec<i32>>>),
    Option<Vec<Vec<Option<i32>>>>,
);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple3::new(
        PrefixAndSuffix::new("", DecInt, "/"),
        OutsideSequences::new(Choice::new(vec![
            Box::new(Dict::new(-1, ".")),
            Box::new(HexInt),
        ])),
        Choice::new(vec![
            Box::new(Optionalize::new(ContextBasedGrid::new(Choice::new(vec![
                Box::new(Optionalize::new(HexInt)),
                Box::new(Spaces::new(None, 'g')),
            ])))),
            Box::new(Dict::new(None, "")),
        ]),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1 .1.len();
    let width = problem.1 .0.len();
    problem_to_url_with_context_and_site(
        combinator(),
        "japanesesums",
        "https://pzprxs.vercel.app/p?",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["japanesesums"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        let clue_vertical = vec![
            Some(vec![-1, -1, -1]),
            Some(vec![10]),
            Some(vec![-1, 5]),
            None,
            Some(vec![1, -1]),
            Some(vec![4, -1]),
        ];
        let clue_horizontal = vec![
            Some(vec![2, 5, 3]),
            Some(vec![-1, -1, -1]),
            Some(vec![-1, 4]),
            None,
            Some(vec![8, -1]),
        ];
        let given_numbers = vec![
            vec![None, None, None, None, None, None],
            vec![None, None, None, Some(4), None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
        ];
        (4, (clue_vertical, clue_horizontal), Some(given_numbers))
    }

    #[test]
    fn test_japanese_sums_problem() {
        let (k, problem, given_numbers) = problem_for_tests();
        let ans = solve_japanese_sums(k, &problem.0, &problem.1, &given_numbers);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [2, 0, 4, 1, 0, 3],
            [0, 3, 0, 4, 0, 1],
            [4, 2, 0, 3, 1, 0],
            [0, 1, 2, 0, 0, 4],
            [1, 4, 3, 0, 2, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_japanese_sums_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?japanesesums/6/5/4/...ah5.j.1g.4g352...4.j.8go4z";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
