use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, Dict, MultiDigit, Optionalize, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{count_true, Solver, TRUE};

pub fn solve_tilecity(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = borders.base_shape();

    let rooms = graph::borders_to_rooms(borders);
    let mut room_id = vec![vec![0; w]; h];
    for (i, room) in rooms.iter().enumerate() {
        for &(y, x) in room {
            room_id[y][x] = i;
        }
    }

    let mut edges = vec![];
    for y in 0..h {
        for x in 0..w {
            if y + 1 < h && room_id[y][x] != room_id[y + 1][x] {
                let a = room_id[y][x];
                let b = room_id[y + 1][x];
                if a < b {
                    edges.push((a, b));
                } else {
                    edges.push((b, a));
                }
            }
            if x + 1 < w && room_id[y][x] != room_id[y][x + 1] {
                let a = room_id[y][x];
                let b = room_id[y][x + 1];
                if a < b {
                    edges.push((a, b));
                } else {
                    edges.push((b, a));
                }
            }
        }
    }
    edges.sort();
    edges.dedup();

    let mut g = graph::Graph::new(rooms.len());
    for (a, b) in edges {
        g.add_edge(a, b);
    }

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let is_black_room = &solver.bool_var_1d(rooms.len());
    for (i, room) in rooms.iter().enumerate() {
        for &(y, x) in room {
            solver.add_expr(is_black.at((y, x)).iff(is_black_room.at(i)));
        }
    }

    let room_size = rooms.iter().map(|r| r.len() as i32).collect::<Vec<_>>();

    solver.add_expr(!(is_black.conv2d_and((2, 2))));

    // all white cell should be connected to a room of size 1
    {
        let mut aux_graph = graph::Graph::new(rooms.len() + 1);
        for v in 0..rooms.len() {
            if room_size[v] == 1 {
                aux_graph.add_edge(v, rooms.len());
            }
        }
        for i in 0..g.n_edges() {
            let (a, b) = g[i];
            aux_graph.add_edge(a, b);
        }

        let mut aux_vertices = is_black_room.into_iter().map(|x| !x).collect::<Vec<_>>();
        aux_vertices.push(TRUE);
        graph::active_vertices_connected(&mut solver, aux_vertices, &aux_graph);
    }

    let max_size = room_size.iter().cloned().max().unwrap_or(0);
    let room_max_size = &solver.int_var_1d(rooms.len(), 0, max_size);

    for i in 0..g.n_edges() {
        let (a, b) = g[i];
        solver.add_expr(
            (!is_black_room.at(a) & !is_black_room.at(b))
                .imp(room_max_size.at(a).eq(room_max_size.at(b))),
        );
    }

    let mut room_indices_by_size = vec![vec![]; (max_size + 1) as usize];
    for (i, &size) in room_size.iter().enumerate() {
        room_indices_by_size[size as usize].push(i);
    }

    for r in 0..rooms.len() {
        if room_size[r] != 1 {
            continue;
        }

        let cc = &solver.bool_var_1d(rooms.len());
        solver.add_expr((!is_black_room.at(r)).imp(cc.at(r)));
        solver.add_expr(is_black_room.at(r).imp(!(cc.any())));
        solver.add_expr(is_black_room.imp(!cc));

        for i in 0..g.n_edges() {
            let (a, b) = g[i];
            solver.add_expr(
                (!is_black_room.at(a) & !is_black_room.at(b)).imp(cc.at(a).iff(cc.at(b))),
            );
        }

        for s in 1..=max_size {
            let items = room_indices_by_size[s as usize]
                .iter()
                .map(|&r| cc.at(r))
                .collect::<Vec<_>>();
            solver.add_expr(count_true(items).eq(room_max_size.at(r).ge(s).ite(1, 0)));
        }
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                if c == -1 {
                    continue;
                }
                let r = room_id[y][x];
                solver.add_expr(!is_black_room.at(r));
                solver.add_expr(room_max_size.at(r).eq(c));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(MultiDigit::new(10, 1))),
            Box::new(Spaces::new(None, 'a')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context_pzprxs(
        combinator(),
        "tilecity",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["tilecity"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 0, 1, 1, 1],
                [1, 0, 1, 1, 1, 1],
                [1, 1, 0, 0, 1, 0],
                [0, 1, 1, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [1, 0, 1, 1, 0],
                [1, 1, 1, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 1, 1, 1],
                [1, 0, 1, 1, 0],
            ]),
        };

        let clues = vec![
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, None, None, None, None, None],
            vec![None, Some(3), None, None, None, None],
            vec![None, None, None, None, None, None],
        ];

        (borders, clues)
    }

    #[test]
    fn test_tilecity_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_tilecity(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [1, 1, 1, 0, 1, 1],
            [1, 0, 1, 0, 0, 1],
            [0, 0, 0, 1, 1, 0],
            [1, 0, 0, 1, 0, 0],
            [1, 1, 1, 0, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_tilecity_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?tilecity/6/5/mttnmbru9qs3j";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
