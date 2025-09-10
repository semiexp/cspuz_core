use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, HexInt, Optionalize, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_nanro(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = borders.base_shape();
    let mut solver = Solver::new();

    let rooms = graph::borders_to_rooms(borders);
    let is_num = &solver.bool_var_2d((h, w));
    let mut ranges = vec![vec![(1, 1); w]; h];
    let room_num = &solver.int_var_2d((rooms.len(), 1), 0, (h * w) as i32);

    graph::active_vertices_connected_2d(&mut solver, is_num);

    solver.add_expr(!is_num.conv2d_and((2, 2)));

    for room in &rooms {
        for &(y, x) in room {
            ranges[y][x] = (0, room.len() as i32);
        }
    }

    let num = &solver.int_var_2d_from_ranges((h, w), &ranges);
    solver.add_answer_key_int(num);

    solver.add_expr(num.ge(1).iff(is_num));

    for i in 0..rooms.len() {
        for &(y, x) in &rooms[i] {
            solver.add_expr(
                is_num
                    .at((y, x))
                    .imp(room_num.at((i, 0)).eq(num.at((y, x)))),
            );
        }
        solver.add_expr(
            is_num
                .select(&rooms[i])
                .count_true()
                .eq(room_num.at((i, 0))),
        );
        solver.add_expr(is_num.select(&rooms[i]).count_true().ge(1));
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                if c == -1 {
                    solver.add_expr(num.at((y, x)).ne(0));
                } else {
                    solver.add_expr(num.at((y, x)).eq(c));
                }
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 && borders.horizontal[y][x] {
                solver.add_expr(
                    (is_num.at((y, x)) & is_num.at((y + 1, x)))
                        .imp(num.at((y + 1, x)).ne(num.at((y, x)))),
                );
            }
            if x < w - 1 && borders.vertical[y][x] {
                solver.add_expr(
                    (is_num.at((y, x)) & is_num.at((y, x + 1)))
                        .imp(num.at((y, x + 1)).ne(num.at((y, x)))),
                );
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
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "nanro",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["nanro"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([[1, 1, 1, 0], [0, 0, 0, 0], [0, 1, 1, 1]]),
            vertical: crate::util::tests::to_bool_2d([[0, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 0]]),
        };

        let clues = vec![
            vec![None, None, None, Some(1)],
            vec![Some(3), None, None, None],
            vec![None, None, None, None],
            vec![None, Some(1), None, None],
        ];

        (borders, clues)
    }

    #[test]
    fn test_nanro_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_nanro(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [0, 2, 2, 1],
            [3, 1, 0, 0],
            [3, 0, 0, 0],
            [3, 1, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_nanro_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?nanro/4/4/6r0s1oi13n1h"; // Example puzzle on puzz.link
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
