use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_pzprxs, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize,
    Spaces,
};
use cspuz_rs::solver::{Solver, FALSE, TRUE};

pub fn solve_city_space(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    for y in 0..h {
        for x in 0..w {
            let lr = is_black.at_offset((y, x), (0, -1), FALSE)
                | is_black.at_offset((y, x), (0, 1), FALSE);
            let ud = is_black.at_offset((y, x), (-1, 0), FALSE)
                | is_black.at_offset((y, x), (1, 0), FALSE);
            solver.add_expr(is_black.at((y, x)).imp(lr ^ ud));
        }
    }
    graph::active_vertices_connected_2d(&mut solver, !is_black);

    solver.add_expr(is_black.conv2d_or((2, 2)));

    let mut aux_graph = graph::Graph::new(h * w + 1);
    let mut aux_vertices = is_black.expr().into_iter().collect::<Vec<_>>();
    aux_vertices.push(TRUE);

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 {
                aux_graph.add_edge(y * w + x, (y + 1) * w + x);
            }
            if x < w - 1 {
                aux_graph.add_edge(y * w + x, y * w + (x + 1));
            }
            if y < h - 1 && x < w - 1 {
                aux_graph.add_edge(y * w + x, (y + 1) * w + (x + 1));
            }
            if y < h - 1 && x > 0 {
                aux_graph.add_edge(y * w + x, (y + 1) * w + (x - 1));
            }
            if y == 0 || y == h - 1 || x == 0 || x == w - 1 {
                aux_graph.add_edge(y * w + x, h * w);
            }
        }
    }
    graph::active_vertices_connected(&mut solver, &aux_vertices, &aux_graph);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                if n < 0 {
                    continue;
                }
                solver.add_expr(
                    ((!is_black.slice_fixed_y((y, ..x)).reverse()).consecutive_prefix_true()
                        + (!is_black.slice_fixed_y((y, x + 1..))).consecutive_prefix_true()
                        + (!is_black.slice_fixed_x((..y, x)).reverse()).consecutive_prefix_true()
                        + (!is_black.slice_fixed_x((y + 1.., x))).consecutive_prefix_true())
                    .eq(n - 1),
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
    problem_to_url_pzprxs(combinator(), "cityspace", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["cityspace"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        vec![
            vec![None, Some(3), None, Some(2), None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, Some(3), None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
        ]
    }

    #[test]
    fn test_archipelago_problem() {
        let problem = problem_for_tests();

        let ans = solve_city_space(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 0, 1, 0, 1, 1],
            [1, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 0],
            [0, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_archipelago_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?cityspace/6/5/g3g2p3u";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
