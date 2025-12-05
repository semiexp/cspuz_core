use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, Dict, HexInt,
    Optionalize, PrefixAndSuffix, Rooms, Seq, Sequencer, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_aquarium(
    region_aware: bool,
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues_up: &[Option<i32>],
    clues_left: &[Option<i32>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    for y in 0..h {
        if let Some(n) = clues_left[y] {
            let row = is_black.slice_fixed_y((y, ..));
            solver.add_expr(row.count_true().eq(n));
        }
    }
    for x in 0..w {
        if let Some(n) = clues_up[x] {
            let col = is_black.slice_fixed_x((.., x));
            solver.add_expr(col.count_true().eq(n));
        }
    }

    if region_aware {
        let regions = graph::borders_to_rooms(borders);
        for mut region in regions {
            region.sort();
            for i in 1..region.len() {
                if region[i - 1].0 == region[i].0 {
                    solver.add_expr(is_black.at(region[i - 1]).iff(is_black.at(region[i])));
                } else {
                    solver.add_expr(is_black.at(region[i - 1]).imp(is_black.at(region[i])));
                }
            }
        }
    } else {
        for y in 0..h {
            for x in 0..w {
                let mut visited = vec![vec![false; w]; h];
                let mut stack = vec![(y, x)];
                while let Some((cy, cx)) = stack.pop() {
                    if visited[cy][cx] {
                        continue;
                    }
                    visited[cy][cx] = true;

                    if cx > 0 && !borders.vertical[cy][cx - 1] {
                        stack.push((cy, cx - 1));
                    }
                    if cx + 1 < w && !borders.vertical[cy][cx] {
                        stack.push((cy, cx + 1));
                    }
                    if cy < h - 1 && !borders.horizontal[cy][cx] {
                        stack.push((cy + 1, cx));
                    }
                    if cy > y && !borders.horizontal[cy - 1][cx] {
                        stack.push((cy - 1, cx));
                    }
                }

                for y2 in 0..h {
                    for x2 in 0..w {
                        if visited[y2][x2] {
                            solver.add_expr(is_black.at((y, x)).imp(is_black.at((y2, x2))));
                        }
                    }
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = (
    bool,
    (
        graph::InnerGridEdges<Vec<Vec<bool>>>,
        (Vec<Option<i32>>, Vec<Option<i32>>),
    ),
);

fn internal_combinator() -> impl Combinator<Option<i32>> {
    Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
    ])
}

pub struct AquariumCombinator;

impl Combinator<(Vec<Option<i32>>, Vec<Option<i32>>)> for AquariumCombinator {
    fn serialize(
        &self,
        ctx: &cspuz_rs::serializer::Context,
        input: &[(Vec<Option<i32>>, Vec<Option<i32>>)],
    ) -> Option<(usize, Vec<u8>)> {
        if input.is_empty() {
            return None;
        }

        let height = ctx.height?;
        let width = ctx.width?;

        let problem = &input[0];

        let surrounding = [&problem.0[..], &problem.1[..]].concat();
        let ret = Seq::new(internal_combinator(), width + height)
            .serialize(ctx, &[surrounding])?
            .1;

        Some((1, ret))
    }

    fn deserialize(
        &self,
        ctx: &cspuz_rs::serializer::Context,
        input: &[u8],
    ) -> Option<(usize, Vec<(Vec<Option<i32>>, Vec<Option<i32>>)>)> {
        let mut sequencer = Sequencer::new(input);

        let height = ctx.height?;
        let width = ctx.width?;

        let surrounding =
            sequencer.deserialize(ctx, Seq::new(internal_combinator(), width + height))?;
        if surrounding.len() != 1 {
            return None;
        }
        let surrounding = surrounding.into_iter().next().unwrap();

        let clues_up = surrounding[..width].to_vec();
        let clues_left = surrounding[width..].to_vec();

        Some((sequencer.n_read(), vec![(clues_up, clues_left)]))
    }
}

fn combinator() -> impl Combinator<Problem> {
    Tuple2::new(
        Choice::new(vec![
            Box::new(Dict::new(true, "r/")),
            Box::new(Dict::new(false, "")),
        ]),
        Size::new(Tuple2::new(
            Rooms,
            PrefixAndSuffix::new("/", AquariumCombinator, ""),
        )),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1 .1 .1.len();
    let width = problem.1 .1 .0.len();

    problem_to_url_with_context(
        combinator(),
        "aquarium",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["aquarium"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 1, 1, 1],
                [1, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 1, 0, 0],
                [1, 1, 0, 1],
                [0, 1, 1, 1],
                [0, 1, 0, 1],
            ]),
        };
        let clues_up = vec![Some(3), None, None, Some(1), Some(2)];
        let clues_left = vec![None, None, Some(1), Some(3)];
        (false, (borders, (clues_up, clues_left)))
    }

    fn problem_for_tests2() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 1, 1, 0],
                [1, 0, 1, 0, 1],
                [0, 1, 0, 1, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 0, 0, 0],
                [1, 0, 0, 1],
                [1, 1, 1, 1],
                [0, 1, 0, 1],
            ]),
        };
        let clues_up = vec![None, None, None, None, None];
        let clues_left = vec![None, Some(2), Some(4), None];
        (true, (borders, (clues_up, clues_left)))
    }

    #[test]
    fn test_aquarium_problem() {
        {
            let (region_aware, (borders, (clues_up, clues_left))) = problem_for_tests1();
            let ans = solve_aquarium(region_aware, &borders, &clues_up, &clues_left);
            assert!(ans.is_some());
            let ans = ans.unwrap();

            let expected = crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 1, 1],
                [1, 0, 0, 0, 0],
                [0, 0, 1, 0, 0],
                [1, 1, 0, 0, 1],
            ]);
            assert_eq!(ans, expected);
        }

        {
            let (region_aware, (borders, (clues_up, clues_left))) = problem_for_tests2();
            let ans = solve_aquarium(region_aware, &borders, &clues_up, &clues_left);
            assert!(ans.is_some());
            let ans = ans.unwrap();

            let expected = crate::util::tests::to_option_bool_2d([
                [-1, -1, -1, -1, -1],
                [1, 0, 0, 0, 1],
                [-1, 1, -1, 1, -1],
                [-1, -1, -1, -1, -1],
            ]);
            assert_eq!(ans, expected);
        }
    }

    #[test]
    fn test_aquarium_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://puzz.link/p?aquarium/5/4/9lqgfk4/3h12h13";
            crate::util::tests::serializer_test(
                problem,
                url,
                serialize_problem,
                deserialize_problem,
            );
        }
        {
            let problem = problem_for_tests2();
            let url = "https://puzz.link/p?aquarium/r/5/4/17qgela/l24g";
            crate::util::tests::serializer_test(
                problem,
                url,
                serialize_problem,
                deserialize_problem,
            );
        }
    }
}
