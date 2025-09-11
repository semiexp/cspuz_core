use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{problem_to_url, url_to_problem, Combinator, Grid, Map, MultiDigit};
use cspuz_rs::solver::Solver;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YinYangClue {
    None,
    White,
    Black,
}

pub fn solve_yinyang(clues: &[Vec<YinYangClue>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    graph::active_vertices_connected_2d(&mut solver, is_black);
    solver.add_expr(!is_black.conv2d_and((2, 2)));

    graph::active_vertices_connected_2d(&mut solver, !is_black);
    solver.add_expr(!(!is_black).conv2d_and((2, 2)));

    for y in 0..h {
        for x in 0..w {
            let p = (y, x);
            match clues[y][x] {
                YinYangClue::None => (),
                YinYangClue::White => {
                    solver.add_expr(!is_black.at(p));
                }
                YinYangClue::Black => {
                    solver.add_expr(is_black.at(p));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = Vec<Vec<YinYangClue>>;

fn combinator() -> impl Combinator<Vec<Vec<YinYangClue>>> {
    Grid::new(Map::new(
        MultiDigit::new(3, 3),
        |x: YinYangClue| {
            Some(match x {
                YinYangClue::None => 0,
                YinYangClue::White => 1,
                YinYangClue::Black => 2,
            })
        },
        |n: i32| match n {
            0 => Some(YinYangClue::None),
            1 => Some(YinYangClue::White),
            2 => Some(YinYangClue::Black),
            _ => None,
        },
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "yinyang", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["yinyang"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Vec<Vec<YinYangClue>> {
        let mut ret = vec![vec![YinYangClue::None; 6]; 6];
        ret[0][1] = YinYangClue::Black;
        ret[0][3] = YinYangClue::White;
        ret[0][5] = YinYangClue::White;
        ret[1][2] = YinYangClue::White;
        ret[1][4] = YinYangClue::Black;
        ret[2][1] = YinYangClue::Black;
        ret[2][3] = YinYangClue::White;
        ret[2][5] = YinYangClue::Black;
        ret[3][2] = YinYangClue::Black;
        ret[3][4] = YinYangClue::White;
        ret[4][3] = YinYangClue::White;
        ret
    }

    #[test]
    fn test_yinyang_problem() {
        let problem = problem_for_tests();
        let ans = solve_yinyang(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 1, 0, 0, 0],
            [1, 0, 0, 0, 1, 1],
            [1, 1, 1, 0, 0, 1],
            [1, 0, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_yinyang_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?yinyang/6/6/6a166b230900";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
