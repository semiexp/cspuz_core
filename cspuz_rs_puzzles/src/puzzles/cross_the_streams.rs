use cspuz_rs::complex_constraints::japanese;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, Dict, HexInt,
    OutsideSequences, Size,
};
use cspuz_rs::solver::{BoolVarArray1D, Solver};

pub fn solve_cross_the_streams(
    clue_vertical: &[Option<Vec<i32>>],
    clue_horizontal: &[Option<Vec<i32>>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let h = clue_horizontal.len();
    let w = clue_vertical.len();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    graph::active_vertices_connected_2d(&mut solver, is_black);
    solver.add_expr(!(is_black.conv2d_and((2, 2))));

    let mut add_constraint = |target: BoolVarArray1D, clue: &[i32]| {
        let clue = compress_stars(clue);
        let maybe_absent = clue.iter().map(|&c| c == -1).collect::<Vec<_>>();
        let group_id = japanese(&mut solver, &target, &maybe_absent);

        for i in 0..clue.len() {
            let c = clue[i];
            if c > 0 {
                solver.add_expr((&target & (group_id.eq(i as i32))).count_true().eq(c));
            }
        }
    };

    for y in 0..h {
        if let Some(clue) = &clue_horizontal[y] {
            add_constraint(is_black.slice_fixed_y((y, ..)), clue);
        }
    }
    for x in 0..w {
        if let Some(clue) = &clue_vertical[x] {
            add_constraint(is_black.slice_fixed_x((.., x)), clue);
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

fn compress_stars(clues: &[i32]) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    for &c in clues {
        if c == -1 && !ret.is_empty() && *ret.last().unwrap() == -1 {
            continue;
        }
        ret.push(c);
    }
    ret
}

type Problem = (Vec<Option<Vec<i32>>>, Vec<Option<Vec<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(OutsideSequences::new(Choice::new(vec![
        Box::new(Dict::new(0, ".")),
        Box::new(Dict::new(-1, "0")),
        Box::new(HexInt),
    ])))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1.len();
    let width = problem.0.len();
    problem_to_url_with_context(
        combinator(),
        "cts",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["cts"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        let clue_vertical = vec![
            Some(vec![-1, 3, -1]),
            Some(vec![0, 0, -1]),
            Some(vec![-1]),
            Some(vec![-1]),
            Some(vec![-1, 2]),
            Some(vec![-1, 1]),
        ];
        let clue_horizontal = vec![
            None,
            Some(vec![0, 1, 0]),
            Some(vec![2, 0]),
            Some(vec![2, 2]),
            Some(vec![-1, 3, -1]),
        ];
        (clue_vertical, clue_horizontal)
    }

    #[test]
    fn test_cross_the_stream_problem() {
        let problem = problem_for_tests();
        let ans = solve_cross_the_streams(&problem.0, &problem.1);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 1, 1, 1, 1],
            [1, 0, 0, 1, 0, 1],
            [1, 1, 0, 1, 0, 0],
            [0, 1, 1, 0, 1, 1],
            [0, 0, 1, 1, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_cross_the_streams_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?cts/6/5/0300..0h0h20g10j.1..2g22g030";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
