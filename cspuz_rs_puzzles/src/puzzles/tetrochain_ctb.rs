use std::collections::BTreeMap;

use cspuz_rs::graph;
use cspuz_rs::serializer::{
    get_kudamono_url_info_detailed, parse_kudamono_dimension, problem_to_url_with_context_pzprxs,
    url_to_problem, Choice, Combinator, Context, DecInt, Dict, HexInt, KudamonoBorder,
    KudamonoGrid, Optionalize, PrefixAndSuffix, RoomsWithValues, Size, Spaces,
};
use cspuz_rs::solver::{count_true, Solver};

use crate::puzzles::tetrochain_common;

pub fn solve_tetrochain_ctb(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Option<i32>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let rooms = graph::borders_to_rooms(&borders);
    for i in 0..rooms.len() {
        if let Some(n) = clues[i] {
            solver.add_expr(count_true(is_black.select(&rooms[i])).eq(n));
        }
    }

    tetrochain_common::add_tetrochain_constraints(&mut solver, is_black, Some(borders));

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

pub(super) type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Option<i32>>);

pub(super) fn combinator() -> impl Combinator<Problem> {
    Size::new(RoomsWithValues::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ])))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context_pzprxs(
        combinator(),
        "tetroctb",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    if let Some(info) = get_kudamono_url_info_detailed(url) {
        return deserialize_problem_kudamono(&info);
    }

    url_to_problem(combinator(), &["tetroctb"], url)
}

pub fn deserialize_problem_kudamono(parsed: &BTreeMap<String, &str>) -> Option<Problem> {
    let (width, height) = parse_kudamono_dimension(parsed.get("W")?)?;

    let ctx = Context::sized_with_kudamono_mode(height, width, true);

    let raw_clues;
    if let Some(p) = parsed.get("L") {
        let clues_combinator = KudamonoGrid::new(
            Optionalize::new(PrefixAndSuffix::new("(", DecInt, ")")),
            None,
        );
        raw_clues = clues_combinator.deserialize(&ctx, p.as_bytes())?.1.pop()?;
    } else {
        raw_clues = vec![vec![None; width]; height];
    }

    let border;
    if let Some(p) = parsed.get("SIE") {
        border = KudamonoBorder.deserialize(&ctx, p.as_bytes())?.1.pop()?;
    } else {
        border = graph::InnerGridEdges {
            horizontal: vec![vec![false; width]; height - 1],
            vertical: vec![vec![false; width - 1]; height],
        };
    }

    let rooms = graph::borders_to_rooms(&border);
    let mut clues = vec![None; rooms.len()];
    for (i, room) in rooms.iter().enumerate() {
        for &(y, x) in room {
            if let Some(n) = raw_clues[y][x] {
                if let Some(m) = clues[i] {
                    if m != n {
                        return None;
                    }
                } else {
                    clues[i] = Some(n);
                }
            }
        }
    }

    Some((border, clues))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 1, 1, 0, 1, 0],
                [1, 0, 0, 1, 1, 0, 1],
                [0, 1, 1, 0, 0, 0, 0],
                [1, 1, 1, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 0, 0, 1, 1, 0],
                [1, 0, 1, 0, 1, 1],
                [1, 0, 1, 0, 0, 0],
                [1, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0],
            ]),
        };

        let clues = vec![Some(4), Some(1), Some(3), Some(0), None, Some(1), Some(1)];

        (borders, clues)
    }

    #[test]
    fn test_tetrochain_ctb_problem() {
        let problem = problem_for_tests();
        let ans = solve_tetrochain_ctb(&problem.0, &problem.1);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 0, 1, 0, 1, 1],
            [1, 0, 0, 1, 0, 1, 1],
            [1, 0, 0, 1, 1, 0, 0],
            [0, 1, 1, 0, 0, 0, 0],
            [0, 0, 1, 1, 0, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_tetrochain_ctb_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?tetroctb/7/5/3at208ekqoe04130g11";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }

    #[test]
    fn test_tetrochain_ctb_serializer_kudamono() {
        let problem = problem_for_tests();
        let url = "https://pedros.works/kudamono/player?W=7x5&L=(1)0(1)2(4)2(0)4(1)16(3)5&SIE=1RU2RURRRU4RRD14RRUU1DDLLU12RDR&G=tetrochain-ctb";
        assert_eq!(deserialize_problem(url), Some(problem));
    }
}
