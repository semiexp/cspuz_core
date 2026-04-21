use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, NumSpaces, Size, Spaces,
};
use cspuz_rs::solver::{count_true, Solver, TRUE};

pub const GOKIGEN_SLASH: i32 = 0;
pub const GOKIGEN_BACKSLASH: i32 = 1;

pub fn solve_gokigen(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<i32>>>> {
    let h = clues.len() - 1;
    let w = clues[0].len() - 1;

    let mut solver = Solver::new();
    let ans = &solver.int_var_2d((h, w), 0, 1);
    solver.add_answer_key_int(ans);

    for y in 0..=h {
        for x in 0..=w {
            if let Some(n) = clues[y][x] {
                let mut adj = vec![];

                if y > 0 && x > 0 {
                    adj.push(ans.at((y - 1, x - 1)).eq(GOKIGEN_BACKSLASH));
                }
                if y > 0 && x < w {
                    adj.push(ans.at((y - 1, x)).eq(GOKIGEN_SLASH));
                }
                if y < h && x > 0 {
                    adj.push(ans.at((y, x - 1)).eq(GOKIGEN_SLASH));
                }
                if y < h && x < w {
                    adj.push(ans.at((y, x)).eq(GOKIGEN_BACKSLASH));
                }
                solver.add_expr(count_true(adj).eq(n));
            }
        }
    }

    /*
    Each cell contains 4 segments:
    0 /\ 1
    2 \/ 3
     */
    let mut g = graph::Graph::new(h * w * 4 + 1);
    let mut is_active = vec![];
    for y in 0..(h * 2) {
        for x in 0..(w * 2) {
            if y == 0 || y == h * 2 - 1 || x == 0 || x == w * 2 - 1 {
                g.add_edge(h * w * 4, y * w * 2 + x);
            }
            if y % 2 == x % 2 {
                is_active.push(ans.at((y / 2, x / 2)).eq(GOKIGEN_SLASH));
            } else {
                is_active.push(ans.at((y / 2, x / 2)).eq(GOKIGEN_BACKSLASH));
            }

            if y % 2 == x % 2 {
                if y < h * 2 - 1 {
                    if x > 0 {
                        g.add_edge(y * w * 2 + x, (y + 1) * w * 2 + x - 1);
                    }
                    g.add_edge(y * w * 2 + x, (y + 1) * w * 2 + x);
                }
                if x < w * 2 - 1 {
                    g.add_edge(y * w * 2 + x, y * w * 2 + x + 1);
                }
            } else {
                if y < h * 2 - 1 {
                    g.add_edge(y * w * 2 + x, (y + 1) * w * 2 + x);
                }
                if x < w * 2 - 1 {
                    g.add_edge(y * w * 2 + x, y * w * 2 + x + 1);
                }
                if y < h * 2 - 1 && x < w * 2 - 1 {
                    g.add_edge(y * w * 2 + x, (y + 1) * w * 2 + x + 1);
                }
            }
        }
    }
    is_active.push(TRUE);
    graph::active_vertices_connected(&mut solver, &is_active, &g);

    solver.irrefutable_facts().map(|f| f.get(ans))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Size::with_offset(
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(NumSpaces::new(4, 2)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
        1,
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(problem);
    problem_to_url_with_context(
        combinator(),
        "gokigen",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["gokigen"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        vec![
            vec![None, None, None, None, None],
            vec![None, Some(3), Some(2), None, Some(2)],
            vec![Some(1), None, Some(1), None, Some(1)],
            vec![None, Some(1), None, None, None],
        ]
    }

    #[test]
    fn test_gokigen_problem() {
        let problem = problem_for_tests();
        let ans = solve_gokigen(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([[1, 0, 1, 1], [0, 0, 1, 0], [0, 0, 0, 0]]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_gokigen_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?gokigen/4/3/l372666bg";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
