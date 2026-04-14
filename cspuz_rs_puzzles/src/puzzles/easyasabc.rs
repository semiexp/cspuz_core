use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Choice2, Combinator, Context,
    ContextBasedGrid, DecInt, Dict, HexInt, Optionalize, OutsideCells4, PrefixAndSuffix, Size,
    Spaces, Tuple3,
};
use cspuz_rs::solver::Solver;

pub fn solve_easy_as_abc(
    range: i32,
    clues_up: &[Option<i32>],
    clues_down: &[Option<i32>],
    clues_left: &[Option<i32>],
    clues_right: &[Option<i32>],
    cells: &Option<Vec<Vec<Option<i32>>>>,
) -> Option<Vec<Vec<Option<i32>>>> {
    let h = clues_left.len();
    let w = clues_up.len();

    let mut solver = Solver::new();
    let numbers = &solver.int_var_2d((h, w), -1, range);
    solver.add_answer_key_int(numbers);

    for y in 0..h {
        for x in 0..w {
            if let Some(clues) = cells {
                if let Some(n) = clues[y][x] {
                    if n > 0 {
                        solver.add_expr(numbers.at((y, x)).eq(n));
                    } else {
                        solver.add_expr(numbers.at((y, x)).ge(0));
                    }
                }
            }

            if let Some(n) = &clues_up[x] {
                solver.add_expr(
                    (numbers.at((y, x)).eq(*n))
                        .imp(numbers.ge(0).slice((..=y, x..=x)).count_true().eq(1)),
                );
            }
            if let Some(n) = &clues_down[x] {
                solver.add_expr(
                    (numbers.at((y, x)).eq(*n))
                        .imp(numbers.ge(0).slice((y.., x..=x)).count_true().eq(1)),
                );
            }
            if let Some(n) = &clues_left[y] {
                solver.add_expr(
                    (numbers.at((y, x)).eq(*n))
                        .imp(numbers.ge(0).slice((y..=y, ..=x)).count_true().eq(1)),
                );
            }
            if let Some(n) = &clues_right[y] {
                solver.add_expr(
                    (numbers.at((y, x)).eq(*n))
                        .imp(numbers.ge(0).slice((y..=y, x..)).count_true().eq(1)),
                );
            }
        }
    }

    for i in 1..=range {
        for y in 0..h {
            solver.add_expr(numbers.eq(i).slice_fixed_y((y, ..)).count_true().eq(1));
        }
        for x in 0..w {
            solver.add_expr(numbers.eq(i).slice_fixed_x((.., x)).count_true().eq(1));
        }
    }

    solver.add_expr(numbers.ne(0));

    solver.irrefutable_facts().map(|f| f.get(numbers))
}

pub type Problem = (
    i32,
    (
        Vec<Option<i32>>,
        Vec<Option<i32>>,
        Vec<Option<i32>>,
        Vec<Option<i32>>,
    ),
    Option<Vec<Vec<Option<i32>>>>,
);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple3::new(
        PrefixAndSuffix::new("", DecInt, "/"),
        OutsideCells4::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
        ])),
        Choice2::new(
            Optionalize::new(ContextBasedGrid::new(Choice::new(vec![
                Box::new(Optionalize::new(HexInt)),
                Box::new(Dict::new(Some(-1), ".")),
                Box::new(Spaces::new(None, 'g')),
            ]))),
            Dict::new(None, ""),
        ),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1 .1.len();
    let width = problem.1 .3.len();

    problem_to_url_with_context(
        combinator(),
        "easyasabc",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["easyasabc"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        (
            3,
            (
                vec![Some(1), None, None, None],
                vec![None, None, None, None],
                vec![Some(2), None, None, Some(3)],
                vec![None, Some(3), None, None],
            ),
            None,
        )
    }

    #[test]
    fn test_easy_as_abc_problem() {
        {
            let (range, (clues_up, clues_down, clues_left, clues_right), cells) =
                problem_for_tests();
            let ans = solve_easy_as_abc(
                range,
                &clues_up,
                &clues_down,
                &clues_left,
                &clues_right,
                &cells,
            );
            assert!(ans.is_some());
            let ans = ans.unwrap();
            let expected = crate::util::tests::to_option_2d([
                [-1, 2, 3, 1],
                [1, -1, 2, 3],
                [2, 3, 1, -1],
                [3, 1, -1, 2],
            ]);
            assert_eq!(ans, expected);
        }
    }

    #[test]
    fn test_easy_as_abc_serializer() {
        {
            let problem = problem_for_tests();
            let url = "https://puzz.link/p?easyasabc/4/4/3/1m2h3g3h";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
