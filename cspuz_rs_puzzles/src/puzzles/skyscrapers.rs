use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Choice2, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, Optionalize, OutsideCells4, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_skyscrapers(
    clues_up: &[Option<i32>],
    clues_down: &[Option<i32>],
    clues_left: &[Option<i32>],
    clues_right: &[Option<i32>],
    cells: &Option<Vec<Vec<Option<i32>>>>,
) -> Option<Vec<Vec<Option<i32>>>> {
    let h = clues_left.len();
    let w = clues_up.len();

    if h != w {
        return None;
    }

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((h, w), 1, h as i32);
    solver.add_answer_key_int(num);

    for y in 0..h {
        for x in 0..w {
            if let Some(clues) = cells {
                if let Some(n) = clues[y][x] {
                    if n > 0 {
                        solver.add_expr(num.at((y, x)).eq(n));
                    }
                }
            }

            if let Some(n) = &clues_up[x] {
                let mut visible = vec![];
                for y1 in 0..h {
                    visible.push(
                        num.slice((..y1, x..=x))
                            .ge(num.at((y1, x)) + 1)
                            .count_true()
                            .eq(0),
                    );
                }
                solver.add_expr(count_true(visible).eq(*n));
            }
            if let Some(n) = &clues_down[x] {
                let mut visible = vec![];
                for y1 in 0..h {
                    visible.push(
                        num.slice((y1.., x..=x))
                            .ge(num.at((y1, x)) + 1)
                            .count_true()
                            .eq(0),
                    );
                }
                solver.add_expr(count_true(visible).eq(*n));
            }
            if let Some(n) = &clues_left[y] {
                let mut visible = vec![];
                for x1 in 0..w {
                    visible.push(
                        num.slice((y..=y, ..x1))
                            .ge(num.at((y, x1)) + 1)
                            .count_true()
                            .eq(0),
                    );
                }
                solver.add_expr(count_true(visible).eq(*n));
            }
            if let Some(n) = &clues_right[y] {
                let mut visible = vec![];
                for x1 in 0..w {
                    visible.push(
                        num.slice((y..=y, x1..))
                            .ge(num.at((y, x1)) + 1)
                            .count_true()
                            .eq(0),
                    );
                }
                solver.add_expr(count_true(visible).eq(*n));
            }
        }
    }

    for i in 0..h {
        solver.all_different(num.slice_fixed_y((i, ..)));
        solver.all_different(num.slice_fixed_x((.., i)));
    }

    solver.irrefutable_facts().map(|f| f.get(num))
}

pub type Problem = (
    (
        Vec<Option<i32>>,
        Vec<Option<i32>>,
        Vec<Option<i32>>,
        Vec<Option<i32>>,
    ),
    Option<Vec<Vec<Option<i32>>>>,
);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
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
    let height = problem.0 .1.len();
    let width = problem.0 .3.len();

    problem_to_url_with_context(
        combinator(),
        "skyscrapers",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["skyscrapers"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        (
            (
                vec![None, None, None, None],
                vec![None, Some(1), Some(3), None],
                vec![None, Some(4), None, None],
                vec![None, None, Some(3), None],
            ),
            None,
        )
    }

    #[test]
    fn test_skyscrapers_problem() {
        {
            let ((clues_up, clues_down, clues_left, clues_right), cells) = problem_for_tests();
            let ans = solve_skyscrapers(&clues_up, &clues_down, &clues_left, &clues_right, &cells);
            assert!(ans.is_some());
            let ans = ans.unwrap();
            let expected = crate::util::tests::to_option_2d([
                [2, 1, 4, 3],
                [1, 2, 3, 4],
                [4, 3, 1, 2],
                [3, 4, 2, 1],
            ]);
            assert_eq!(ans, expected);
        }
    }

    #[test]
    fn test_skyscrapers_serializer() {
        {
            let problem = problem_for_tests();
            let url = "https://puzz.link/p?skyscrapers/4/4/k13h4j3g";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
