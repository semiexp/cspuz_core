use crate::puzzles::heyawake;
use cspuz_rs::graph;
use cspuz_rs::serializer::{problem_to_url_with_context, url_to_problem, Context};
use cspuz_rs::solver::Solver;

pub fn solve_ayeheya(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Option<i32>],
) -> Option<Vec<Vec<Option<bool>>>> {
    assert!(all_room_symmetry(borders));
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    heyawake::add_constraints(&mut solver, is_black, borders, clues);

    let rooms = graph::borders_to_rooms(borders);
    for room in rooms {
        let mut room = room;
        room.sort();
        for i in 0..(room.len() / 2) {
            solver.add_expr(
                is_black
                    .at(room[i])
                    .iff(is_black.at(room[room.len() - 1 - i])),
            );
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

pub fn all_room_symmetry(borders: &graph::InnerGridEdges<Vec<Vec<bool>>>) -> bool {
    let rooms = graph::borders_to_rooms(borders);
    for room in rooms {
        let mut room = room;
        room.sort();
        assert!(!room.is_empty());
        let midpoint = (
            room[0].0 + room[room.len() - 1].0,
            room[0].1 + room[room.len() - 1].1,
        );
        for i in 1..room.len() {
            let y = room[i].0 + room[room.len() - 1 - i].0;
            let x = room[i].1 + room[room.len() - 1 - i].1;
            if midpoint != (y, x) {
                return false;
            }
        }
    }
    true
}

type Problem = heyawake::Problem;

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        heyawake::combinator(),
        "ayeheya",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(heyawake::combinator(), &["ayeheya"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 0, 0],
                [0, 0, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0],
                [1, 1, 1, 1, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 1, 1, 0, 1],
                [0, 1, 1, 0, 1],
                [0, 1, 1, 0, 1],
                [0, 1, 0, 1, 0],
                [0, 1, 0, 1, 0],
                [0, 1, 1, 1, 0],
            ]),
        };
        let clues = vec![
            None,
            Some(1),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        (borders, clues)
    }

    #[test]
    fn test_ayeheya_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_ayeheya(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 1, 0],
            [0, 0, 0, 1, 0, 0],
            [1, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_ayeheya_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?ayeheya/6/6/dddaae0c1s1sg1n";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
