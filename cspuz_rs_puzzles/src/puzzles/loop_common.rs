use cspuz_rs::graph;
use cspuz_rs::solver::{BoolVarArray2D, Solver, FALSE};

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
    let cell_sides = &solver.bool_var_2d((height - 1, width - 1));
    for y in 0..height {
        for x in 0..width {
            if y < height - 1 {
                let a = if x == 0 {
                    FALSE
                } else {
                    cell_sides.at((y, x - 1)).expr()
                };
                let b = if x == width - 1 {
                    FALSE
                } else {
                    cell_sides.at((y, x)).expr()
                };
                solver.add_expr(is_line.vertical.at((y, x)) ^ a.iff(b));
            }
            if x < width - 1 {
                let a = if y == 0 {
                    FALSE
                } else {
                    cell_sides.at((y - 1, x)).expr()
                };
                let b = if y == height - 1 {
                    FALSE
                } else {
                    cell_sides.at((y, x)).expr()
                };
                solver.add_expr(is_line.horizontal.at((y, x)) ^ a.iff(b));
            }
        }
    }
    for y in 1..height {
        for x in 1..width {
            solver.add_expr(is_black.at((y, x)).imp(!cell_sides.at((y - 1, x - 1))))
        }
    }
}
