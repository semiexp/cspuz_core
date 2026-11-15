use cspuz_rs::graph::OrthogonalSegments;
use cspuz_rs::solver::{BoolVarArray2D, Solver};

/// Adds ordinary Akari lighting rules: no lights on blocked cells, no lights
/// seeing each other, and every unblocked cell illuminated.
/// Numbered clues and region counts are left to the caller.
pub fn add_akari_lighting_constraints(
    solver: &mut Solver,
    has_light: &BoolVarArray2D,
    segments: &OrthogonalSegments,
) {
    let (h, w) = has_light.shape();
    assert_eq!(segments.horizontal_id.len(), h);
    assert_eq!(segments.vertical_id.len(), h);
    assert!(segments.horizontal_id.iter().all(|row| row.len() == w));
    assert!(segments.vertical_id.iter().all(|row| row.len() == w));
    let horizontal = solver.bool_var_1d(segments.horizontal.len());
    let vertical = solver.bool_var_1d(segments.vertical.len());
    for (runs, lit) in [
        (&segments.horizontal, &horizontal),
        (&segments.vertical, &vertical),
    ] {
        for (id, cells) in runs.iter().enumerate() {
            solver.add_expr(
                has_light
                    .select(cells)
                    .count_true()
                    .eq(lit.at(id).ite(1, 0)),
            );
        }
    }
    for y in 0..h {
        for x in 0..w {
            if let Some(horizontal_id) = segments.horizontal_id[y][x] {
                let vertical_id = segments.vertical_id[y][x].unwrap();
                solver.add_expr(horizontal.at(horizontal_id) | vertical.at(vertical_id));
            } else {
                solver.add_expr(!has_light.at((y, x)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cspuz_rs::graph::orthogonal_segments;

    #[test]
    fn test_lighting_matches_direct_sightlines() {
        let (h, w) = (2, 3);
        for blocks in 0..64 {
            let blocked: Vec<Vec<_>> = (0..h)
                .map(|y| (0..w).map(|x| blocks & (1 << (y * w + x)) != 0).collect())
                .collect();
            let segments = orthogonal_segments(&blocked);
            for lights in 0..64 {
                let has_light = |y, x| lights & (1 << (y * w + x)) != 0;
                let mut expected = blocks & lights == 0;
                for y in 0..h {
                    for x in 0..w {
                        if blocked[y][x] {
                            continue;
                        }
                        let mut illuminated = has_light(y, x);
                        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let (mut yy, mut xx) = (y as i32 + dy, x as i32 + dx);
                            while 0 <= yy && yy < h as i32 && 0 <= xx && xx < w as i32 {
                                let (y2, x2) = (yy as usize, xx as usize);
                                if blocked[y2][x2] {
                                    break;
                                }
                                if has_light(y2, x2) {
                                    illuminated = true;
                                    expected &= !has_light(y, x);
                                }
                                yy += dy;
                                xx += dx;
                            }
                        }
                        expected &= illuminated;
                    }
                }
                let mut solver = Solver::new();
                let vars = solver.bool_var_2d((h, w));
                for y in 0..h {
                    for x in 0..w {
                        solver.add_expr(vars.at((y, x)).iff(has_light(y, x)));
                    }
                }
                add_akari_lighting_constraints(&mut solver, &vars, &segments);
                assert_eq!(
                    solver.solve().is_some(),
                    expected,
                    "blocks={blocks}, lights={lights}"
                );
            }
        }
    }
}
