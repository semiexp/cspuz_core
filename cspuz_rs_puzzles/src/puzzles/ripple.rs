use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, HexInt, Optionalize, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_ripple(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = borders.base_shape();

    let rooms = graph::borders_to_rooms(borders);
    let mut ranges = vec![vec![(1, 1); w]; h];

    for room in &rooms {
        for &(y, x) in room {
            ranges[y][x] = (1, room.len() as i32);
        }
    }

    let mut solver = Solver::new();
    let num = &solver.int_var_2d_from_ranges((h, w), &ranges);
    solver.add_answer_key_int(num);

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                if c > 0 {
                    solver.add_expr(num.at((y, x)).eq(c));
                }
            }
        }
    }

    for room in &rooms {
        let room_nums = num.select(room);
        for i in 1..=room.len() {
            solver.add_expr(room_nums.eq(i as i32).count_true().eq(1));
        }
    }

    for y in 0..h {
        for x1 in 0..w {
            for x2 in (x1 + 1)..w {
                let lo = (x2 - x1) as i32;
                let hi = ranges[y][x1].1.min(ranges[y][x2].1);

                for i in lo..=hi {
                    solver.add_expr(!(num.at((y, x1)).eq(i) & num.at((y, x2)).eq(i)));
                }
            }
        }
    }

    for x in 0..w {
        for y1 in 0..h {
            for y2 in (y1 + 1)..h {
                let lo = (y2 - y1) as i32;
                let hi = ranges[y1][x].1.min(ranges[y2][x].1);

                for i in lo..=hi {
                    solver.add_expr(!(num.at((y1, x)).eq(i) & num.at((y2, x)).eq(i)));
                }
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
        "ripple",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["ripple"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0, 1, 0],
                [1, 1, 1, 0, 0],
                [1, 1, 1, 1, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [1, 0, 1, 0],
                [1, 0, 1, 1],
                [0, 1, 0, 1],
                [0, 0, 0, 1],
            ]),
        };

        let clues = vec![
            vec![None, Some(4), None, Some(3), None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, Some(1)],
        ];

        (borders, clues)
    }

    #[test]
    fn test_ripple_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_ripple(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [1, 4, 2, 3, 5],
            [2, 3, 1, 2, 4],
            [1, 2, 3, 1, 2],
            [3, 1, 2, 4, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_ripple_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?ripple/5/4/ld8g2sug4g3u1";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
