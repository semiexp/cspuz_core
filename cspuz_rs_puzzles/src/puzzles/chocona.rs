use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, HexInt, Optionalize,
    RoomsWithValues, Size, Spaces,
};
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_chocona(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Option<i32>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let rooms = graph::borders_to_rooms(borders);
    assert_eq!(rooms.len(), clues.len());

    for i in 0..rooms.len() {
        if let Some(n) = clues[i] {
            let mut cells = vec![];
            for &pt in &rooms[i] {
                cells.push(is_black.at(pt))
            }
            solver.add_expr(count_true(cells).eq(n));
        }
    }

    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            solver.add_expr(is_black.slice((y..(y + 2), x..(x + 2))).count_true().ne(3));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Option<i32>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(RoomsWithValues::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
    ])))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "chocona",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["chocona"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: vec![
                vec![false, false, false, true],
                vec![true, true, true, true],
                vec![false, false, true, true],
            ],
            vertical: vec![
                vec![false, true, true],
                vec![false, true, false],
                vec![true, true, false],
                vec![true, false, false],
            ],
        };
        let clues = vec![Some(1), Some(3), None, Some(2), Some(3), None];
        (borders, clues)
    }

    #[test]
    fn test_chocona_problem() {
        let problem = problem_for_tests();
        let ans = solve_chocona(&problem.0, &problem.1);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 0, 1, 1],
            [0, 0, 1, 1],
            [1, 1, 0, 0],
            [1, 1, 0, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_chocona_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?chocona/4/4/dd03so13g23g";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
