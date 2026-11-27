use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, NumSpaces, Spaces,
};
use cspuz_rs::solver::{count_true, BoolVar, IntVar, Solver};

pub fn solve_ichimaga(clues: &[Vec<Option<i32>>]) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let line_ul = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    let line_dr = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_expr(
        is_line
            .as_sequence()
            .iff(line_ul.as_sequence() | line_dr.as_sequence()),
    );
    solver.add_expr(!(line_ul.as_sequence() & line_dr.as_sequence()));

    // connected
    {
        let (edges, g) = is_line.representation();
        graph::active_vertices_connected(&mut solver, &edges, &g.line_graph());
    }

    let n_turn_horizontal = &solver.int_var_2d((h, w - 1), 0, 1);
    let n_turn_vertical = &solver.int_var_2d((h - 1, w), 0, 1);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    solver.add_expr(is_line.vertex_neighbors((y, x)).count_true().eq(n));
                } else {
                    solver.add_expr(is_line.vertex_neighbors((y, x)).any());
                }
            } else {
                let mut adj: [Option<(BoolVar, BoolVar, IntVar)>; 4] = [None, None, None, None];

                if y > 0 {
                    adj[0] = Some((
                        line_dr.vertical.at((y - 1, x)),
                        line_ul.vertical.at((y - 1, x)),
                        n_turn_vertical.at((y - 1, x)),
                    ));
                }
                if y < h - 1 {
                    adj[1] = Some((
                        line_ul.vertical.at((y, x)),
                        line_dr.vertical.at((y, x)),
                        n_turn_vertical.at((y, x)),
                    ));
                }
                if x > 0 {
                    adj[2] = Some((
                        line_dr.horizontal.at((y, x - 1)),
                        line_ul.horizontal.at((y, x - 1)),
                        n_turn_horizontal.at((y, x - 1)),
                    ));
                }
                if x < w - 1 {
                    adj[3] = Some((
                        line_ul.horizontal.at((y, x)),
                        line_dr.horizontal.at((y, x)),
                        n_turn_horizontal.at((y, x)),
                    ));
                }

                let mut in_edges = vec![];
                let mut out_edges = vec![];
                for elm in &adj {
                    if let Some((in_edge, out_edge, _)) = elm {
                        in_edges.push(in_edge);
                        out_edges.push(out_edge);
                    }
                }
                solver.add_expr(count_true(&in_edges).le(1));
                solver.add_expr(count_true(&in_edges).eq(count_true(&out_edges)));

                for i in 0..4 {
                    for j in 0..4 {
                        if i != j {
                            if let (
                                Some((in_edge_i, _, n_turn_i)),
                                Some((_, out_edge_j, n_turn_j)),
                            ) = (&adj[i], &adj[j])
                            {
                                if i / 2 == j / 2 {
                                    solver.add_expr(
                                        (in_edge_i & out_edge_j).imp(n_turn_i.eq(n_turn_j)),
                                    );
                                } else {
                                    solver.add_expr(
                                        (in_edge_i & out_edge_j).imp(n_turn_i.eq(n_turn_j + 1)),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
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
    problem_to_url(combinator(), "ichimaga", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["ichimaga"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let problem = vec![
            vec![None, None, None, None, Some(3), None],
            vec![None, Some(4), None, None, None, None],
            vec![None, None, None, None, Some(1), Some(1)],
            vec![Some(3), None, None, None, Some(1), None],
            vec![None, None, None, Some(1), None, None],
        ];
        problem
    }

    #[test]
    fn test_ichimaga_problem() {
        let problem = problem_for_tests();
        let ans = solve_ichimaga(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 1, 1],
                [1, 1, 1, 1, 0],
                [0, 1, 1, 1, 0],
                [1, 1, 1, 1, 0],
                [1, 1, 1, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 0, 1, 1],
                [1, 1, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_ichimaga_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?ichimaga/6/5/jdel11dgbhb";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
