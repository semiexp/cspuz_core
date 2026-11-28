use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Choice2, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, Map, MultiDigit, Optionalize, Rooms, Size, Spaces, Tuple3,
};
use cspuz_rs::solver::Solver;

pub fn solve_cojun(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
    is_hole: &Option<Vec<Vec<bool>>>,
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = borders.base_shape();
    let mut room_borders = borders.clone();
    if let Some(holes) = is_hole {
        for y in 0..h {
            for x in 0..w {
                if x + 1 < w && holes[y][x] != holes[y][x + 1] {
                    room_borders.vertical[y][x] = true;
                }
                if y + 1 < h && holes[y][x] != holes[y + 1][x] {
                    room_borders.horizontal[y][x] = true;
                }
            }
        }
    }
    let rooms = graph::borders_to_rooms(&room_borders);
    let mut ranges = vec![vec![(1, 1); w]; h];
    for room in &rooms {
        for &(y, x) in room {
            ranges[y][x] = if is_hole.as_ref().is_some_and(|holes| holes[y][x]) {
                (0, 0)
            } else {
                (1, room.len() as i32)
            };
        }
    }

    let mut solver = Solver::new();
    let num = &solver.int_var_2d_from_ranges((h, w), &ranges);
    solver.add_answer_key_int(num);
    for room in &rooms {
        if ranges[room[0].0][room[0].1] == (0, 0) {
            continue;
        }
        let room_nums = num.select(room);
        for n in 1..=room.len() {
            solver.add_expr(room_nums.eq(n as i32).count_true().eq(1));
        }
    }
    for y in 0..h {
        for x in 0..w {
            if ranges[y][x] == (0, 0) {
                continue;
            }
            if let Some(n) = clues[y][x] {
                if n > 0 {
                    solver.add_expr(num.at((y, x)).eq(n));
                }
            }
            if x + 1 < w && ranges[y][x + 1] != (0, 0) {
                solver.add_expr(num.at((y, x)).ne(num.at((y, x + 1))));
            }
            if y + 1 < h && ranges[y + 1][x] != (0, 0) {
                if room_borders.horizontal[y][x] {
                    solver.add_expr(num.at((y, x)).ne(num.at((y + 1, x))));
                } else {
                    solver.add_expr(num.at((y, x)).gt(num.at((y + 1, x))));
                }
            }
        }
    }
    solver.irrefutable_facts().map(|f| f.get(num))
}

pub type Problem = (
    graph::InnerGridEdges<Vec<Vec<bool>>>,
    Vec<Vec<Option<i32>>>,
    Option<Vec<Vec<bool>>>,
);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple3::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
        Choice2::new(
            Optionalize::new(ContextBasedGrid::new(Map::new(
                MultiDigit::new(2, 5),
                |x: bool| Some(if x { 1 } else { 0 }),
                |n: i32| Some(n == 1),
            ))),
            Dict::new(None, ""),
        ),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (height, width) = problem.0.base_shape();
    problem_to_url_with_context(
        combinator(),
        "cojun",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["cojun"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tests::{serializer_test, to_bool_2d, to_option_2d};

    fn problem_for_tests() -> Problem {
        (
            graph::InnerGridEdges {
                horizontal: to_bool_2d([
                    [1, 1, 0, 0, 0],
                    [0, 1, 1, 1, 0],
                    [1, 0, 0, 0, 1],
                    [1, 1, 0, 0, 0],
                ]),
                vertical: to_bool_2d([
                    [0, 1, 1, 0],
                    [1, 0, 1, 0],
                    [1, 1, 1, 1],
                    [0, 1, 1, 0],
                    [0, 1, 1, 0],
                ]),
            },
            vec![
                vec![None, None, None, None, None],
                vec![None, None, None, None, None],
                vec![None, None, None, None, Some(3)],
                vec![None, None, None, Some(4), None],
                vec![None, None, None, None, None],
            ],
            None,
        )
    }

    #[test]
    fn test_cojun_problem() {
        let (borders, clues, is_hole) = problem_for_tests();
        assert_eq!(
            solve_cojun(&borders, &clues, &is_hole),
            Some(to_option_2d([
                [1, 2, 3, 2, 5],
                [2, 1, 2, 1, 4],
                [1, 2, 3, 5, 3],
                [3, 1, 2, 4, 2],
                [1, 2, 1, 3, 1],
            ]))
        );
        let mut impossible = clues;
        impossible[2][4] = Some(9);
        assert_eq!(solve_cojun(&borders, &impossible, &is_hole), None);
    }

    #[test]
    fn test_cojun_multiple_answers() {
        let borders = graph::InnerGridEdges {
            horizontal: vec![],
            vertical: vec![vec![false]],
        };
        let clues = vec![vec![None, None]];
        assert_eq!(
            solve_cojun(&borders, &clues, &None),
            Some(vec![vec![None, None]])
        );
    }

    #[test]
    fn test_cojun_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?cojun/5/5/dbr6oehot3i4l";
        serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
