use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Choice2, Combinator, Context,
    ContextBasedGrid, DecInt, Dict, Map, MultiDigit, Optionalize, PrefixAndSuffix, Size, Tuple3,
};
use cspuz_rs::solver::Solver;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsowatariClue {
    None,
    White,
    Black,
}

pub fn solve_isowatari(
    cluster_size: i32,
    clues: &[Vec<IsowatariClue>],
    is_hole: &Option<Vec<Vec<bool>>>,
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(clues);
    let mut solver = Solver::new();
    // Using int var instead of bool var for holes
    let is_black = &solver.int_var_2d((h, w), -1, 1);
    solver.add_answer_key_int(is_black);

    let is_white = &solver.bool_var_2d((h, w));
    solver.add_expr(is_white.iff(is_black.eq(0)));
    solver.add_expr(!is_white.conv2d_and((2, 2)));
    graph::active_vertices_connected_2d(&mut solver, is_white);

    for y in 0..h {
        for x in 0..w {
            match clues[y][x] {
                IsowatariClue::None => (),
                IsowatariClue::White => solver.add_expr(is_black.at((y, x)).eq(0)),
                IsowatariClue::Black => solver.add_expr(is_black.at((y, x)).eq(1)),
            }
            if let Some(holes) = is_hole {
                solver.add_expr(is_black.at((y, x)).eq(-1) ^ !holes[y][x]);
            } else {
                solver.add_expr(is_black.at((y, x)).ne(-1))
            }
            let connected = &solver.bool_var_2d((h, w));
            for y2 in 0..h {
                for x2 in 0..w {
                    if y == y2 && x == x2 {
                        solver.add_expr(connected.at((y2, x2)));
                    } else {
                        solver.add_expr(connected.at((y2, x2)).imp(is_black.at((y2, x2)).eq(1)));
                    }
                }
            }
            solver
                .add_expr((is_black.at((y, x)).eq(1)).imp(connected.count_true().eq(cluster_size)));
            graph::active_vertices_connected_2d(&mut solver, connected);

            for nb in connected.four_neighbor_indices((y, x)) {
                solver.add_expr(is_black.at(nb).eq(1).imp(connected.at(nb)));
            }
            solver.add_expr(
                (is_black.eq(1).slice((1.., ..)) & is_black.eq(1).slice((..(h - 1), ..))).imp(
                    connected
                        .slice((1.., ..))
                        .iff(connected.slice((..(h - 1), ..))),
                ),
            );
            solver.add_expr(
                (is_black.eq(1).slice((.., 1..)) & is_black.eq(1).slice((.., ..(w - 1)))).imp(
                    connected
                        .slice((.., 1..))
                        .iff(connected.slice((.., ..(w - 1)))),
                ),
            );
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

pub type Problem = (i32, Vec<Vec<IsowatariClue>>, Option<Vec<Vec<bool>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple3::new(
        PrefixAndSuffix::new("", DecInt, "/"),
        ContextBasedGrid::new(Map::new(
            MultiDigit::new(3, 3),
            |x: IsowatariClue| {
                Some(match x {
                    IsowatariClue::None => 0,
                    IsowatariClue::White => 1,
                    IsowatariClue::Black => 2,
                })
            },
            |n: i32| match n {
                0 => Some(IsowatariClue::None),
                1 => Some(IsowatariClue::White),
                2 => Some(IsowatariClue::Black),
                _ => None,
            },
        )),
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
    problem_to_url_with_context_pzprxs(
        combinator(),
        "isowatari",
        problem.clone(),
        &Context::sized(problem.1.len(), problem.1[0].len()),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["isowatari"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        let mut holes = vec![vec![false; 4]; 4];
        holes[0][3] = true;
        holes[1][3] = true;
        let mut clues = vec![vec![IsowatariClue::None; 4]; 4];
        clues[0][2] = IsowatariClue::Black;
        clues[3][0] = IsowatariClue::Black;
        clues[3][1] = IsowatariClue::White;

        (2, clues, Some(holes))
    }

    fn problem_for_tests2() -> Problem {
        let mut clues = vec![vec![IsowatariClue::None; 3]; 3];
        clues[0][0] = IsowatariClue::White;
        clues[2][1] = IsowatariClue::Black;

        (1, clues, None)
    }

    #[test]
    fn test_isowatari_problem1() {
        let (size, clues, holes) = problem_for_tests1();
        let ans = solve_isowatari(size, &clues, &holes);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [0, 1, 1, -1],
            [0, 0, 0, -1],
            [1, 0, 1, 1],
            [1, 0, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_isowatari_problem2() {
        let (size, clues, holes) = problem_for_tests2();
        let ans = solve_isowatari(size, &clues, &holes);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([[0, 1, 0], [0, 0, 0], [0, 1, 0]]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_isowatari_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://pzprxs.vercel.app/p?isowatari/4/4/2/2000l02400";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://pzprxs.vercel.app/p?isowatari/3/3/1/906";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
