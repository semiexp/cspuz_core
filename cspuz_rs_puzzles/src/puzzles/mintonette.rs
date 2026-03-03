use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_pzprxs, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize,
    Spaces,
};
use cspuz_rs::solver::{any, count_true, Solver, TRUE};

pub fn solve_mintonette(
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let mut clue_pos = vec![];
    let is_turn = &solver.bool_var_2d((h, w));
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    for y in 0..h {
        for x in 0..w {
            solver.add_expr(is_turn.at((y, x)).iff(
                (is_line.vertical.at_offset((y, x), (-1, 0), false)
                    | is_line.vertical.at_offset((y, x), (0, 0), false))
                    & (is_line.horizontal.at_offset((y, x), (0, -1), false)
                        | is_line.horizontal.at_offset((y, x), (0, 0), false)),
            ));
            if let Some(n) = clues[y][x] {
                clue_pos.push((y, x, n));
                solver.add_expr(is_line.vertex_neighbors((y, x)).count_true().eq(1));
            } else {
                solver.add_expr(is_line.vertex_neighbors((y, x)).count_true().eq(2));
            }
        }
    }

    if clue_pos.len() % 2 == 1 {
        return None;
    }

    let mut blocks = vec![];
    for &(y, x, _) in &clue_pos {
        let block = solver.bool_var_2d((h, w));
        graph::active_vertices_connected_2d(&mut solver, &block);
        solver.add_expr(block.at((y, x)));
        blocks.push(block);
    }

    for i in 0..blocks.len() {
        for j in (i + 1)..blocks.len() {
            let (yi, xi, ni) = clue_pos[i];
            let (yj, xj, nj) = clue_pos[j];

            if (ni != nj) & (ni > -1) & (nj > -1) {
                solver.add_expr(!(blocks[i].at((yj, xj))));
                solver.add_expr(!(blocks[j].at((yi, xi))));
            }
            if ni > -1 {
                solver.add_expr((blocks[i].clone() & is_turn).count_true().eq(ni));
            }
            if nj > -1 {
                solver.add_expr((blocks[j].clone() & is_turn).count_true().eq(nj));
            }
        }
    }
    for y in 0..h {
        for x in 0..w {
            let mut indicators = vec![];
            for i in 0..blocks.len() {
                indicators.push(blocks[i].at((y, x)));
            }
            solver.add_expr(count_true(indicators).eq(2));
        }
    }

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 {
                let diff = blocks
                    .iter()
                    .map(|block| block.at((y, x)) ^ block.at((y + 1, x)))
                    .collect::<Vec<_>>();
                solver.add_expr(any(diff).imp(!is_line.vertical.at((y, x))));
            }
            if x < w - 1 {
                let diff = blocks
                    .iter()
                    .map(|block| block.at((y, x)) ^ block.at((y, x + 1)))
                    .collect::<Vec<_>>();
                solver.add_expr(any(diff).imp(!is_line.horizontal.at((y, x))));
            }
        }
    }

    let mut aux_graph = graph::Graph::new((h - 1) * (w - 1) + 1 + (h - 1) * w + h * (w - 1));
    let mut indicator = vec![TRUE; (h - 1) * (w - 1) + 1];

    for y in 0..h {
        for x in 0..w - 1 {
            let v1 = if y == 0 {
                (h - 1) * (w - 1)
            } else {
                (y - 1) * (w - 1) + x
            };
            let v2 = if y == h - 1 {
                (h - 1) * (w - 1)
            } else {
                y * (w - 1) + x
            };

            let e = indicator.len();
            aux_graph.add_edge(e, v1);
            aux_graph.add_edge(e, v2);

            indicator.push(!is_line.horizontal.at((y, x)));
        }
    }

    for y in 0..h - 1 {
        for x in 0..w {
            let v1 = if x == 0 {
                (h - 1) * (w - 1)
            } else {
                y * (w - 1) + x - 1
            };
            let v2 = if x == w - 1 {
                (h - 1) * (w - 1)
            } else {
                y * (w - 1) + x
            };

            let e = indicator.len();
            aux_graph.add_edge(e, v1);
            aux_graph.add_edge(e, v2);

            indicator.push(!is_line.vertical.at((y, x)));
        }
    }

    graph::active_vertices_connected(&mut solver, &indicator, &aux_graph);

    solver.irrefutable_facts().map(|f| f.get(is_line))
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
    problem_to_url_pzprxs(combinator(), "mintonette", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["mintonette"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let height = 6;
        let width = 6;
        let mut ret = vec![vec![None; width]; height];
        ret[0][3] = Some(1);
        ret[0][4] = Some(2);
        ret[1][0] = Some(1);
        ret[1][1] = Some(7);
        ret[1][2] = Some(0);
        ret[1][3] = Some(2);
        ret[4][2] = Some(-1);
        ret[4][3] = Some(-1);
        ret[4][4] = Some(-1);
        ret[4][5] = Some(-1);
        ret[5][1] = Some(-1);
        ret[5][2] = Some(-1);

        ret
    }

    #[test]
    fn test_mintonette_problem() {
        let problem = problem_for_tests();
        let ans = solve_mintonette(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 0, 1],
                [0, 0, 0, 1, 1],
                [1, 0, 0, 1, 1],
                [1, 0, 0, 0, 1],
                [1, 0, 0, 0, 0],
                [1, 0, 1, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 0, 0, 1],
                [0, 1, 1, 0, 0, 0],
                [1, 0, 1, 1, 0, 1],
                [0, 1, 1, 1, 1, 0],
                [1, 0, 0, 0, 0, 1],
            ]),
        };

        assert_eq!(ans, expected);
    }

    #[test]
    fn test_mintonette_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?mintonette/6/6/i12g1702v....g..i";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
