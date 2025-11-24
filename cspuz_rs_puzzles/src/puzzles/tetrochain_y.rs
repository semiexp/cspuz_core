use crate::util;
use cspuz_rs::items::NumberedArrow;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Grid, MaybeSkip, NumberedArrowCombinator,
    Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_tetrochain_y(clues: &[Vec<Option<NumberedArrow>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    super::tetrochain_common::add_tetrochain_constraints(&mut solver, is_black);

    for y in 0..h {
        for x in 0..w {
            if let Some((dir, n)) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                if let Some(cells) = is_black.pointing_cells((y, x), dir) {
                    solver.add_expr(cells.count_true().eq(n));
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
    problem_to_url(combinator(), "tetrochain", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["tetrochain"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cspuz_rs::items::Arrow;

    fn problem_for_tests() -> Problem {
        let mut ret = vec![vec![None; 7]; 5];
        ret[1][2] = Some((Arrow::Down, 2));
        ret[2][1] = Some((Arrow::Left, 1));
        ret[2][3] = Some((Arrow::Up, 2));
        ret[4][6] = Some((Arrow::Up, 4));
        ret
    }

    #[test]
    fn test_tetrochain_y_problem() {
        let clues = problem_for_tests();
        let ans = solve_tetrochain_y(&clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, -1, 0, 1, 1, 0, 1],
            [1, -1, 0, 1, 1, 0, 1],
            [1, 0, 1, 0, 0, 0, 1],
            [0, 1, 1, 0, 1, 0, 1],
            [0, 1, 0, 1, 1, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_tetrochain_y_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?tetrochain/7/5/i22e31a12p14";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
