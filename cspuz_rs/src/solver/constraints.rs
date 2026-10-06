use super::traits::{BoolArrayLike, IntArrayLike};
use super::{BoolExpr, IntExpr};

use cspuz_core::csp::BoolExpr as CSPBoolExpr;
use cspuz_core::csp::IntExpr as CSPIntExpr;

pub fn any<T: BoolArrayLike>(values: T) -> BoolExpr {
    let terms = values.to_vec().into_iter().map(Box::new).collect();
    BoolExpr::from_raw(CSPBoolExpr::Or(terms))
}

pub fn all<T: BoolArrayLike>(values: T) -> BoolExpr {
    let terms = values.to_vec().into_iter().map(Box::new).collect();
    BoolExpr::from_raw(CSPBoolExpr::And(terms))
}

pub fn sum<T: IntArrayLike>(values: T) -> IntExpr {
    let terms = values
        .to_vec()
        .into_iter()
        .map(|x| (Box::new(x), 1))
        .collect();
    IntExpr::from_raw(CSPIntExpr::Linear(terms))
}

pub fn count_true<T: BoolArrayLike>(values: T) -> IntExpr {
    let terms = values
        .to_vec()
        .into_iter()
        .map(|x| {
            (
                Box::new(x.ite(CSPIntExpr::Const(1), CSPIntExpr::Const(0))),
                1,
            )
        })
        .collect();
    IntExpr::from_raw(CSPIntExpr::Linear(terms))
}

pub fn consecutive_prefix_true<T: BoolArrayLike>(values: T) -> IntExpr {
    let terms = values.to_vec();

    let mut ret = CSPIntExpr::Const(0);
    for t in terms.into_iter().rev() {
        ret = t.ite(ret + CSPIntExpr::Const(1), CSPIntExpr::Const(0));
    }

    IntExpr::from_raw(ret)
}

pub fn bool_constant(b: bool) -> BoolExpr {
    BoolExpr::from_raw(CSPBoolExpr::Const(b))
}

/// Counts consecutive true cells in each direction, starting next to `center`.
/// The center itself is excluded. A ray ends at its first false cell or the
/// boundary; empty rays have length zero. No solver variables are allocated.
/// Panics if the center is out of bounds.
pub fn orthogonal_prefix_lengths<T>(
    cells: T,
    center: (usize, usize),
) -> crate::items::FourDirections<IntExpr>
where
    T: super::traits::Operand<Shape = (usize, usize), Value = CSPBoolExpr>,
{
    use crate::items::{Arrow, FourDirections};
    let cells = cells.as_ndarray();
    FourDirections {
        up: cells.cell_ray(center, Arrow::Up).consecutive_prefix_true(),
        down: cells
            .cell_ray(center, Arrow::Down)
            .consecutive_prefix_true(),
        left: cells
            .cell_ray(center, Arrow::Left)
            .consecutive_prefix_true(),
        right: cells
            .cell_ray(center, Arrow::Right)
            .consecutive_prefix_true(),
    }
}

pub fn int_constant(n: i32) -> IntExpr {
    IntExpr::from_raw(CSPIntExpr::Const(n))
}

pub const TRUE: BoolExpr = BoolExpr::from_raw(CSPBoolExpr::Const(true));
pub const FALSE: BoolExpr = BoolExpr::from_raw(CSPBoolExpr::Const(false));

#[cfg(test)]
mod tests {
    use super::orthogonal_prefix_lengths;
    use crate::items::Arrow;
    use crate::solver::Solver;

    const DIRECTIONS: [(Arrow, i32, i32); 4] = [
        (Arrow::Up, -1, 0),
        (Arrow::Down, 1, 0),
        (Arrow::Left, 0, -1),
        (Arrow::Right, 0, 1),
    ];

    #[test]
    fn test_cell_lengths_stop_at_first_false_cell() {
        for (h, w) in [(1, 1), (1, 5), (5, 1), (3, 3)] {
            for mask in 0..1usize << (h * w) {
                let mut solver = Solver::new();
                let black = solver.bool_var_2d((h, w));
                for y in 0..h {
                    for x in 0..w {
                        solver.add_expr(black.at((y, x)).iff(mask & (1 << (y * w + x)) == 0));
                    }
                }
                let mut checks = vec![];
                for y in 0..h {
                    for x in 0..w {
                        let lengths = orthogonal_prefix_lengths(!&black, (y, x));
                        for (expr, (_, dy, dx)) in
                            [lengths.up, lengths.down, lengths.left, lengths.right]
                                .into_iter()
                                .zip(DIRECTIONS)
                        {
                            let (mut yy, mut xx) = (y as i32 + dy, x as i32 + dx);
                            let mut expected = 0;
                            while 0 <= yy && yy < h as i32 && 0 <= xx && xx < w as i32 {
                                if mask & (1 << (yy as usize * w + xx as usize)) == 0 {
                                    break;
                                }
                                expected += 1;
                                yy += dy;
                                xx += dx;
                            }
                            let value = solver.int_var(0, (h + w) as i32);
                            solver.add_expr(value.eq(expr));
                            checks.push((value, expected));
                        }
                    }
                }
                let model = solver.solve().unwrap();
                for (value, expected) in checks {
                    assert_eq!(model.get(&value), expected);
                }
            }
        }
    }
}
