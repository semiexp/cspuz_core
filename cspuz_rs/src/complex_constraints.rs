use crate::solver::{all, any, Array0DImpl, Array1DImpl, CSPIntExpr, Operand, Solver, Value};

/// Adds a constraint that,
/// - given values are all different,
/// - the sum of the values is equal to `sum`, and
/// - the values are between `value_low` and `value_high` (inclusive).
pub fn sum_all_different<T>(
    solver: &mut Solver,
    values: T,
    sum: i32,
    value_low: i32,
    value_high: i32,
) where
    T: IntoIterator,
    T::Item: Operand<Output = Array0DImpl<CSPIntExpr>>,
{
    let terms: Vec<Value<Array0DImpl<CSPIntExpr>>> = values
        .into_iter()
        .map(|x| Value(x.as_expr_array()))
        .collect();
    let terms = &Value::<Array1DImpl<_>>::new(terms);
    solver.all_different(terms);

    let mut indicators = vec![];

    for i in value_low..=value_high {
        indicators.push(terms.eq(i).any());
    }

    let part = partitions(sum, terms.len() as i32, value_low, value_high);
    let mut cands = vec![];
    for p in part {
        let mut expr = vec![];
        for i in 0..p.len() {
            expr.push(&indicators[i] ^ !p[i]);
        }
        cands.push(all(expr));
    }
    solver.add_expr(any(cands));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_all_different() {
        {
            let mut solver = Solver::new();
            let nums = &solver.int_var_1d(3, 1, 9);
            solver.add_answer_key_int(nums);
            sum_all_different(&mut solver, nums, 9, 1, 9);
            assert_eq!(solver.answer_iter().count(), 3 * 6);
        }
        {
            let mut solver = Solver::new();
            let nums = &solver.int_var_1d(4, 1, 9);
            solver.add_answer_key_int(nums);
            sum_all_different(&mut solver, nums, 16, 1, 9);
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
            sum_all_different(&mut solver, nums, 3, -1, 9);
            assert_eq!(solver.answer_iter().count(), 3 * 6);
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
}
