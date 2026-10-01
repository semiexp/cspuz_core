use cspuz_rs::graph;
use cspuz_rs::solver::{BoolVarArray2D, Solver};

pub fn add_full_loop_constraints(
    solver: &mut Solver,
    is_line: &graph::BoolGridEdges,
    height: usize,
    width: usize,
) {
    for y in 0..height + 1 {
        for x in 0..width + 1 {
            solver.add_expr(is_line.vertex_neighbors((y, x)).count_true().ne(0));
        }
    }
}

pub fn force_shaded_outside(
    solver: &mut Solver,
    is_black: &BoolVarArray2D,
    is_line: &graph::BoolGridEdges,
    height: usize,
    width: usize,
) {
    assert_eq!(is_line.base_shape(), (height - 1, width - 1));
    let cell_sides = graph::face_parities(solver, is_line);
    for y in 1..height {
        for x in 1..width {
            solver.add_expr(is_black.at((y, x)).imp(!cell_sides.at((y - 1, x - 1))))
        }
    }
}
