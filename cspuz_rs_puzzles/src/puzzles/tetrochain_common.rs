use crate::polyomino::{adjacent_outside_cells, bbox, enumerate_variants, named_tetrominoes};
use cspuz_rs::graph;
use cspuz_rs::solver::{all, any, BoolVarArray2D, Solver};

pub fn add_tetrochain_constraints(solver: &mut Solver, is_black: &BoolVarArray2D) {
    let (h, w) = is_black.shape();

    let mut variants = vec![];
    for (_, piece) in named_tetrominoes() {
        variants.push(
            enumerate_variants(&piece)
                .into_iter()
                .map(|v| {
                    let adj = adjacent_outside_cells(&v);
                    (v, adj)
                })
                .collect::<Vec<_>>(),
        );
    }
    assert_eq!(variants.len(), 5);

    let tetromino_kind = &solver.int_var_2d((h, w), -1, 4);
    solver.add_expr(tetromino_kind.eq(-1) ^ is_black);

    for y in 0..h {
        for x in 0..w {
            for k in 0..5 {
                eprintln!("y={}, x={}, k={}", y, x, k);
                let mut cands = vec![];
                for (piece, outside_cells) in &variants[k] {
                    let (ph, pw) = bbox(piece);
                    for &(py, px) in piece {
                        if y < py || x < px {
                            continue;
                        }
                        let oy = y - py;
                        let ox = x - px;
                        if oy + ph > h || ox + pw > w {
                            continue;
                        }

                        eprint!("- ");
                        let mut conditions = vec![];
                        for &(py2, px2) in piece {
                            eprint!("({},{}) ", oy + py2, ox + px2);
                            conditions.push(is_black.at((oy + py2, ox + px2)).expr());
                        }
                        for &(dy, dx) in outside_cells {
                            let y2 = oy as isize + dy;
                            let x2 = ox as isize + dx;

                            if 0 <= y2 && y2 < h as isize && 0 <= x2 && x2 < w as isize {
                                eprint!("!({},{}) ", y2, x2);
                                conditions.push(!is_black.at((y2 as usize, x2 as usize)));
                            }
                        }
                        eprintln!();

                        cands.push(all(conditions));
                    }
                }
                solver.add_expr(tetromino_kind.at((y, x)).eq(k as i32).imp(any(cands)));
            }
        }
    }

    // black cells are 8-neighbor connected
    let mut aux_graph = graph::Graph::new(h * w);
    for y in 0..h {
        for x in 0..w {
            if y < h - 1 {
                aux_graph.add_edge(y * w + x, (y + 1) * w + x);
            }
            if x < w - 1 {
                aux_graph.add_edge(y * w + x, y * w + x + 1);
            }
            if y < h - 1 && x < w - 1 {
                aux_graph.add_edge(y * w + x, (y + 1) * w + x + 1);
            }
            if y < h - 1 && x > 0 {
                aux_graph.add_edge(y * w + x, (y + 1) * w + x - 1);
            }
        }
    }
    graph::active_vertices_connected(solver, is_black, &aux_graph);

    // same tetrominoes do not touch diagonally
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            solver.add_expr(
                (is_black.at((y, x))
                    & is_black.at((y + 1, x + 1))
                    & !is_black.at((y, x + 1))
                    & !is_black.at((y + 1, x)))
                .imp(
                    tetromino_kind
                        .at((y, x))
                        .ne(tetromino_kind.at((y + 1, x + 1))),
                ),
            );
            solver.add_expr(
                (is_black.at((y, x + 1))
                    & is_black.at((y + 1, x))
                    & !is_black.at((y, x))
                    & !is_black.at((y + 1, x + 1)))
                .imp(
                    tetromino_kind
                        .at((y, x + 1))
                        .ne(tetromino_kind.at((y + 1, x))),
                ),
            );
        }
    }
}
