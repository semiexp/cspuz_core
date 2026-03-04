use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    DecInt, HexInt, Optionalize, PrefixAndSuffix, Seq, Sequencer, Size, Spaces, Tuple2,
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

pub type Grid = (
    Vec<Option<i32>>,
    Vec<Option<i32>>,
    Vec<Option<i32>>,
    Vec<Option<i32>>,
    Option<Vec<Vec<Option<i32>>>>,
);

pub type Problem = (i32, Grid);

fn internal_combinator() -> impl Combinator<Option<i32>> {
    Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
    ])
}

pub struct GridCombinator;

impl Combinator<Grid> for GridCombinator {
    fn serialize(
        &self,
        ctx: &cspuz_rs::serializer::Context,
        input: &[Grid],
    ) -> Option<(usize, Vec<u8>)> {
        if input.is_empty() {
            return None;
        }

        let height = ctx.height?;
        let width = ctx.width?;

        let problem = &input[0];

        let surrounding = [
            &problem.0[..],
            &problem.1[..],
            &problem.2[..],
            &problem.3[..],
        ]
        .concat();
        let mut ret = Seq::new(internal_combinator(), 2 * (width + height))
            .serialize(ctx, &[surrounding])?
            .1;

        if let Some(cells) = &problem.4 {
            ret.extend(
                ContextBasedGrid::new(internal_combinator())
                    .serialize(ctx, &[cells.clone()])?
                    .1,
            );
        }

        Some((1, ret))
    }

    fn deserialize(
        &self,
        ctx: &cspuz_rs::serializer::Context,
        input: &[u8],
    ) -> Option<(usize, Vec<Grid>)> {
        let mut sequencer = Sequencer::new(input);

        let height = ctx.height?;
        let width = ctx.width?;

        let surrounding =
            sequencer.deserialize(ctx, Seq::new(internal_combinator(), 2 * (width + height)))?;
        if surrounding.len() != 1 {
            return None;
        }
        let surrounding = surrounding.into_iter().next().unwrap();

        let clues_up = surrounding[..width].to_vec();
        let clues_down = surrounding[width..(2 * width)].to_vec();
        let clues_left = surrounding[(2 * width)..(2 * width + height)].to_vec();
        let clues_right = surrounding[(2 * width + height)..].to_vec();

        if sequencer.n_remaining() > 0 {
            let cells = sequencer.deserialize(ctx, ContextBasedGrid::new(internal_combinator()))?;
            if cells.len() != 1 {
                return None;
            }
            let cells = cells.into_iter().next().unwrap();
            Some((
                sequencer.n_read(),
                vec![(clues_up, clues_down, clues_left, clues_right, Some(cells))],
            ))
        } else {
            Some((
                sequencer.n_read(),
                vec![(clues_up, clues_down, clues_left, clues_right, None)],
            ))
        }
    }
}

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        PrefixAndSuffix::new("", DecInt, "/"),
        GridCombinator,
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
                None,
            ),
        )
    }

    #[test]
    fn test_easy_as_abc_problem() {
        {
            let (range, (clues_up, clues_down, clues_left, clues_right, cells)) =
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
