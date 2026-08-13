use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, Dict, HexInt,
    Optionalize, RoomsWithValues, Size, Spaces,
};
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_detour(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Option<i32>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_turn = &solver.bool_var_2d((h, w));
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let rooms = graph::borders_to_rooms(borders);
    assert_eq!(rooms.len(), clues.len());

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, is_line);

    for y in 0..h {
        for x in 0..w {
            solver.add_expr(is_turn.at((y, x)).iff(
                (is_line.vertical.at_offset((y, x), (-1, 0), false)
                    | is_line.vertical.at_offset((y, x), (0, 0), false))
                    & (is_line.horizontal.at_offset((y, x), (0, -1), false)
                        | is_line.horizontal.at_offset((y, x), (0, 0), false)),
            ));

            solver.add_expr(is_passed.at((y, x)));
        }
    }

    for i in 0..rooms.len() {
        if let Some(n) = clues[i] {
            let mut cells = vec![];
            for &pt in &rooms[i] {
                cells.push(is_turn.at(pt));
            }
            if n >= 0 {
                solver.add_expr(count_true(cells).eq(n));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Option<i32>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(RoomsWithValues::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ])))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "detour",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["detour"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 1, 0, 1],
                [0, 1, 1, 1, 0],
                [1, 0, 1, 1, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 0, 1, 0],
                [1, 0, 0, 1],
                [1, 0, 0, 1],
                [0, 1, 0, 0],
            ]),
        };
        let clues = vec![Some(1), Some(2), Some(3), Some(-1)];
        (borders, clues)
    }

    #[test]
    fn test_detour_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_detour(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 1],
                [0, 1, 1, 1],
                [0, 1, 1, 1],
                [1, 1, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 0, 1],
                [1, 1, 0, 0, 0],
                [1, 0, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_detour_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?detour/5/4/56a0dem123.";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
