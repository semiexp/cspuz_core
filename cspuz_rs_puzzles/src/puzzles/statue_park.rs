use crate::util;
use cspuz_rs::graph;
use cspuz_rs::polyomino::{
    normalize_and_merge_pieces, pentominoes, polyomino_placement, tetrominoes, PiecesCombinator,
};
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Combinator, Context, ContextBasedGrid, Map,
    MultiDigit, Size, Tuple2,
};
use cspuz_rs::solver::Solver;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatueParkClue {
    None,
    White,
    Black,
}

pub fn solve_statue_park(
    board: &[Vec<StatueParkClue>],
    pieces: &[Vec<Vec<bool>>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(board);
    let (pieces_merged, cnts) = normalize_and_merge_pieces(pieces);

    let mut solver = Solver::new();
    let is_block = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_block);
    graph::active_vertices_connected_2d(&mut solver, !is_block);

    polyomino_placement(&mut solver, is_block, &pieces_merged, &cnts, &cnts, false);

    for y in 0..h {
        for x in 0..w {
            match board[y][x] {
                StatueParkClue::None => (),
                StatueParkClue::White => solver.add_expr(!is_block.at((y, x))),
                StatueParkClue::Black => solver.add_expr(is_block.at((y, x))),
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_block))
}

fn double_tetrominoes() -> Vec<Vec<Vec<bool>>> {
    let mut ret = vec![];
    for p in tetrominoes() {
        ret.push(p.clone());
        ret.push(p.clone());
    }
    ret
}

type Problem = (Vec<Vec<StatueParkClue>>, Vec<Vec<Vec<bool>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        ContextBasedGrid::new(Map::new(
            MultiDigit::new(3, 3),
            |x: StatueParkClue| {
                Some(match x {
                    StatueParkClue::None => 0,
                    StatueParkClue::White => 1,
                    StatueParkClue::Black => 2,
                })
            },
            |n: i32| match n {
                0 => Some(StatueParkClue::None),
                1 => Some(StatueParkClue::White),
                2 => Some(StatueParkClue::Black),
                _ => None,
            },
        )),
        PiecesCombinator::new(vec![
            (tetrominoes(), b"//t"),
            (double_tetrominoes(), b"//d"),
            (pentominoes(), b"//p"),
        ]),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url_with_context(
        combinator(),
        "statuepark",
        problem.clone(),
        &Context::sized(problem.0.len(), problem.0[0].len()),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["statuepark"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> (Vec<Vec<StatueParkClue>>, Vec<Vec<Vec<bool>>>) {
        // https://puzz.link/p?statuepark/7/6/0l050060i0107i//t
        let mut ret = vec![vec![StatueParkClue::None; 7]; 6];
        ret[0][3] = StatueParkClue::Black;
        ret[0][4] = StatueParkClue::White;
        ret[1][3] = StatueParkClue::White;
        ret[1][4] = StatueParkClue::Black;
        ret[2][5] = StatueParkClue::Black;
        ret[3][3] = StatueParkClue::Black;
        ret[4][4] = StatueParkClue::White;
        ret[5][2] = StatueParkClue::Black;
        ret[5][3] = StatueParkClue::White;
        ret[5][4] = StatueParkClue::Black;

        (ret, tetrominoes())
    }

    fn problem_for_tests2() -> (Vec<Vec<StatueParkClue>>, Vec<Vec<Vec<bool>>>) {
        // https://puzz.link/p?statuepark/6/5/0000591i00/2/23lg/22u
        let mut ret = vec![vec![StatueParkClue::None; 6]; 5];
        ret[2][1] = StatueParkClue::White;
        ret[2][2] = StatueParkClue::Black;
        ret[2][3] = StatueParkClue::White;
        ret[3][2] = StatueParkClue::White;
        ret[3][3] = StatueParkClue::Black;

        let pieces = vec![
            vec![vec![true, false], vec![true, false], vec![true, true]],
            vec![vec![true, true], vec![true, true]],
        ];
        (ret, pieces)
    }

    #[test]
    fn test_statue_park_problem1() {
        let (board, pieces) = problem_for_tests1();
        let ans = solve_statue_park(&board, &pieces);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 0],
            [0, 1, 1, 0, 1, 1, 0],
            [0, 0, 1, 1, 0, 0, 0],
            [1, 0, 0, 0, 0, 1, 0],
            [1, 1, 1, 0, 1, 1, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_statue_park_problem2() {
        let (board, pieces) = problem_for_tests2();
        let ans = solve_statue_park(&board, &pieces);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 0],
            [0, 0, 0, 1, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_statue_park_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://puzz.link/p?statuepark/7/6/0l050060i0107i//t";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://puzz.link/p?statuepark/6/5/0000591i00/2/23lg/22u";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
