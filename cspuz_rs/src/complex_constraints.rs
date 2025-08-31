use crate::solver::{
    all, any, int_constant,
    traits::{BoolArrayLike, IntArrayLike},
    BoolExpr, BoolExprArray1D, IntExprArray1D, IntVarArray1D, Solver,
};

/// Adds a constraint that, if `condition` is true (or not present),
/// - given values are all different,
/// - the sum of the values is equal to `sum`, and
/// - the values are between `value_low` and `value_high` (inclusive).
///
/// Returns true if there is at least one possible assignment that satisfies the constraints, otherwise false.
/// Note that this function returns true if `condition` is present, because the constraint is satisfied
/// if `condition` is false.
pub fn sum_all_different<T: IntArrayLike>(
    solver: &mut Solver,
    values: T,
    sum: i32,
    value_low: i32,
    value_high: i32,
    condition: Option<BoolExpr>,
) -> bool {
    let values = values.to_vec();
    let terms = &IntExprArray1D::from_raw(values);

    if let Some(condition) = &condition {
        for i in 0..terms.len() {
            for j in (i + 1)..terms.len() {
                solver.add_expr(condition.imp(terms.at(i).ne(terms.at(j))));
            }
        }
    } else {
        solver.all_different(terms);
    }

    let mut indicators = vec![];

    for i in value_low..=value_high {
        indicators.push(terms.eq(i).any());
    }

    let part = partitions(sum, terms.len() as i32, value_low, value_high);
    let mut cands = vec![];
    for p in &part {
        let mut expr = vec![];
        for i in 0..p.len() {
            expr.push(&indicators[i] ^ !p[i]);
        }
        cands.push(all(expr));
    }
    if let Some(condition) = &condition {
        solver.add_expr(condition.imp(any(cands)));
    } else {
        solver.add_expr(any(cands));
    }

    !part.is_empty()
}

fn partitions(sum: i32, n: i32, value_low: i32, value_high: i32) -> Vec<Vec<bool>> {
    fn partition_impl(
        sum: i32,
        i: i32,
        n: i32,
        value_low: i32,
        cur_value_low: i32,
        value_high: i32,
        res: &mut Vec<Vec<bool>>,
        current: &mut [bool],
    ) {
        if i == n {
            if sum == 0 {
                res.push(current.to_vec());
            }
            return;
        }
        let rem = n - i;
        let min_possible = cur_value_low * rem + (rem * (rem - 1)) / 2;
        let max_possible = value_high * rem - (rem * (rem - 1)) / 2;
        if !(min_possible <= sum && sum <= max_possible) {
            return;
        }
        if value_high - cur_value_low + 1 < rem {
            return;
        }

        for v in cur_value_low..=value_high {
            if sum - v >= 0 {
                current[(v - value_low) as usize] = true;
                partition_impl(
                    sum - v,
                    i + 1,
                    n,
                    value_low,
                    v + 1,
                    value_high,
                    res,
                    current,
                );
                current[(v - value_low) as usize] = false;
            }
        }
    }

    let mut ret = vec![];
    let mut current = vec![false; (value_high - value_low + 1) as usize];
    partition_impl(
        sum,
        0,
        n,
        value_low,
        value_low,
        value_high,
        &mut ret,
        &mut current,
    );

    ret
}

