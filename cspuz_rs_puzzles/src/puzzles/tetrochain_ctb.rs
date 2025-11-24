use cspuz_rs::graph;
use cspuz_rs::serializer::{
    get_kudamono_url_info_detailed, parse_kudamono_dimension, Combinator, Context, DecInt,
    KudamonoBorder, KudamonoGrid, Optionalize, PrefixAndSuffix,
};
use cspuz_rs::solver::{count_true, Solver};

use crate::puzzles::tetrochain_common;

pub fn solve_tetrochain_ctb(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>], // clue on a cell (not region)
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let rooms = graph::borders_to_rooms(&borders);
    for i in 0..rooms.len() {
        let mut clue: Option<i32> = None;

        for &(y, x) in &rooms[i] {
            if let Some(c) = clues[y][x] {
                if let Some(cc) = clue {
                    if cc != c {
                        return None;
                    }
                } else {
                    clue = Some(c);
                }
            }
        }

        if let Some(n) = clue {
            solver.add_expr(count_true(is_black.select(&rooms[i])).eq(n));
        }
    }

    tetrochain_common::add_tetrochain_constraints(&mut solver, is_black, Some(borders));

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let parsed = get_kudamono_url_info_detailed(url)?;
    let (width, height) = parse_kudamono_dimension(parsed.get("W")?)?;

    let ctx = Context::sized_with_kudamono_mode(height, width, true);

    let clues;
    if let Some(p) = parsed.get("L") {
        let clues_combinator = KudamonoGrid::new(
            Optionalize::new(PrefixAndSuffix::new("(", DecInt, ")")),
            None,
        );
        clues = clues_combinator.deserialize(&ctx, p.as_bytes())?.1.pop()?;
    } else {
        clues = vec![vec![None; width]; height];
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

        let clues = vec![
            vec![Some(4), None, None, None, Some(1), Some(3), None],
            vec![None, Some(0), None, None, None, None, None],
            vec![Some(1), None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None],
            vec![Some(1), None, None, None, None, None, None],
        ];

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
        let url = "https://pedros.works/kudamono/player?W=7x5&L=(1)0(1)2(4)2(0)4(1)16(3)5&SIE=1RU2RURRRU4RRD14RRUU1DDLLU12RDR&G=tetrochain-ctb";
        assert_eq!(deserialize_problem(url), Some(problem));
    }
}
