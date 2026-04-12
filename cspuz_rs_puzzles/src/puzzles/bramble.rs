use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Choice, Combinator, Context, Dict, HexInt,
    Optionalize, RoomsWithValues, Size, Spaces,
};
use cspuz_rs::solver::{Solver, TRUE};

pub fn solve_bramble(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Option<i32>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let h = borders.vertical.len();
    assert!(h > 0);
    let w = borders.vertical[0].len() + 1;

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let is_single = solver.bool_var_2d((h, w));
    for y in 0..h {
        for x in 0..w {
            solver.add_expr(
                is_single
                    .at((y, x))
                    .imp(!(is_black.four_neighbors((y, x)).any())),
            );
            solver.add_expr(
                (is_black.at((y, x)) & !is_single.at((y, x)))
                    .imp(is_black.four_neighbors((y, x)).count_true().eq(1)),
            );
        }
    }

    for y in 0..h {
        for x in 0..(w - 1) {
            if borders.vertical[y][x] {
                solver.add_expr(!(is_black.at((y, x)) & is_black.at((y, x + 1))));
            }
        }
    }
    for y in 0..(h - 1) {
        for x in 0..w {
            if borders.horizontal[y][x] {
                solver.add_expr(!(is_black.at((y, x)) & is_black.at((y + 1, x))));
            }
        }
    }

    let rooms = graph::borders_to_rooms(borders);
    assert_eq!(rooms.len(), clues.len());
    for i in 0..rooms.len() {
        if let Some(n) = clues[i] {
            solver.add_expr(is_black.select(&rooms[i]).count_true().eq(n));
        }
    }

    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            solver.add_expr(!(is_single.at((y, x)) & is_single.at((y + 1, x + 1))));
            solver.add_expr(!(is_single.at((y + 1, x)) & is_single.at((y, x + 1))));
        }
    }

    {
        // black cells are connected by 8-connectivity
        let mut g = graph::Graph::new(h * w);
        for y in 0..h {
            for x in 0..w {
                let v = y * w + x;
                if y > 0 {
                    g.add_edge(v, (y - 1) * w + x);
                }
                if x > 0 {
                    g.add_edge(v, y * w + (x - 1));
                }
                if y > 0 && x > 0 {
                    g.add_edge(v, (y - 1) * w + (x - 1));
                }
                if y > 0 && x < w - 1 {
                    g.add_edge(v, (y - 1) * w + (x + 1));
                }
            }
        }
        graph::active_vertices_connected(&mut solver, is_black.flatten(), &g);
    }

    {
        // white cells are connected to the outside
        let mut aux_graph = graph::infer_graph_from_2d_array((h, w));
        let mut aux_vertices = (!is_black).into_iter().collect::<Vec<_>>();

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
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Option<i32>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(RoomsWithValues::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ])))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context_pzprxs(
        combinator(),
        "bramble",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["bramble"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        // Example problem in Puzzle Square
        // https://puzsq.logicpuzzle.app/puzzle/125768
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0, 1, 1],
                [1, 1, 1, 0, 0],
                [1, 1, 1, 1, 0],
                [0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 0, 1, 0],
                [0, 0, 1, 1],
                [0, 0, 1, 1],
                [0, 1, 0, 1],
                [0, 1, 0, 1],
            ]),
        };
        let clues = vec![Some(3), Some(2), None, Some(2), Some(2), Some(2), None];
        (borders, clues)
    }

    #[test]
    fn test_bramble_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_bramble(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 0, 1, 1],
            [0, 0, 1, 0, 0],
            [1, 1, 0, 0, 1],
            [0, 0, 1, 1, 0],
            [1, 1, 0, 0, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_bramble_serializer() {
        let (borders, clues) = problem_for_tests();
        let problem = (borders, clues);
        let url = "https://pzprxs.vercel.app/p?bramble/5/5/4cql3su032g222g";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
