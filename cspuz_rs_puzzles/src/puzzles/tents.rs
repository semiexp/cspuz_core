use crate::util;

use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid, Dict, HexInt, Map, NumSpaces, OutsideSequences, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_tents(
    clue_vertical: &[Option<Vec<i32>>],
    clue_horizontal: &[Option<Vec<i32>>],
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
            solver.add_expr((is_tent.at((y, x)) | is_tree.at((y, x))).imp(is_pair.vertex_neighbors((y, x)).count_true().eq(1))); // Each tree is paired with one tent, and vice versa
            if trees[y][x] {
                solver.add_expr(is_tree.at((y, x)));
                solver.add_expr(!is_tent.at((y, x)));
            }
            else{
                solver.add_expr(!is_tree.at((y, x)));
            }
        }
    }

    solver.add_expr(!(is_tent.slice((..(h - 1), ..)) & is_tent.slice((1.., ..)))); // Star battle like constraints for tents
    solver.add_expr(!(is_tent.slice((.., ..(w - 1))) & is_tent.slice((.., 1..))));
    solver
        .add_expr(!(is_tent.slice((..(h - 1), ..(w - 1))) & is_tent.slice((1.., 1..))));
    solver
        .add_expr(!(is_tent.slice((..(h - 1), 1..)) & is_tent.slice((1.., ..(w - 1)))));


    solver.add_expr(is_tree.count_true().eq(is_tent.count_true()));
    solver.add_expr(is_tent.count_true().eq(&is_pair.horizontal.count_true() + &is_pair.vertical.count_true())); // There are as many pairs as there are tents and trees

    

    for y in 0..h {
        if let Some(clue) = &clue_vertical[y] {
            solver.add_expr(is_tent.slice_fixed_y((y, ..)).count_true().eq(clue[0]));
        }
    }
    for x in 0..w {
        if let Some(clue) = &clue_horizontal[x] {
            solver.add_expr(is_tent.slice_fixed_x((.., x)).count_true().eq(clue[0]));
        }
    }

    solver.irrefutable_facts().map(|f| (f.get(is_pair), f.get(&is_tent)))
}

type Problem = ((Vec<Option<Vec<i32>>>, Vec<Option<Vec<i32>>>), Vec<Vec<bool>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        OutsideSequences::new(Choice::new(vec![
            Box::new(Dict::new(-1, ".")),
            Box::new(HexInt),
        ])),
        ContextBasedGrid::new(Map::new(
            Choice::new(vec![
                Box::new(NumSpaces::new(0, 17)),
                Box::new(Spaces::new_with_maximum(None, 'i', 'z')),
            ]),
            |x: bool| match x {
                true => Some(Some(0)),
                false => None,
            },
            |n: Option<i32>| match n {
                Some(0) => Some(true),
                _ => Some(false),
            },
        )
    )))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0 .1.len();
    let width = problem.0 .0.len();
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
        let clue_vertical = vec![
            None,
            None,
            Some(vec![1]),
            None,
            None,
        ];
        let clue_horizontal = vec![
            Some(vec![3]),
            None,
            Some(vec![2]),
            None,
            None,
        ];
        let trees = crate::util::tests::to_bool_2d([
            [0, 0, 0, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 0, 1, 0, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 0, 0],
        ]);
        ((clue_vertical, clue_horizontal), trees)
    }

    #[test]
    fn test_tents_problem() {
        let (clues, trees) = problem_for_tests();
        let ans = solve_tents(&clues.0, &clues.1, &trees);
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
        let url = "https://pzprxs.vercel.app/p?tents/5/5/3g2j1hk1322313";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}