/// Adds a "Japanese" constraint and returns an array representing the index of blocks which each cell belongs to.
///
/// Suppose `is_present` consists of N bool values (corresponding to "cells"). Then, they consist of some (possibly zero) consecutive `true` cells.
/// The returned array consists of N integer values, where each value represents the index of the block which the corresponding cell belongs to.
/// If `is_present[i]` is false, the i-th element of the returned array is not guaranteed.
///
/// If i-th element of `star` is true, then the i-th block may be absent (skipped).
/// Otherwise, the i-th block must be present (at least one element of the returned array equals to i).
///
/// In `star`, `true` must not appear in consecutive positions.
pub fn japanese<T: BoolArrayLike>(
    solver: &mut Solver,
    is_present: T,
    star: &[bool],
) -> IntVarArray1D {
    for i in 1..star.len() {
        if star[i] && star[i - 1] {
            panic!("In japanese(), true must not appear in consecutive positions in star.");
        }
    }

    let is_present = &BoolExprArray1D::from_raw(is_present.to_vec());
    let n = is_present.len();

    let ret = solver.int_var_1d(n, -1, star.len() as i32 - 1);
    for i in 0..n {
        let starts_new_block = &(if i == 0 {
            is_present.at(i).expr()
        } else {
            is_present.at(i) & !is_present.at(i - 1)
        });
        let last = &(if i == 0 {
            int_constant(-1)
        } else {
            ret.at(i - 1).expr()
        });
        let cur = &ret.at(i);

        solver.add_expr((!starts_new_block).imp(cur.eq(last)));
        for j in 0..(star.len() as i32 + 1) {
            let mut cands = vec![cur.eq(j)];
            if j < star.len() as i32 && star[j as usize] {
                cands.push(cur.eq(j + 1));
            }
            if j > 0 && star[(j - 1) as usize] {
                cands.push(cur.eq(j - 1));
            }
            solver.add_expr(starts_new_block.imp(last.eq(j - 1).imp(any(cands))));
        }
    }
    if star[star.len() - 1] {
        solver.add_expr(
            ret.at(n - 1).eq(star.len() as i32 - 1) | ret.at(n - 1).eq(star.len() as i32 - 2),
        );
    } else {
        solver.add_expr(ret.at(n - 1).eq(star.len() as i32 - 1));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_all_different() {
        {
            let mut solver = Solver::new();
            let nums = &solver.int_var_1d(3, 1, 9);
            solver.add_answer_key_int(nums);
            assert!(sum_all_different(&mut solver, nums, 9, 1, 9, None));
            assert_eq!(solver.answer_iter().count(), 3 * 6);
        }
        {
            let mut solver = Solver::new();
            let nums = &solver.int_var_1d(4, 1, 9);
            solver.add_answer_key_int(nums);
            assert!(sum_all_different(&mut solver, nums, 16, 1, 9, None));
            // 1 + 2 + 4 + 9
            // 1 + 2 + 5 + 8
            // 1 + 2 + 6 + 7
            // 1 + 3 + 4 + 8
            // 1 + 3 + 5 + 7
            // 1 + 4 + 5 + 6
            // 2 + 3 + 4 + 7
            // 2 + 3 + 5 + 6
            assert_eq!(solver.answer_iter().count(), 8 * 24);
        }
        {
            let mut solver = Solver::new();
            let nums = &solver.int_var_1d(3, -1, 9);
            solver.add_answer_key_int(nums);
            assert!(sum_all_different(&mut solver, nums, 3, -1, 9, None));
            assert_eq!(solver.answer_iter().count(), 3 * 6);
        }
        {
            let mut solver = Solver::new();
            let nums = &solver.int_var_1d(3, 1, 9);
            solver.add_answer_key_int(nums);
            assert!(!sum_all_different(&mut solver, nums, 25, 1, 9, None));
            assert!(solver.solve().is_none());
        }
    }

    #[test]
    fn test_sum_all_different_conditional() {
        {
            let mut solver = Solver::new();
            let nums = &solver.int_var_1d(3, 1, 6);
            let b = solver.bool_var();
            solver.add_answer_key_int(nums);
            solver.add_answer_key_bool(&b);
            assert!(sum_all_different(
                &mut solver,
                nums,
                9,
                1,
                6,
                Some(b.expr())
            ));
            assert_eq!(solver.answer_iter().count(), 3 * 6 + 6 * 6 * 6);
        }
    }

    #[test]
    fn test_partitions() {
        let result = partitions(9, 3, 1, 7);
        let expected = vec![
            vec![true, true, false, false, false, true, false],
            vec![true, false, true, false, true, false, false],
            vec![false, true, true, true, false, false, false],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_japanese() {
        {
            let mut solver = Solver::new();
            let is_present = &solver.bool_var_1d(6);
            solver.add_expr(is_present.at(0).iff(true));
            solver.add_expr(is_present.at(1).iff(true));
            solver.add_expr(is_present.at(2).iff(false));
            solver.add_expr(is_present.at(3).iff(true));
            solver.add_expr(is_present.at(4).iff(false));
            solver.add_expr(is_present.at(5).iff(true));
            let group_id = japanese(&mut solver, is_present, &[false, false, false]);
            let model = solver.solve();
            assert!(model.is_some());
            let model = model.unwrap();
            assert_eq!(model.get(&group_id.at(0)), 0);
            assert_eq!(model.get(&group_id.at(1)), 0);
            assert_eq!(model.get(&group_id.at(3)), 1);
            assert_eq!(model.get(&group_id.at(5)), 2);
        }
    }

    fn binom(n: usize, k: usize) -> usize {
        if k > n {
            return 0;
        }
        let mut num = 1;
        let mut denom = 1;
        for i in 0..k {
            num *= n - i;
            denom *= i + 1;
        }
        num / denom
    }

    #[test]
    fn test_japanese_exhaustive() {
        let n = 9;
        let n_groups = 4;
        for mask in 0..(1 << n_groups) {
            let mut star = vec![];
            for i in 0..n_groups {
                star.push((mask & (1 << i)) != 0);
            }

            let mut consecutive_star = false;
            for i in 1..n_groups {
                if star[i] && star[i - 1] {
                    consecutive_star = true;
                    break;
                }
            }
            if consecutive_star {
                continue;
            }

            let mut solver = Solver::new();
            let is_present = &solver.bool_var_1d(n);
            let group_id = &japanese(&mut solver, is_present, &star);

            let mut n_patterns_actual = 0;
            while let Some(model) = solver.solve() {
                let p = model.get(is_present);
                let g = model.get(group_id);

                let mut used = vec![false; n_groups];
                for i in 0..n {
                    if p[i] {
                        assert!(0 <= g[i] && (g[i] as usize) < n_groups);
                        used[g[i] as usize] = true;
                    }
                    if i > 0 {
                        assert!(g[i] >= g[i - 1]);
                    }
                }
                for i in 0..n_groups {
                    if !star[i] {
                        assert!(used[i]);
                    }
                }

                let mut refutation = vec![];
                for i in 0..n {
                    refutation.push(is_present.at(i).iff(p[i]));
                    if p[i] {
                        refutation.push(group_id.at(i).eq(g[i]));
                    }
                }
                n_patterns_actual += 1;
                solver.add_expr(!all(refutation));
            }

            let n_star = star.iter().filter(|&&b| b).count();
            let n_fixed = n_groups - n_star;
            let mut n_patterns_expected = 0;
            for p in 0..(1 << n) {
                let mut g = 0;
                for i in 0..n {
                    if (p & (1 << i)) != 0 {
                        if i == 0 || (p & (1 << (i - 1))) == 0 {
                            g += 1;
                        }
                    }
                }
                if n_fixed <= g {
                    if n_star == 0 {
                        if g == n_fixed {
                            n_patterns_expected += 1;
                        }
                        continue;
                    }
                    n_patterns_expected += binom(g - n_fixed + n_star - 1, n_star - 1);
                }
            }

            assert_eq!(n_patterns_actual, n_patterns_expected, "{} {}", n, mask);
        }
    }
}
