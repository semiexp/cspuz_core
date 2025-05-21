use crate::util;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, NumSpaces, Spaces,
};
use cspuz_rs::solver::{BoolVar, Solver};

pub fn solve_akari(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let has_light = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(has_light);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!has_light.at((y, x)));
                if n >= 0 {
                    solver.add_expr(has_light.four_neighbors((y, x)).count_true().eq(n));
                }
            }
        }
    }

    let mut horizontal_group: Vec<Vec<Option<BoolVar>>> = vec![vec![None; w]; h];
    for y in 0..h {
        let mut start: Option<usize> = None;
        for x in 0..=w {
            if x < w && clues[y][x].is_none() {
                if start.is_none() {
                    start = Some(x);
                }
            } else {
                if let Some(s) = start {
                    let v = solver.bool_var();
                    solver.add_expr(
                        has_light
                            .slice_fixed_y((y, s..x))
                            .count_true()
                            .eq(v.ite(1, 0)),
                    );
                    for x2 in s..x {
                        horizontal_group[y][x2] = Some(v.clone());
                    }
                    start = None;
                }
            }
        }
    }

    let mut vertical_group: Vec<Vec<Option<BoolVar>>> = vec![vec![None; w]; h];
    for x in 0..w {
        let mut start: Option<usize> = None;
        for y in 0..=h {
            if y < h && clues[y][x].is_none() {
                if start.is_none() {
                    start = Some(y);
                }
            } else {
                if let Some(s) = start {
                    let v = solver.bool_var();
                    solver.add_expr(
                        has_light
                            .slice_fixed_x((s..y, x))
                            .count_true()
                            .eq(v.ite(1, 0)),
                    );
                    for y2 in s..y {
                        vertical_group[y2][x] = Some(v.clone());
                    }
                    start = None;
                }
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if clues[y][x].is_none() {
                solver.add_expr(
                    horizontal_group[y][x].as_ref().unwrap()
                        | vertical_group[y][x].as_ref().unwrap(),
                );
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(has_light))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(NumSpaces::new(4, 2)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "akari", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["akari"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Vec<Vec<Option<i32>>> {
        // https://twitter.com/semiexp/status/1225770511080144896
        vec![
            vec![None, None, Some(2), None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None, Some(2), None],
            vec![None, None, None, None, None, None, None, Some(-1), None, None],
            vec![Some(-1), None, None, None, Some(3), None, None, None, None, None],
            vec![None, None, None, None, None, Some(-1), None, None, None, Some(-1)],
            vec![Some(2), None, None, None, Some(2), None, None, None, None, None],
            vec![None, None, None, None, None, Some(3), None, None, None, Some(-1)],
            vec![None, None, Some(-1), None, None, None, None, None, None, None],
            vec![None, Some(2), None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, Some(-1), None, None],
        ]
    }

    #[test]
    fn test_akari_problem() {
        let problem = problem_for_tests();
        let ans = solve_akari(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn test_akari_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?akari/10/10/hcscl.h.idn.i.cgcndg.h.ncs.h";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
