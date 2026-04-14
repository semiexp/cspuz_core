use crate::util;

use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    HexInt, Map, MultiDigit, Optionalize, OutsideCells2, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnakeClue {
    None,
    White,
    Black,
}

pub fn solve_snake(
    board: &[Vec<SnakeClue>],
    clue_vertical: &[Option<i32>],
    clue_horizontal: &[Option<i32>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(board);

    let mut solver = Solver::new();
    let is_snake = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_snake);
    let is_deadend = &solver.bool_var_2d((h, w));

    graph::active_vertices_connected_2d(&mut solver, is_snake);
    // No checkerboards
    solver.add_expr(
        !(is_snake.slice((..(h - 1), ..(h - 1)))
            & is_snake.slice((1.., 1..))
            & !is_snake.slice((..(h - 1), 1..))
            & !is_snake.slice((1.., ..(h - 1)))),
    );
    solver.add_expr(
        !(!is_snake.slice((..(h - 1), ..(h - 1)))
            & !is_snake.slice((1.., 1..))
            & is_snake.slice((..(h - 1), 1..))
            & is_snake.slice((1.., ..(h - 1)))),
    );

    for y in 0..h {
        if let Some(n) = &clue_horizontal[y] {
            let row = is_snake.slice_fixed_y((y, ..));
            solver.add_expr(row.count_true().eq(*n));
        }
    }
    for x in 0..w {
        if let Some(n) = &clue_vertical[x] {
            let col = is_snake.slice_fixed_x((.., x));
            solver.add_expr(col.count_true().eq(*n));
        }
    }

    for y in 0..h {
        for x in 0..w {
            let p = (y, x);
            solver.add_expr(
                (is_snake.four_neighbors(p).count_true().eq(1) & is_snake.at(p))
                    .iff(is_deadend.at(p)),
            );
            solver.add_expr(
                is_snake
                    .at(p)
                    .imp(is_snake.four_neighbors(p).count_true().le(2)),
            );
            match board[y][x] {
                SnakeClue::None => (),
                SnakeClue::White => {
                    solver.add_expr(is_snake.four_neighbors(p).count_true().eq(2));
                    solver.add_expr(is_snake.at(p));
                }
                SnakeClue::Black => {
                    solver.add_expr(is_snake.four_neighbors(p).count_true().eq(1));
                    solver.add_expr(is_snake.at(p));
                }
            }
        }
    }

    solver.add_expr(is_deadend.count_true().eq(2));

    solver.irrefutable_facts().map(|f| f.get(is_snake))
}

pub type Problem = (Vec<Vec<SnakeClue>>, (Vec<Option<i32>>, Vec<Option<i32>>));

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        ContextBasedGrid::new(Map::new(
            MultiDigit::new(3, 3),
            |x: SnakeClue| {
                Some(match x {
                    SnakeClue::None => 0,
                    SnakeClue::White => 1,
                    SnakeClue::Black => 2,
                })
            },
            |n: i32| match n {
                0 => Some(SnakeClue::None),
                1 => Some(SnakeClue::White),
                2 => Some(SnakeClue::Black),
                _ => None,
            },
        )),
        OutsideCells2::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1 .1.len();
    let width = problem.1 .0.len();

    problem_to_url_with_context(
        combinator(),
        "snake",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["snake"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        let clue_vertical = vec![None, Some(1), None, None];
        let clue_horizontal = vec![Some(4), None, None, None];
        let mut board = vec![vec![SnakeClue::None; 4]; 4];
        board[1][0] = SnakeClue::Black;
        board[2][2] = SnakeClue::White;
        (board, (clue_vertical, clue_horizontal))
    }

    #[test]
    fn test_snake_problem() {
        let (board, (clue_vertical, clue_horizontal)) = problem_for_tests();
        let ans = solve_snake(&board, &clue_vertical, &clue_horizontal);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 1, 1],
            [1, 0, 0, 1],
            [0, 0, 1, 1],
            [0, 0, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_snake_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?snake/4/4/060300g1h4i";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
