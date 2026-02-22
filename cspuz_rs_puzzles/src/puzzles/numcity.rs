use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, Optionalize, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

fn limit(size: usize) -> i32 {
    for i in 1.. {
        if i * (i - 1) / 2 > size {
            return (i - 1) as i32;
        }
    }
    panic!();
}

pub fn solve_numcity(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = borders.base_shape();

    let rooms = graph::borders_to_rooms(borders);
    let mut room_limits = vec![];
    let mut ranges = vec![vec![(1, 1); w]; h];

    for room in &rooms {
        let lim = limit(room.len());
        room_limits.push(lim);
        for &(y, x) in room {
            ranges[y][x] = (1, lim);
        }
    }

    let mut solver = Solver::new();
    let num = &solver.int_var_2d_from_ranges((h, w), &ranges);
    solver.add_answer_key_int(num);

    for i in 0..rooms.len() {
        let room = &rooms[i];

        let mut sizes = vec![];
        for j in 1..=room_limits[i] {
            let v = solver.int_var(0, (room.len() as i32 - j * (j - 1) / 2) / j);
            solver.add_expr(num.select(room).eq(j).count_true().eq(&v));
            sizes.push(v);
        }

        for j in 1..sizes.len() {
            solver.add_expr((sizes[j - 1].eq(0) & sizes[j].eq(0)) | sizes[j - 1].gt(&sizes[j]));
        }
        for j in 1..=room_limits[i] {
            graph::active_vertices_connected_2d_region(&mut solver, num.eq(j), room);
        }
    }

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 && borders.horizontal[y][x] {
                solver.add_expr(num.at((y, x)).ne(num.at((y + 1, x))));
            }
            if x < w - 1 && borders.vertical[y][x] {
                solver.add_expr(num.at((y, x)).ne(num.at((y, x + 1))));
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                if c > 0 {
                    solver.add_expr(num.at((y, x)).eq(c));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(num))
}

pub type Problem = (Vec<Vec<Option<i32>>>, graph::InnerGridEdges<Vec<Vec<bool>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
        Rooms,
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1.vertical.len();
    let width = problem.1.vertical[0].len() + 1;
    problem_to_url_with_context_pzprxs(
        combinator(),
        "numcity",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["numcity"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0],
                [1, 1, 1, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 1, 0, 1, 0],
                [1, 1, 0, 1, 0],
                [0, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
            ]),
        };

        let clues = vec![
            vec![None, Some(1), None, None, None, Some(2)],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, Some(4), None, None],
        ];

        (clues, borders)
    }

    #[test]
    fn test_ripple_problem() {
        let (clues, borders) = problem_for_tests();
        let ans = solve_numcity(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [1, 1, 2, 1, 2, 2],
            [2, 3, 2, 1, 2, 1],
            [1, 1, 3, 1, 2, 1],
            [1, 2, 2, 3, 3, 1],
            [2, 1, 1, 4, 1, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_ripple_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?numcity/6/5/g1i2zg4haqa44881jg";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
