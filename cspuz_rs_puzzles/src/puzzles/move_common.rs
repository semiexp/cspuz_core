use cspuz_rs::graph;
use cspuz_rs::solver::{count_true, IntVarArray2D, Solver};

pub fn add_movement_constraints(
    solver: &mut Solver,
    movement: &graph::BoolGridEdges,
    start_state: &[Vec<Option<i32>>],
    end_state: &IntVarArray2D,
    h: usize, // height
    w: usize, // width
    straight: bool,
) {
    // Get max number
    let mut clue_max = 0;
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = start_state[y][x] {
                clue_max = clue_max.max(n);
            }
        }
    }
    // Create int array to track number movement
    let movement_as_num = &solver.int_var_2d((h, w), -2, clue_max);
    let dir = &solver.int_var_2d((h, w), 0, 4); // 1: up, 2: down, 3: left, 4: right

    // Link direction with movement grid
    solver.add_expr(
        movement
            .horizontal
            .iff(dir.slice((.., ..(w - 1))).eq(4) | dir.slice((.., 1..)).eq(3)),
    );
    solver.add_expr(
        movement
            .vertical
            .iff(dir.slice((..(h - 1), ..)).eq(2) | dir.slice((1.., ..)).eq(1)),
    );

    for y in 0..h {
        for x in 0..w {
            let has_in_edge = &solver.bool_var();
            let mut in_edge_cand = vec![];
            if y > 0 {
                in_edge_cand.push(dir.at((y - 1, x)).eq(2));
            }
            if y < h - 1 {
                in_edge_cand.push(dir.at((y + 1, x)).eq(1));
            }
            if x > 0 {
                in_edge_cand.push(dir.at((y, x - 1)).eq(4));
            }
            if x < w - 1 {
                in_edge_cand.push(dir.at((y, x + 1)).eq(3));
            }
            solver.add_expr(has_in_edge.ite(1, 0).eq(count_true(in_edge_cand)));
            let d = &dir.at((y, x));

            // Start state are source, nothing can move into
            if let Some(n) = start_state[y][x] {
                if n == -1 {
                    solver.add_expr(movement_as_num.at((y, x)).ge(0));
                } else if n > -1 {
                    solver.add_expr(movement_as_num.at((y, x)).eq(n));
                }
                solver.add_expr(!has_in_edge);
            } else {
                // Apart from clues that did not move, cells with no movement have no number
                solver.add_expr(
                    movement
                        .vertex_neighbors((y, x))
                        .count_true()
                        .eq(0)
                        .imp(end_state.at((y, x)).eq(-2)),
                );
            }

            // Handle number propagation and grid edge restriction
            if y == 0 {
                solver.add_expr(d.ne(1));
            } else {
                solver.add_expr(
                    d.eq(1).imp(
                        movement_as_num
                            .at((y - 1, x))
                            .eq(movement_as_num.at((y, x))),
                    ),
                );
            }
            if y == h - 1 {
                solver.add_expr(d.ne(2));
            } else {
                solver.add_expr(
                    d.eq(2).imp(
                        movement_as_num
                            .at((y + 1, x))
                            .eq(movement_as_num.at((y, x))),
                    ),
                );
            }
            if x == 0 {
                solver.add_expr(d.ne(3));
            } else {
                solver.add_expr(
                    d.eq(3).imp(
                        movement_as_num
                            .at((y, x - 1))
                            .eq(movement_as_num.at((y, x))),
                    ),
                );
            }
            if x == w - 1 {
                solver.add_expr(d.ne(4));
            } else {
                solver.add_expr(
                    d.eq(4).imp(
                        movement_as_num
                            .at((y, x + 1))
                            .eq(movement_as_num.at((y, x))),
                    ),
                );
            }

            // Add straight lines constraints if needed
            if straight {
                if y > 0 {
                    solver.add_expr(dir.at((y - 1, x)).eq(2).imp(d.eq(2) | d.eq(0)));
                }
                if y < h - 1 {
                    solver.add_expr(dir.at((y + 1, x)).eq(1).imp(d.eq(1) | d.eq(0)));
                }
                if x > 0 {
                    solver.add_expr(dir.at((y, x - 1)).eq(4).imp(d.eq(4) | d.eq(0)));
                }
                if x < w - 1 {
                    solver.add_expr(dir.at((y, x + 1)).eq(3).imp(d.eq(3) | d.eq(0)));
                }
            }

            // If a cell doesn't have a number in the end state, either the cell is on a movement line but not at its end, or its not on a line at all
            solver.add_expr((d.ne(0)).imp(end_state.at((y, x)).eq(-2)));
            solver.add_expr((end_state.at((y, x)).eq(movement_as_num.at((y, x)))).iff(d.eq(0)));
        }
    }
    solver.add_expr(end_state.ne(-1));
}
