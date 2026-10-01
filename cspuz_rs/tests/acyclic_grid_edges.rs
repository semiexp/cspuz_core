use cspuz_rs::graph::{active_edges_acyclic_grid_edges, BoolGridEdges};
use cspuz_rs::solver::Solver;

#[test]
fn acyclicity_matches_union_find() {
    // Includes empty grids, paths, disconnected trees, branches, and cycles.
    for shape in [(0, 0), (0, 3), (3, 0), (1, 1), (1, 2), (2, 2)] {
        let mut solver = Solver::new();
        let edges = BoolGridEdges::new(&mut solver, shape);
        let (_, graph) = edges.representation();
        for mask in 0..1usize << graph.n_edges() {
            let mut parent: Vec<_> = (0..graph.n_vertices()).collect();
            let mut acyclic = true;
            for i in 0..graph.n_edges() {
                if mask & (1 << i) == 0 {
                    continue;
                }
                let (mut a, mut b) = graph[i];
                while parent[a] != a {
                    a = parent[a];
                }
                while parent[b] != b {
                    b = parent[b];
                }
                if a == b {
                    acyclic = false;
                    break;
                }
                parent[a] = b;
            }

            let mut solver = Solver::new();
            let edges = BoolGridEdges::new(&mut solver, shape);
            let (vars, _) = edges.representation();
            for (i, var) in vars.iter().enumerate() {
                solver.add_expr(var.iff(mask & (1 << i) != 0));
            }
            active_edges_acyclic_grid_edges(&mut solver, &edges);
            assert_eq!(solver.solve().is_some(), acyclic, "{shape:?}, {mask}");
        }
    }
}
