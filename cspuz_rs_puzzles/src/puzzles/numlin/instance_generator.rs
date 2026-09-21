use cspuz_core::config::Config;
use cspuz_rs::graph;
use cspuz_rs::solver::{Solver, TRUE};

use rand::{Rng, SeedableRng};

use super::Problem;

pub(super) fn generate_instance_by_csp(height: usize, width: usize, seed: u64) -> Option<Problem> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut config = Config::default();
    config.glucose_random_seed = Some(rng.gen_range(0.0..1.0));
    let mut solver = Solver::with_config(config);

    let is_line = graph::GridEdges::new(&mut solver, (height - 1, width - 1));
    let is_endpoint = &solver.bool_var_2d((height, width));

    let max_num_endpoint = (height + width).min(height * width / 10).max(4) as i32;
    solver.add_expr(is_endpoint.count_true().le(max_num_endpoint * 2));

    for y in 0..height {
        for x in 0..width {
            solver.add_expr(
                is_line
                    .vertex_neighbors((y, x))
                    .count_true()
                    .eq(is_endpoint.at((y, x)).ite(1, 2)),
            );
        }
    }

    // add randomness
    for y in 0..height {
        for x in 0..width {
            if rng.gen_range(0.0..1.0) < 0.02 {
                solver.add_expr(is_endpoint.at((y, x)).iff(rng.gen_range(0..2) == 0));
            }
        }
    }
    for y in 0..height {
        for x in 0..(width - 1) {
            if rng.gen_range(0.0..1.0) < 0.01 {
                solver.add_expr(is_line.horizontal.at((y, x)).iff(rng.gen_range(0..2) == 0));
            }
        }
    }
    for y in 0..(height - 1) {
        for x in 0..width {
            if rng.gen_range(0.0..1.0) < 0.01 {
                solver.add_expr(is_line.vertical.at((y, x)).iff(rng.gen_range(0..2) == 0));
            }
        }
    }

    // forbid trivial detour
    for y in 0..(height - 1) {
        for x in 0..(width - 1) {
            solver.add_expr(is_line.cell_neighbors((y, x)).count_true().le(2));
        }
    }

    // forbid trivial paths (length == 1)
    for y in 0..height {
        for x in 0..(width - 1) {
            solver.add_expr(
                is_line
                    .horizontal
                    .at((y, x))
                    .imp(!(is_endpoint.at((y, x)) & is_endpoint.at((y, x + 1)))),
            );
        }
    }
    for y in 0..(height - 1) {
        for x in 0..width {
            solver.add_expr(
                is_line
                    .vertical
                    .at((y, x))
                    .imp(!(is_endpoint.at((y, x)) & is_endpoint.at((y + 1, x)))),
            );
        }
    }

    // acyclic
    let mut graph = graph::Graph::new((height - 1) * (width - 1) + 1);
    let outside = (height - 1) * (width - 1);
    let mut indicator = vec![];

    for y in 0..height {
        for x in 0..(width - 1) {
            let v1 = if y == 0 {
                outside
            } else {
                (y - 1) * (width - 1) + x
            };
            let v2 = if y == height - 1 {
                outside
            } else {
                y * (width - 1) + x
            };
            graph.add_edge(v1, v2);
            indicator.push(!is_line.horizontal.at((y, x)));
        }
    }
    for y in 0..(height - 1) {
        for x in 0..width {
            let v1 = if x == 0 {
                outside
            } else {
                y * (width - 1) + (x - 1)
            };
            let v2 = if x == width - 1 {
                outside
            } else {
                y * (width - 1) + x
            };
            graph.add_edge(v1, v2);
            indicator.push(!is_line.vertical.at((y, x)));
        }
    }
    let is_active = vec![TRUE; graph.n_vertices()];
    graph::active_vertices_connected_via_active_edges(&mut solver, &is_active, &indicator, &graph);

    solver.solve().map(|model| {
        let line = model.get(&is_line);
        let endpoint = model.get(is_endpoint);

        let mut problem = vec![vec![None; width]; height];
        let mut next_clue = 0;
        for y in 0..height {
            for x in 0..width {
                if endpoint[y][x] && problem[y][x].is_none() {
                    // traverse
                    let mut py = y;
                    let mut px = x;
                    let mut last_y = !0;
                    let mut last_x = !0;

                    loop {
                        let mut next_y = None;
                        let mut next_x = None;

                        if py > 0 && line.vertical[py - 1][px] && last_y != py - 1 {
                            next_y = Some(py - 1);
                            next_x = Some(px);
                        }
                        if py + 1 < height && line.vertical[py][px] && last_y != py + 1 {
                            next_y = Some(py + 1);
                            next_x = Some(px);
                        }
                        if px > 0 && line.horizontal[py][px - 1] && last_x != px - 1 {
                            next_y = Some(py);
                            next_x = Some(px - 1);
                        }
                        if px + 1 < width && line.horizontal[py][px] && last_x != px + 1 {
                            next_y = Some(py);
                            next_x = Some(px + 1);
                        }
                        if let (Some(ny), Some(nx)) = (next_y, next_x) {
                            last_y = py;
                            last_x = px;
                            py = ny;
                            px = nx;
                        } else {
                            break;
                        }
                    }

                    problem[y][x] = Some(next_clue);
                    problem[py][px] = Some(next_clue);
                    next_clue += 1;
                }
            }
        }
        problem
    })
}
