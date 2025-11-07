use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{problem_to_url, url_to_problem, Combinator, Grid, Map, MultiDigit};
use cspuz_rs::solver::Solver;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircleSquareClue {
    None,
    White,
    Black,
}

pub fn solve_circlesquare(clues: &[Vec<CircleSquareClue>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);
    graph::active_vertices_connected_2d(&mut solver, is_black);

    solver.add_expr(!is_black.conv2d_and((2, 2)));

    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_expr(
        (is_black.slice((.., ..(w - 1))) ^ is_black.slice((.., 1..))).iff(&is_border.vertical),
    );
    solver.add_expr(
        (is_black.slice((..(h - 1), ..)) ^ is_black.slice((1.., ..))).iff(&is_border.horizontal),
    );

    let num_up = &solver.int_var_2d((h, w), 0, h as i32 - 1);
    solver.add_expr(num_up.slice_fixed_y((0, ..)).eq(0));
    solver.add_expr(
        num_up.slice((1.., ..)).eq(is_border
            .horizontal
            .ite(0, num_up.slice((..(h - 1), ..)) + 1)),
    );
    let num_down = &solver.int_var_2d((h, w), 0, h as i32 - 1);
    solver.add_expr(num_down.slice_fixed_y((h - 1, ..)).eq(0));
    solver.add_expr(
        num_down
            .slice((..(h - 1), ..))
            .eq(is_border.horizontal.ite(0, num_down.slice((1.., ..)) + 1)),
    );
    let num_left = &solver.int_var_2d((h, w), 0, w as i32 - 1);
    solver.add_expr(num_left.slice_fixed_x((.., 0)).eq(0));
    solver.add_expr(
        num_left.slice((.., 1..)).eq(is_border
            .vertical
            .ite(0, num_left.slice((.., ..(w - 1))) + 1)),
    );
    let num_right = &solver.int_var_2d((h, w), 0, w as i32 - 1);
    solver.add_expr(num_right.slice_fixed_x((.., w - 1)).eq(0));
    solver.add_expr(
        num_right
            .slice((.., ..(w - 1)))
            .eq(is_border.vertical.ite(0, num_right.slice((.., 1..)) + 1)),
    );

    for x in 0..h {
        for y in 0..w {
            solver.add_expr(
                (!is_black.at((y, x))).imp(
                    (num_up.at((y, x)) + num_down.at((y, x)))
                        .eq(num_left.at((y, x)) + num_right.at((y, x))),
                ),
            );
        }
    }

    for y in 0..h {
        for x in 0..w {
            match clues[y][x] {
                CircleSquareClue::None => (),
                CircleSquareClue::White => solver.add_expr(!is_black.at((y, x))),
                CircleSquareClue::Black => solver.add_expr(is_black.at((y, x))),
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = Vec<Vec<CircleSquareClue>>;

fn combinator() -> impl Combinator<Vec<Vec<CircleSquareClue>>> {
    Grid::new(Map::new(
        MultiDigit::new(3, 3),
        |x: CircleSquareClue| {
            Some(match x {
                CircleSquareClue::None => 0,
                CircleSquareClue::White => 1,
                CircleSquareClue::Black => 2,
            })
        },
        |n: i32| match n {
            0 => Some(CircleSquareClue::None),
            1 => Some(CircleSquareClue::White),
            2 => Some(CircleSquareClue::Black),
            _ => None,
        },
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "circlesquare", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["circlesquare"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        vec![
            vec![CircleSquareClue::None, CircleSquareClue::None, CircleSquareClue::None, CircleSquareClue::None, CircleSquareClue::Black],
            vec![CircleSquareClue::White, CircleSquareClue::White, CircleSquareClue::None, CircleSquareClue::None, CircleSquareClue::White],
            vec![CircleSquareClue::None, CircleSquareClue::None, CircleSquareClue::Black, CircleSquareClue::None, CircleSquareClue::None],
            vec![CircleSquareClue::None, CircleSquareClue::Black, CircleSquareClue::Black, CircleSquareClue::None, CircleSquareClue::None],
            vec![CircleSquareClue::None, CircleSquareClue::None, CircleSquareClue::None, CircleSquareClue::White, CircleSquareClue::None],
        ]
    }

    #[test]
    fn test_circlesquare_problem() {
        let problem = problem_for_tests();
        let ans = solve_circlesquare(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 0, 1, 0, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_circlesquare_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?circlesquare/5/5/0799i8010";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
