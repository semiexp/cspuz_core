use crate::util;
use cspuz_rs::graph;
use cspuz_rs::items::NumberedArrow;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Grid, MaybeSkip, NumberedArrowCombinator,
    Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_yajikazu(clues: &[Vec<Option<NumberedArrow>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);
    solver.add_expr(!is_black.conv2d_and((1, 2)));
    solver.add_expr(!is_black.conv2d_and((2, 1)));
    graph::active_vertices_connected_2d(&mut solver, !is_black);

    for y in 0..h {
        for x in 0..w {
            if let Some((dir, n)) = clues[y][x] {
                if n < 0 {
                    continue;
                }
                if let Some(cells) = is_black.pointing_cells((y, x), dir) {
                    solver.add_expr((!is_black.at((y, x))).imp(cells.count_true().eq(n)));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = Vec<Vec<Option<NumberedArrow>>>;

fn combinator() -> impl Combinator<Problem> {
    MaybeSkip::new(
        "b/",
        Grid::new(Choice::new(vec![
            Box::new(Optionalize::new(NumberedArrowCombinator)),
            Box::new(Spaces::new(None, 'a')),
        ])),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "yajikazu", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["yajikazu", "yajikazu"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cspuz_rs::items::Arrow;

    fn problem_for_tests() -> Problem {
        let mut ret = vec![vec![None; 6]; 5];
        ret[1][1] = Some((Arrow::Right, 0));
        ret[1][2] = Some((Arrow::Down, 2));
        ret[1][5] = Some((Arrow::Left, 2));
        ret[3][0] = Some((Arrow::Right, 1));
        ret[3][4] = Some((Arrow::Up, 2));
        ret[4][4] = Some((Arrow::Left, 23));
        ret
    }

    #[test]
    fn test_yajikazu_problem() {
        let clues = problem_for_tests();
        let ans = solve_yajikazu(&clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 0, 0, 0, 1, 0],
            [0, 1, 0, 1, 0, 0],
            [0, 0, 1, 0, 1, 0],
            [1, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_yajikazu_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?yajikazu/6/5/g4022b32f41c12e817a";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
