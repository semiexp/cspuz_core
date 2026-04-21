use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice2, Combinator, Context, ContextBasedGrid,
    Dict, Map, MultiDigit, Optionalize, Rooms, Size, Tuple2,
};
use cspuz_rs::solver::{count_true, Solver};
use std::cmp::min;

pub fn solve_doubleback(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    holes: &Option<Vec<Vec<bool>>>,
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let h = borders.vertical.len();
    assert!(h > 0);
    let w = borders.vertical[0].len() + 1;

    // Making a copy of the borders for holes
    let mut borders_with_holes = graph::InnerGridEdges {
        horizontal: borders.horizontal.clone(),
        vertical: borders.vertical.clone(),
    };

    // If there are holes, add a border between cells with holes and cells with no holes
    if let Some(is_hole) = holes {
        for y in 0..h {
            for x in 0..w {
                if is_hole[y][x] ^ is_hole[y][min(x + 1, w - 1)] {
                    borders_with_holes.vertical[y][x] = true;
                }
                if is_hole[min(y + 1, h - 1)][x] ^ is_hole[y][x] {
                    borders_with_holes.horizontal[y][x] = true;
                }
            }
        }
    }

    let mut parity_diff = 0;
    for y in 0..h {
        for x in 0..w {
            if let Some(is_hole) = holes {
                if is_hole[y][x] {
                    continue;
                }
            }
            if (y + x) % 2 == 0 {
                parity_diff += 1;
            } else {
                parity_diff -= 1;
            }
        }
    }
    if parity_diff != 0 {
        return None;
    }
    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, is_line);

    for y in 0..h {
        for x in 0..w {
            if let Some(is_hole) = holes {
                solver.add_expr(is_passed.at((y, x)) ^ is_hole[y][x]);
            } else {
                solver.add_expr(is_passed.at((y, x)));
            }
        }
    }

    let rooms = graph::borders_to_rooms(&borders_with_holes);
    let mut room_id = vec![vec![0; w]; h];

    for i in 0..rooms.len() {
        for &(y, x) in &rooms[i] {
            room_id[y][x] = i;
        }
    }

    let mut room_entrance = vec![vec![]; rooms.len()];
    for y in 0..h {
        for x in 0..w {
            if y < h - 1 && room_id[y][x] != room_id[y + 1][x] {
                room_entrance[room_id[y][x]].push(is_line.vertical.at((y, x)));
                room_entrance[room_id[y + 1][x]].push(is_line.vertical.at((y, x)));
            }
            if x < w - 1 && room_id[y][x] != room_id[y][x + 1] {
                room_entrance[room_id[y][x]].push(is_line.horizontal.at((y, x)));
                room_entrance[room_id[y][x + 1]].push(is_line.horizontal.at((y, x)));
            }
        }
    }

    for i in 0..rooms.len() {
        // Check every room is entered twice
        if let Some(is_hole) = holes {
            // Unless it's a "hole" room
            if is_hole[rooms[i][0].0][rooms[i][0].1] {
                continue;
            }
        }
        solver.add_expr(count_true(&room_entrance[i]).eq(4));
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = (
    graph::InnerGridEdges<Vec<Vec<bool>>>,
    Option<Vec<Vec<bool>>>,
);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
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
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "doubleback",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["doubleback"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests1() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([[0, 0, 1, 1], [0, 0, 0, 0], [1, 1, 1, 1]]),
            vertical: crate::util::tests::to_bool_2d([[0, 0, 0], [0, 1, 0], [0, 1, 0], [0, 0, 0]]),
        };
        (borders, None)
    }

    fn problem_for_tests2() -> Problem {
        let mut holes = vec![vec![false; 4]; 4];
        holes[0][0] = true;
        holes[1][0] = true;
        holes[2][0] = true;
        holes[3][0] = true;
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]),
            vertical: crate::util::tests::to_bool_2d([[0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0]]),
        };
        (borders, Some(holes))
    }

    #[test]
    fn test_doubleback_problem1() {
        let (borders, holes) = problem_for_tests1();
        let ans = solve_doubleback(&borders, &holes);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        #[rustfmt::skip]
        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1],
                [0, 1, 0],
                [0, 0, 0],
                [1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_doubleback_problem2() {
        let (borders, holes) = problem_for_tests2();
        let ans = solve_doubleback(&borders, &holes);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        #[rustfmt::skip]
        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 1],
                [0, 1, 0],
                [0, 1, 0],
                [0, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 1],
                [0, 0, 1, 1],
                [0, 1, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_doubleback_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://puzz.link/p?doubleback/4/4/14063o"; // puzz.link example puzzle
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://puzz.link/p?doubleback/4/4/94g000h240";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
