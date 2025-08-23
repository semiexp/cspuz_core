use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::{Solver, TRUE};

pub fn solve_cave(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    // white cells are connected
    graph::active_vertices_connected_2d(&mut solver, !is_black);

    let mut aux_graph = graph::infer_graph_from_2d_array((h, w));
    let mut aux_vertices = is_black.expr().into_iter().collect::<Vec<_>>();

    let outer = aux_graph.add_vertex();
    aux_vertices.push(TRUE);

    for y in 0..h {
        for x in 0..w {
            if y == 0 || y == h - 1 || x == 0 || x == w - 1 {
                aux_graph.add_edge(y * w + x, outer);
            }
        }
    }
    graph::active_vertices_connected(&mut solver, &aux_vertices, &aux_graph);

    let is_white = &!is_black;

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                if n < 0 {
                    continue;
                }
                let up = is_white.slice_fixed_x((..y, x)).reverse();
                let down = is_white.slice_fixed_x(((y + 1).., x));
                let left = is_white.slice_fixed_y((y, ..x)).reverse();
                let right = is_white.slice_fixed_y((y, (x + 1)..));
                solver.add_expr(
                    (up.consecutive_prefix_true()
                        + down.consecutive_prefix_true()
                        + left.consecutive_prefix_true()
                        + right.consecutive_prefix_true()
                        + 1)
                    .eq(n),
                );
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "cave", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["cave"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        // https://puzz.link/p?cave/6/6/k3h6j2l7g3g2h3n
        vec![
            vec![None, None, None, None, None, Some(3)],
            vec![None, None, Some(6), None, None, None],
            vec![None, Some(2), None, None, None, None],
            vec![None, None, Some(7), None, Some(3), None],
            vec![Some(2), None, None, Some(3), None, None],
            vec![None, None, None, None, None, None],
        ]
    }

    #[test]
    fn test_cave_problem() {
        let problem = problem_for_tests();
        let ans = solve_cave(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 1, 0, 0, 0],
            [1, 1, 0, 0, 1, 1],
            [1, 0, 0, 1, 1, 1],
            [1, 1, 0, 0, 0, 1],
            [0, 1, 0, 0, 1, 1],
            [0, 0, 0, 1, 1, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_cave_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?cave/6/6/k3h6j2l7g3g2h3n";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
