use crate::graph;
use crate::solver::Solver;

pub fn solve_slitherlink(
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let h = clues.len();
    let w = clues[0].len();

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    graph::single_cycle_grid_edges(&mut solver, &is_line);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(is_line.cell_neighbors((y, x)).count_true().eq(n));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}
