use cspuz_rs::graph;
use cspuz_rs::serializer::{
    get_kudamono_url_info_detailed, parse_kudamono_dimension, Choice, Combinator, Context, Dict,
    KudamonoBorder, KudamonoGrid,
};
use cspuz_rs::solver::{Solver, TRUE};

pub fn solve_windows(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    let mut aux_graph = graph::infer_graph_from_2d_array((h, w));
    let mut aux_vertices = (!is_black).into_iter().collect::<Vec<_>>();

    graph::active_vertices_connected_2d(&mut solver, is_black);

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

    solver.add_expr(!(is_black.conv2d_and((2, 2))));
    solver.add_expr(is_black.conv2d_or((2, 2)));

    for room in &graph::borders_to_rooms(borders) {
        let cnt = is_black.select(room).count_true();
        let n = solver.int_var(room.len() as i32 / 2, room.len().div_ceil(2) as i32);
        solver.add_expr(cnt.eq(n));
    }

    for y in 0..h {
        for x in 0..w {
            if clues[y][x] == Some(1) {
                solver.add_expr(is_black.at((y, x)));
            } else if clues[y][x] == Some(0) {
                solver.add_expr(!is_black.at((y, x)));
            }
        }
    }
    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let parsed = get_kudamono_url_info_detailed(url)?;
    let (width, height) = parse_kudamono_dimension(parsed.get("W")?)?;

    let ctx = Context::sized_with_kudamono_mode(height, width, true);

    let clues;
    if let Some(p) = parsed.get("L") {
        let clues_combinator = KudamonoGrid::new(
            Choice::new(vec![
                Box::new(Dict::new(Some(0), "w")),
                Box::new(Dict::new(Some(1), "b")),
            ]),
            None,
        );
        clues = clues_combinator.deserialize(&ctx, p.as_bytes())?.1.pop()?;
    } else {
        clues = vec![vec![None; width]; height];
    }

    let border;
    if let Some(p) = parsed.get("SIE") {
        border = KudamonoBorder.deserialize(&ctx, p.as_bytes())?.1.pop()?;
    } else {
        border = graph::InnerGridEdges {
            horizontal: vec![vec![false; width]; height - 1],
            vertical: vec![vec![false; width - 1]; height],
        };
    }

    Some((border, clues))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0, 0, 0],
                [1, 1, 1, 1, 0],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 1, 0, 1],
                [0, 1, 0, 1],
                [0, 1, 0, 1],
                [0, 1, 0, 1],
                [1, 0, 1, 0],
                [1, 0, 1, 0],
            ]),
        };

        let clues = vec![
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
            vec![Some(1), None, None, Some(0), None],
            vec![None, None, None, None, None],
            vec![None, Some(1), None, None, None],
        ];

        (borders, clues)
    }

    #[test]
    fn test_windows_problem() {
        let problem = problem_for_tests();
        let ans = solve_windows(&problem.0, &problem.1);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 0, 1, 0, 0],
            [1, 1, 1, 0, 1],
            [1, 0, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 0, 1, 1],
            [0, 1, 0, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_windows_serializer() {
        let problem = problem_for_tests();
        let url = "https://pedros.works/kudamono/player?W=5x6&L=b2b4w14&SIE=2R2RRUU3UURR9UURRUU5UUR10R6LUU&G=windows";
        assert_eq!(deserialize_problem(url), Some(problem));
    }
}
