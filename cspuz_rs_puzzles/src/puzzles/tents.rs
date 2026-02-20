use crate::util;

use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    HexInt, Map, NumSpaces, Optionalize, Seq, Sequencer, Size, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_tents(
    clue_vertical: &[Option<i32>],
    clue_horizontal: &[Option<i32>],
    trees: &Vec<Vec<bool>>,
) -> Option<(graph::BoolGridEdgesIrrefutableFacts, Vec<Vec<Option<bool>>>)> {
    let (h, w) = util::infer_shape(trees);

    let mut solver = Solver::new();
    let is_tent = solver.bool_var_2d((h, w));
    let is_tree = solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(&is_tent);

    let is_pair = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_pair.horizontal);
    solver.add_answer_key_bool(&is_pair.vertical);

    for x in 0..w {
        for y in 0..h {
            solver.add_expr(
                (is_tent.at((y, x)) | is_tree.at((y, x)))
                    .iff(is_pair.vertex_neighbors((y, x)).count_true().eq(1)),
            ); // Each tree is paired with one tent, and vice versa
            if trees[y][x] {
                solver.add_expr(is_tree.at((y, x)));
                solver.add_expr(!is_tent.at((y, x)));
            } else {
                solver.add_expr(!is_tree.at((y, x)));
            }
            solver.add_expr(is_pair.vertex_neighbors((y, x)).count_true().le(1));
            solver.add_expr(
                is_tree
                    .four_neighbors((y, x))
                    .count_true()
                    .eq(0)
                    .imp(!is_tent.at((y, x))),
            );
        }
    }

    solver.add_expr(!(is_tent.slice((..(h - 1), ..)) & is_tent.slice((1.., ..)))); // Star battle like constraints for tents
    solver.add_expr(!(is_tent.slice((.., ..(w - 1))) & is_tent.slice((.., 1..))));
    solver.add_expr(!(is_tent.slice((..(h - 1), ..(w - 1))) & is_tent.slice((1.., 1..))));
    solver.add_expr(!(is_tent.slice((..(h - 1), 1..)) & is_tent.slice((1.., ..(w - 1)))));

    solver.add_expr(is_tree.count_true().eq(is_tent.count_true()));
    solver.add_expr(
        is_tent
            .count_true()
            .eq(&is_pair.horizontal.count_true() + &is_pair.vertical.count_true()),
    ); // There are as many pairs as there are tents and trees

    for y in 0..h {
        if let Some(n) = &clue_horizontal[y] {
            let row = is_tent.slice_fixed_y((y, ..));
            solver.add_expr(row.count_true().eq(*n));
        }
    }
    for x in 0..w {
        if let Some(n) = &clue_vertical[x] {
            let col = is_tent.slice_fixed_x((.., x));
            solver.add_expr(col.count_true().eq(*n));
        }
    }

    solver
        .irrefutable_facts()
        .map(|f| (f.get(is_pair), f.get(&is_tent)))
}

pub type Problem = (Vec<Option<i32>>, Vec<Option<i32>>, Vec<Vec<bool>>);

fn external_combinator() -> impl Combinator<Option<i32>> {
    Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
    ])
}

fn internal_combinator() -> impl Combinator<Vec<Vec<bool>>> {
    ContextBasedGrid::new(Map::new(
        Choice::new(vec![
            Box::new(NumSpaces::new(0, 17)),
            Box::new(Spaces::new_with_maximum(None, 'i', 'z')),
        ]),
        |x: bool| match x {
            true => Some(Some(0)),
            false => Some(None),
        },
        |n: Option<i32>| match n {
            Some(0) => Some(true),
            _ => Some(false),
        },
    ))
}

pub struct TentsCombinator;

impl Combinator<Problem> for TentsCombinator {
    fn serialize(
        &self,
        ctx: &cspuz_rs::serializer::Context,
        input: &[Problem],
    ) -> Option<(usize, Vec<u8>)> {
        if input.is_empty() {
            return None;
        }

        let height = ctx.height?;
        let width = ctx.width?;

        let problem = &input[0];

        let surrounding = [&problem.0[..], &problem.1[..]].concat();
        let mut ret = Seq::new(external_combinator(), width + height)
            .serialize(ctx, &[surrounding])?
            .1;

        let cells = &problem.2;

        ret.extend(internal_combinator().serialize(ctx, &[cells.clone()])?.1);

        Some((1, ret))
    }

    fn deserialize(
        &self,
        ctx: &cspuz_rs::serializer::Context,
        input: &[u8],
    ) -> Option<(usize, Vec<Problem>)> {
        let mut sequencer = Sequencer::new(input);

        let height = ctx.height?;
        let width = ctx.width?;

        let surrounding =
            sequencer.deserialize(ctx, Seq::new(external_combinator(), width + height))?;
        if surrounding.len() != 1 {
            return None;
        }
        let surrounding = surrounding.into_iter().next().unwrap();

        let clues_up = surrounding[..width].to_vec();
        let clues_left = surrounding[width..].to_vec();

        let cells = sequencer.deserialize(ctx, internal_combinator())?;
        if cells.len() != 1 {
            return None;
        }
        let cells = cells.into_iter().next().unwrap();
        Some((sequencer.n_read(), vec![(clues_up, clues_left, cells)]))
    }
}

fn combinator() -> impl Combinator<Problem> {
    Size::new(TentsCombinator)
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1.len();
    let width = problem.0.len();

    problem_to_url_with_context(
        combinator(),
        "tents",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["tents"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        let clue_vertical = vec![Some(3), None, Some(2), None, None];
        let clue_horizontal = vec![None, None, Some(1), None, None];
        let trees = crate::util::tests::to_bool_2d([
            [0, 0, 0, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 0, 1, 0, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 0, 0],
        ]);
        (clue_vertical, clue_horizontal, trees)
    }

    #[test]
    fn test_tents_problem() {
        let (clue_vertical, clue_horizontal, trees) = problem_for_tests();
        let ans = solve_tents(&clue_vertical, &clue_horizontal, &trees);
        assert!(ans.is_some());
        let (_, is_tent) = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 0, 1, 0, 1],
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 0, 0, 0, 1],
        ]);
        assert_eq!(is_tent, expected);
    }

    #[test]
    fn test_tents_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?tents/5/5/3g2j1hk1322313";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
