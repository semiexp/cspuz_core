use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_paintarea(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    // all black / all white in each region
    for y in 0..h {
        for x in 0..(w - 1) {
            if !borders.vertical[y][x] {
                solver.add_expr(is_black.at((y, x)).iff(is_black.at((y, x + 1))));
            }
        }
    }
    for y in 0..(h - 1) {
        for x in 0..w {
            if !borders.horizontal[y][x] {
                solver.add_expr(is_black.at((y, x)).iff(is_black.at((y + 1, x))));
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                if c >= 0 {
                    solver.add_expr(is_black.four_neighbors((y, x)).count_true().eq(c));
                }
            }
        }
    }

    // black cells are connected
    graph::active_vertices_connected_2d(&mut solver, is_black);

    // no 2x2 all-black/white cells
    solver.add_expr(!(is_black.conv2d_and((2, 2))));
    solver.add_expr(is_black.conv2d_or((2, 2)));

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

pub(super) fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Spaces::new(None, 'a')),
            Box::new(Dict::new(Some(-1), ".")),
            Box::new(Dict::new(Some(0), "0")),
            Box::new(Dict::new(Some(1), "1")),
            Box::new(Dict::new(Some(2), "2")),
            Box::new(Dict::new(Some(3), "3")),
            Box::new(Dict::new(Some(4), "4")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "paintarea",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["paintarea"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 1, 1, 1],
                [1, 1, 1, 1, 0],
                [0, 1, 1, 1, 0],
                [1, 0, 0, 1, 1],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [1, 1, 0, 0],
                [1, 1, 0, 1],
                [1, 0, 1, 1],
                [1, 1, 1, 1],
                [0, 1, 1, 0],
            ]),
        };

        let mut clues = vec![vec![None; 5]; 5];
        clues[1][1] = Some(4);
        clues[3][3] = Some(1);

        (borders, clues)
    }

    #[test]
    fn test_paintarea_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_paintarea(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 0, 0, 0],
            [1, 0, 1, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 1, 1, 0],
            [0, 0, 1, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_paintarea_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?paintarea/5/5/pmvmfuejf4k1f";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
