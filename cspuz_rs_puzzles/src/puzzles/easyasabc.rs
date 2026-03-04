use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    HexInt, Optionalize, Seq, Sequencer, Size, Spaces,
};
use cspuz_rs::solver::{sum, IntVarArray1D, Solver};

pub fn solve_easy_as_abc(
    range: i32,
    clues_up: &[Option<i32>],
    clues_down: &[Option<i32>],
    clues_left: &[Option<i32>],
    clues_right: &[Option<i32>],
    cells: &Option<Vec<Vec<Option<i32>>>>,
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(cells);


    let mut solver = Solver::new();
    let numbers = &solver.int_var_2d((h, w), 0, range);
    solver.add_answer_key_int(numbers);

    if let Some(cells) = cells.as_ref() {
        for i in 0..size {
            for j in 0..size {
                if let Some(n) = cells[i][j] {
                    solver.add_expr(numbers.at((i, j)).eq(n));
                }
            }
        }
    }

    
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

        let surrounding = [&problem.0[..], &problem.1[..], &problem.2[..], &problem.3[..]].concat();
        let mut ret = Seq::new(internal_combinator(), 2* (width + height))
            .serialize(ctx, &[surrounding])?
            .1;

        if let Some(cells) = &problem.2 {
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
        let clues_down = surrounding[width..(2*width)].to_vec();
        let clues_left = surrounding[(2*width)..(2*width + height)].to_vec();
        let clues_right = surrounding[(2*width + height)..].to_vec();

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
            Some((sequencer.n_read(), vec![(clues_up, clues_down, clues_left, clues_right, None)]))
        }
    }
}

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(PrefixAndSuffix::new("", DecInt, "/"), Rooms))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.3.len();
    let width = problem.1.len();

    problem_to_url_with_context(
        combinator(),
        "easy_as_abc",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["easy_as_abc"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests1() -> Problem {
        (
            vec![Some(6), Some(1), None, None, None],
            vec![None, Some(4), None, Some(3), Some(5)],
            None,
        )
    }

    fn problem_for_tests2() -> Problem {
        (
            vec![None, Some(1), None, None, Some(3)],
            vec![Some(1), None, None, Some(1), None],
            Some(vec![
                vec![None, None, None, None, None],
                vec![None, Some(2), None, None, None],
                vec![None, None, None, None, None],
                vec![None, None, None, None, None],
                vec![None, None, None, None, None],
            ]),
        )
    }

    #[test]
    fn test_easy_as_abc_problem() {
        {
            let (clues_up, clues_left, cells) = problem_for_tests1();
            let ans = solve_easy_as_abc(&clues_up, &clues_left, &cells);
            assert!(ans.is_some());
            let ans = ans.unwrap();
            let expected = crate::util::tests::to_option_2d([
                [0, 2, 0, 1, 3],
                [2, 0, 1, 3, 0],
                [3, 1, 0, 2, 0],
                [1, 0, 3, 0, 2],
                [0, 3, 2, 0, 1],
            ]);
            assert_eq!(ans, expected);
        }

        {
            let (clues_up, clues_left, cells) = problem_for_tests2();
            let ans = solve_easy_as_abc(&clues_up, &clues_left, &cells);
            assert!(ans.is_some());
            let ans = ans.unwrap();
            let expected = crate::util::tests::to_option_2d([
                [2, 3, 0, 1, 0],
                [0, 2, 1, 0, 3],
                [1, 0, 3, 2, 0],
                [0, 1, 0, 3, 2],
                [3, 0, 2, 0, 1],
            ]);
            assert_eq!(ans, expected);
        }
    }

    #[test]
    fn test_easy_as_abc_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://puzz.link/p?doppelblock/5/5/61j4g35";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://puzz.link/p?doppelblock/5/5/g1h31h1gl2x";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
