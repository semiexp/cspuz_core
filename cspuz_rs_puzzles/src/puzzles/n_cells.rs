use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, Spaces,
};
use cspuz_rs::solver::{count_true, int_constant, Solver};

pub fn solve_ncells(
    clues: &[Vec<Option<i32>>],
    n: i32,
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);

    let mut cell_id = vec![vec![None; w]; h];
    let mut id_last = 0usize;
    for y in 0..h {
        for x in 0..w {
            if clues[y][x] != Some(-2) {
                cell_id[y][x] = Some(id_last);
                id_last += 1;
            }
        }
    }

    let mut edges = vec![];
    let mut edge_vars = vec![];
    for y in 0..h {
        for x in 0..w {
            if y < h - 1 {
                if let (Some(a), Some(b)) = (cell_id[y][x], cell_id[y + 1][x]) {
                    edges.push((a, b));
                    edge_vars.push(is_border.horizontal.at((y, x)));
                } else {
                    solver.add_expr(is_border.horizontal.at((y, x)));
                }
            }
            if x < w - 1 {
                if let (Some(a), Some(b)) = (cell_id[y][x], cell_id[y][x + 1]) {
                    edges.push((a, b));
                    edge_vars.push(is_border.vertical.at((y, x)));
                } else {
                    solver.add_expr(is_border.vertical.at((y, x)));
                }
            }
        }
    }
    solver.add_graph_division(&vec![Some(int_constant(n)); id_last], &edges, &edge_vars);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    let mut nb = vec![];
                    let mut outer = 0;
                    if y > 0 {
                        nb.push(is_border.horizontal.at((y - 1, x)));
                    } else {
                        outer += 1;
                    }
                    if y < h - 1 {
                        nb.push(is_border.horizontal.at((y, x)));
                    } else {
                        outer += 1;
                    }
                    if x > 0 {
                        nb.push(is_border.vertical.at((y, x - 1)));
                    } else {
                        outer += 1;
                    }
                    if x < w - 1 {
                        nb.push(is_border.vertical.at((y, x)));
                    } else {
                        outer += 1;
                    }
                    solver.add_expr(count_true(nb).eq(n - outer));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(&is_border))
}

pub fn solve_fourcells(
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    solve_ncells(clues, 4)
}

pub fn solve_fivecells(
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    solve_ncells(clues, 5)
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Spaces::new(None, 'a')),
        Box::new(Dict::new(Some(-1), ".")),
        Box::new(Dict::new(Some(0), "0")),
        Box::new(Dict::new(Some(1), "1")),
        Box::new(Dict::new(Some(2), "2")),
        Box::new(Dict::new(Some(3), "3")),
        Box::new(Dict::new(Some(4), "4")),
        Box::new(Dict::new(Some(-2), "7")),
    ]))
}

pub fn serialize_fivecells_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "fivecells", problem.clone())
}

pub fn deserialize_fivecells_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["fivecells"], url)
}

pub fn serialize_fourcells_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "fourcells", problem.clone())
}

pub fn deserialize_fourcells_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["fourcells"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests_fivecells() -> Problem {
        vec![
            vec![Some(-2), Some(2), None, None, None, None],
            vec![None, None, Some(2), Some(1), None, None],
            vec![Some(3), Some(1), None, None, None, Some(1)],
            vec![None, None, Some(3), None, None, None],
            vec![None, None, Some(3), None, None, None],
            vec![None, None, None, None, None, None],
        ]
    }

    fn problem_for_tests_fourcells() -> Problem {
        vec![
            vec![Some(-2), None, Some(1), None, None],
            vec![None, None, None, Some(2), None],
            vec![None, None, Some(2), None, None],
            vec![None, Some(3), None, None, None],
            vec![None, None, Some(3), None, None],
        ]
    }

    #[test]
    fn test_fivecells_problem() {
        let problem = problem_for_tests_fivecells();
        let ans = solve_fivecells(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        #[rustfmt::skip]
        let expected = graph::BoolInnerGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 0, 0, 0],
                [1, 1, 1, 0, 1, 0],
                [1, 0, 1, 1, 1, 0],
                [0, 0, 1, 1, 0, 1],
                [0, 1, 1, 0, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 1, 0, 1],
                [0, 0, 1, 0, 1],
                [0, 0, 1, 1, 0],
                [1, 1, 0, 0, 1],
                [1, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_fourcells_problem() {
        let problem = problem_for_tests_fourcells();
        let ans = solve_fourcells(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        #[rustfmt::skip]
        let expected = graph::BoolInnerGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 0, 1, 0],
                [0, 1, 1, 0, 1],
                [0, 1, 0, 1, 0],
                [1, 0, 1, 1, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 1],
                [0, 1, 1, 0],
                [1, 0, 1, 1],
                [1, 1, 0, 1],
                [0, 0, 1, 0],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_fourcells_serializer() {
        let problem = problem_for_tests_fourcells();
        let url = "https://puzz.link/p?fourcells/5/5/7a1e2c2c3e3b";
        util::tests::serializer_test(
            problem,
            url,
            serialize_fourcells_problem,
            deserialize_fourcells_problem,
        );
    }

    #[test]
    fn test_fivecells_serializer() {
        let problem = problem_for_tests_fivecells();
        let url = "https://puzz.link/p?fivecells/6/6/72f21b31c1b3e3i";
        util::tests::serializer_test(
            problem,
            url,
            serialize_fivecells_problem,
            deserialize_fivecells_problem,
        );
    }
}
