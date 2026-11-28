use crate::penpa_editor::{decode_penpa_editor_url, Item, PenpaEditorPuzzle};
use crate::util;
use cspuz_rs::graph;
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_nikakoku(clues: &[Vec<Option<i32>>]) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();

    let is_line = &graph::BoolGridEdges::new(&mut solver, (h + 1, w + 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    // Cell centers are vertices (1..=h, 1..=w). The outermost edges are unused.
    solver.add_expr(!is_line.horizontal.slice_fixed_y((0, ..)));
    solver.add_expr(!is_line.horizontal.slice_fixed_y((h + 1, ..)));
    solver.add_expr(!is_line.vertical.slice_fixed_x((.., 0)));
    solver.add_expr(!is_line.vertical.slice_fixed_x((.., w + 1)));

    // The line must meet the boundary at exactly two endpoints.
    let mut boundary_edges = vec![];
    for x in 1..=w {
        boundary_edges.push(is_line.vertical.at((0, x)));
        boundary_edges.push(is_line.vertical.at((h, x)));
    }
    for y in 1..=h {
        boundary_edges.push(is_line.horizontal.at((y, 0)));
        boundary_edges.push(is_line.horizontal.at((y, w)));
    }
    solver.add_expr(count_true(boundary_edges).eq(2));

    // All line segments belong to the same path.
    {
        let (edges, g) = is_line.representation();
        graph::active_vertices_connected(&mut solver, &edges, &g.line_graph());
    }

    for y in 0..h {
        for x in 0..w {
            if clues[y][x].is_some() {
                solver.add_expr(!(is_line.vertex_neighbors((y + 1, x + 1)).any()));
            } else {
                let adj = is_line.vertex_neighbors((y + 1, x + 1)).count_true();
                solver.add_expr((adj.eq(0)) | (adj.eq(2)));
            }
        }
    }

    let color = &solver.int_var_2d((h + 1, w + 1), 0, 1);
    solver.add_expr(
        color.slice((1.., ..)).eq(&color.slice((..h, ..)))
            ^ is_line.horizontal.slice((1..(h + 1), ..)),
    );
    solver.add_expr(
        color.slice((.., 1..)).eq(&color.slice((.., ..w)))
            ^ is_line.vertical.slice((.., 1..(w + 1))),
    );
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(color.slice((y..(y + 2), x..(x + 2))).eq(n));
            }
        }
    }

    graph::active_vertices_connected_2d(&mut solver, color.eq(0));
    graph::active_vertices_connected_2d(&mut solver, color.eq(1));

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = Vec<Vec<Option<i32>>>;

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let decoded = decode_penpa_editor_url(url).ok()?;
    let decoded = match decoded {
        PenpaEditorPuzzle::Square(s) => s,
        _ => return None,
    };

    let mut ret = vec![vec![None; decoded.width()]; decoded.height()];
    for y in 0..decoded.height() {
        for x in 0..decoded.width() {
            for item in decoded.get_cell(y, x) {
                if let Item::Symbol(symbol) = item {
                    if symbol.name.starts_with("circle_") {
                        ret[y][x] = Some(match symbol.color_id {
                            1 => 0,
                            2 => 1,
                            _ => return None,
                        });
                    }
                }
            }
        }
    }

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(1), None, Some(0), None, Some(1), None, Some(1)],
            vec![Some(1), None, None, None, None, None, None],
            vec![None, None, None, Some(1), None, Some(0), None],
            vec![None, Some(0), None, None, None, None, None],
            vec![None, None, None, None, Some(0), None, Some(1)],
            vec![Some(1), None, Some(1), None, None, None, None],
        ]
    }

    #[test]
    fn test_nikakoku_problem() {
        let problem = problem_for_tests();
        let ans = solve_nikakoku(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 1, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 0, 1, 0],
                [0, 1, 1, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 0, 1, 0, 0, 0, 0],
                [0, 0, 1, 0, 1, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 1, 0, 1, 0],
                [0, 1, 0, 1, 0, 1, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 1, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_nikakoku_serializer() {
        let problem = problem_for_tests();
        let url = "https://opt-pan.github.io/penpa-edit/#m=edit&p=7ZRdb5swFIbv+RWTr30B5iMJd1nX7KbtPpoqihCKnIQ2qBB3BtbJUf57jw9MicGTpmnVejEhjl4en/gcO35dfWu4zOiIRtQfU5d68PhuQFkUwTvB1+2eeV4XWfyOTpt6JyQISj/NZvSeF1XmJF1W6hzUJFZTqj7GCfEIJQxej6RUfYkP6jpWS6puYYhQD9hVm+SDvGwlA7nAca0uWui5oG86DXIJcpPLTZGtrlvyOU7UnBJd5z3+WktSiu8Z6frQ3xtRrnMN1ryGxVS7/KkbqZqteGy6XC89UjX9rXa1bNvVytKuXsUrtztJj0fY9q/Q8CpOdO93Jzk+ydv4APEGo4dxGR8IC2AaBsXOGyQsAuoN6NiW67tWGtpoMLHR0LNVC3W1AR0xK7WuYmStNtbVehQ2Y4ZbwjDOYceo8jF+wOhiDDFeYc4lxgXGC4wBxghzRnrP//hfeaV2EhagxX8+4d/4Sp0EXE8qUayqRt7zDZxhvBTgmALbN+U6kwYqhHgq8r2Zlz/shcysQxpm2wdb/lrIbW/2Z14UBmgvOQO1f72BaglWO/vmUopng5S83hngzJbGTNm+Nhuoudkif+S9auVpzUeH/CD4wiFlcA/DDv+/VP/Jpar/BPetmfittYPnV0ir+QFb/A/U6vOOD6wOfGBqXXDoa6AWawPtuxvQ0OAABx4H9gub61n7Ttdd9c2uSw38rkudWz5JnRc=";
        assert_eq!(deserialize_problem(url).unwrap(), problem);
    }
}
