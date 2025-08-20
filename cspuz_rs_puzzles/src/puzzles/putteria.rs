use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, HexInt, Optionalize, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_putteria(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = borders.base_shape();

    let rooms = graph::borders_to_rooms(borders);
    let mut ranges: Vec<i32> = Vec::new();
    let mut max_number = 0;

    for room in &rooms {
        // Log all possible numbers
        ranges.push(room.len() as i32);
        if max_number < room.len() as i32 {
            max_number = room.len() as i32;
        }
    }

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((h, w), -2, max_number);
    solver.add_answer_key_int(num);

    // Check no duplicates in rows
    for i in 0..h {
        for j in &ranges {
            solver.add_expr(num.slice_fixed_x((.., i)).eq(*j).count_true().le(1));
        }
    }

    for i in 0..w {
        for j in &ranges {
            solver.add_expr(num.slice_fixed_y((i, ..)).eq(*j).count_true().le(1));
        }
    }

    // Check no adjacent
    solver.add_expr(!(num.slice((..(h - 1), ..)).ge(1) & num.slice((1.., ..)).ge(1)));
    solver.add_expr(!(num.slice((.., ..(w - 1))).ge(1) & num.slice((.., 1..)).ge(1)));

    // Check no duplicate in rooms
    for room in &rooms {
        let room_nums = num.select(room);
        solver.add_expr(room_nums.eq(room.len() as i32).count_true().eq(1)); // One cell has the number
        solver.add_expr(room_nums.eq(room.len() as i32) | room_nums.eq(-2)); // The rest are empty
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                solver.add_expr(num.at((y, x)).eq(c));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(num))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Dict::new(Some(-2), ".")),
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "putteria",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["putteria"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0],
                [1, 1, 1, 1, 1, 1],
                [0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 1, 1],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [1, 0, 1, 1, 0],
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 0],
                [1, 1, 1, 1, 0],
                [1, 1, 1, 0, 0],
            ]),
        };

        let clues = vec![
            vec![None, None, None, Some(-2), None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, Some(-2)],
            vec![None, None, None, None, None, None],
        ];

        (borders, clues)
    }

    #[test]
    fn test_putteria_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_putteria(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [2, -2, -2, -2, -2, 4],
            [-2, 2, -2, 3, -2, -2],
            [1, -2, 4, -2, -2, 2],
            [-2, 3, -2, 1, -2, -2],
            [-2, -2, 3, -2, 4, -2],
            [3, -2, -2, 4, -2, -2],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_putteria_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?putteria/6/6/mvvuus8o7s83i.zk.l"; // Credits to botaku
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
