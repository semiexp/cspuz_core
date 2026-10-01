use cspuz_rs::graph::{self, GridNeighborhood};
use cspuz_rs::solver::Solver;

#[test]
fn grid_edges_match_neighborhood() {
    for shape in [(0, 0), (0, 3), (3, 0), (1, 1), (1, 4), (4, 1), (3, 4)] {
        for kind in [
            GridNeighborhood::Orthogonal,
            GridNeighborhood::Diagonal,
            GridNeighborhood::Eight,
        ] {
            let g = graph::grid_graph(shape, kind);
            let (h, w) = shape;
            assert_eq!(g.n_vertices(), h * w);
            let mut expected = vec![];
            for a in 0..h * w {
                for b in a + 1..h * w {
                    let dy = (a / w).abs_diff(b / w);
                    let dx = (a % w).abs_diff(b % w);
                    let adjacent = match kind {
                        GridNeighborhood::Orthogonal => dy + dx == 1,
                        GridNeighborhood::Diagonal => dy == 1 && dx == 1,
                        GridNeighborhood::Eight => dy <= 1 && dx <= 1,
                    };
                    if adjacent {
                        expected.push((a, b));
                    }
                }
            }
            let mut actual: Vec<_> = (0..g.n_edges()).map(|i| g[i]).collect();
            actual.sort_unstable();
            assert_eq!(actual, expected);
        }
        let old = graph::infer_graph_from_2d_array(shape);
        let new = graph::grid_graph(shape, GridNeighborhood::Orthogonal);
        assert_eq!(
            (0..old.n_edges()).map(|i| old[i]).collect::<Vec<_>>(),
            (0..new.n_edges()).map(|i| new[i]).collect::<Vec<_>>()
        );
    }
}

fn accepts(shape: (usize, usize), active: &[(usize, usize)], kind: GridNeighborhood) -> bool {
    let mut solver = Solver::new();
    let cells = solver.bool_var_2d(shape);
    for y in 0..shape.0 {
        for x in 0..shape.1 {
            solver.add_expr(cells.at((y, x)).iff(active.contains(&(y, x))));
        }
    }
    // An expression input is supported as well as variables.
    graph::active_components_touch_boundary_2d(&mut solver, !!&cells, kind);
    solver.solve().is_some()
}

#[test]
fn each_component_must_reach_boundary() {
    use GridNeighborhood::*;
    for kind in [Orthogonal, Diagonal, Eight] {
        assert!(accepts((3, 3), &[], kind));
        assert!(accepts((3, 3), &[(0, 0), (2, 2)], kind));
        assert!(!accepts((3, 3), &[(0, 2), (1, 1)], Orthogonal));
        assert!(!accepts((3, 3), &[(1, 1)], kind));
        assert!(accepts((1, 3), &[(0, 1)], kind));
        assert!(accepts((3, 1), &[(1, 0)], kind));
        assert!(accepts((1, 1), &[(0, 0)], kind));
        assert!(accepts((0, 3), &[], kind));
        assert!(accepts((3, 0), &[], kind));
    }
    assert!(accepts((3, 3), &[(0, 0), (1, 1)], Eight));
    assert!(accepts((3, 3), &[(0, 0), (1, 1)], Diagonal));
    assert!(accepts((3, 3), &[(0, 1), (1, 1)], Orthogonal));
    assert!(!accepts((3, 3), &[(0, 1), (1, 1)], Diagonal));
}
