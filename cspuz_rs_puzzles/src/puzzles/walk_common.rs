use crate::util;
use cspuz_rs::graph;
use cspuz_rs::solver::Solver;

pub fn walk_not_passing_colored_cell(
    colored_cell: &[Vec<bool>],
    num: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(colored_cell);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, &is_line);
    solver.add_expr(is_passed.any());

    let mut num_lines = None;
    for y in 0..h {
        for x in 0..w {
            if colored_cell[y][x] {
                solver.add_expr(!is_passed.at((y, x)));
            }
            if let Some(n) = num[y][x] {
                solver.add_expr(is_passed.at((y, x)));
                if n > 0 {
                    if num_lines.is_some() && num_lines.unwrap() != n {
                        return None;
                    }
                    num_lines = Some(n);
                }
            }
        }
    }

    if let Some(n) = num_lines {
        solver.add_expr(is_passed.count_true().eq(n));
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

fn merge_grids(
    ans1: &Vec<Vec<Option<bool>>>,
    ans2: &Vec<Vec<Option<bool>>>,
) -> Vec<Vec<Option<bool>>> {
    let (h, w) = util::infer_shape(ans1);
    assert_eq!((h, w), util::infer_shape(ans2));

    let mut merged = vec![vec![None; w]; h];
    for y in 0..h {
        for x in 0..w {
            merged[y][x] = match (ans1[y][x], ans2[y][x]) {
                (Some(b1), Some(b2)) => {
                    if b1 != b2 {
                        None
                    } else {
                        Some(b1)
                    }
                }
                _ => None,
            };
        }
    }

    merged
}

pub fn merge_walk_answers(
    ans1: Option<graph::BoolGridEdgesIrrefutableFacts>,
    ans2: Option<graph::BoolGridEdgesIrrefutableFacts>,
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    match (ans1, ans2) {
        (Some(f1), Some(f2)) => {
            let res = graph::BoolGridEdgesIrrefutableFacts {
                horizontal: merge_grids(&f1.horizontal, &f2.horizontal),
                vertical: merge_grids(&f1.vertical, &f2.vertical),
            };
            Some(res)
        }
        (Some(f), None) | (None, Some(f)) => Some(f),
        (None, None) => None,
    }
}
