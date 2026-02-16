use cspuz_rs::graph;
use cspuz_rs::solver::Solver;

pub fn add_full_loop_constraints(
    solver: &mut Solver,
    is_line: &graph::BoolGridEdges,
    height: usize,
    width: usize,
) {
    let is_passed = &graph::single_cycle_grid_edges(solver, is_line);
    for y in 0..height {
        for x in 0..width {
            solver.add_expr(is_passed.at((y, x)));
        }
    }
}
