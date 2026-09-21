use super::direct::LinearInfoForDirectEncoding;
use super::order::LinearInfoForOrderEncoding;
use super::{ClauseSet, EncoderEnv, ExtendedLit, LinearInfo};
use crate::arithmetic::CheckedInt;
use crate::norm_csp::LinearSum;
use crate::sat::Lit;

pub(super) fn encode_linear_ge_mixed(env: &EncoderEnv, sum: &LinearSum) -> ClauseSet {
    let mut info = vec![];
    for (&var, &coef) in sum.iter() {
        let encoding = env.map.int_map[var].as_ref().unwrap();

        if let Some(order_encoding) = &encoding.order_encoding {
            // Prefer order encoding
            info.push(LinearInfo::Order(LinearInfoForOrderEncoding::new(
                coef,
                order_encoding,
            )));
        } else if let Some(direct_encoding) = &encoding.direct_encoding {
            info.push(LinearInfo::Direct(LinearInfoForDirectEncoding::new(
                coef,
                direct_encoding,
            )));
        }
    }

    encode_linear_ge_mixed_from_info(&info, sum.constant, true)
}

#[allow(unused)]
/// Encode the equation "sum(info) + constant == 0" using encode_linear_ge_mixed_from_info twice.
pub(super) fn encode_linear_eq_mixed_from_info(
    mut info: Vec<LinearInfo>,
    constant: CheckedInt,
) -> ClauseSet {
    let mut ret = encode_linear_ge_mixed_from_info(&info, constant, true);
    for x in &mut info {
        match x {
            LinearInfo::Direct(x) => x.coef *= CheckedInt::new(-1),
            LinearInfo::Order(x) => x.coef *= CheckedInt::new(-1),
        }
    }
    ret.append(encode_linear_ge_mixed_from_info(&info, -constant, true));
    ret
}

pub(super) fn encode_linear_ge_mixed_from_info(
    info: &[LinearInfo],
    constant: CheckedInt,
    erase_subsumed_clauses: bool,
) -> ClauseSet {
    fn encode_sub(
        info: &[LinearInfo],
        clause: &mut Vec<Lit>,
        idx: usize,
        upper_bound: CheckedInt,
        min_relax_on_erasure: Option<CheckedInt>,
        clauses_buf: &mut ClauseSet,
        erase_subsumed_clauses: bool,
    ) {
        if upper_bound < 0 {
            let cannot_prune = !erase_subsumed_clauses
                || min_relax_on_erasure
                    .map(|min_relax| upper_bound + min_relax >= 0)
                    .unwrap_or(true);
            if cannot_prune {
                clauses_buf.push(clause);
            }
            return;
        }
        if idx == info.len() {
            return;
        }

        match &info[idx] {
            LinearInfo::Order(order_encoding) => {
                if idx + 1 == info.len() {
                    match order_encoding.at_least_val(-(upper_bound - order_encoding.domain_max()))
                    {
                        ExtendedLit::True => (),
                        ExtendedLit::False => panic!(),
                        ExtendedLit::Lit(lit) => {
                            clause.push(lit);
                            clauses_buf.push(clause);
                            clause.pop();
                        }
                    }
                    return;
                }
                let ub_for_this_term = order_encoding.domain_max();

                for i in 0..(order_encoding.domain_size() - 1) {
                    // assume (value) <= domain[i]
                    let value = order_encoding.domain(i);
                    let next_ub = upper_bound - ub_for_this_term + value;
                    // let next_min_relax = min_relax_on_erasure.unwrap_or(CheckedInt::max_value()).min(order_encoding.domain(i + 1) - value);
                    clause.push(order_encoding.at_least(i + 1));
                    encode_sub(
                        info,
                        clause,
                        idx + 1,
                        next_ub,
                        None,
                        clauses_buf,
                        erase_subsumed_clauses,
                    );
                    clause.pop();
                }

                encode_sub(
                    info,
                    clause,
                    idx + 1,
                    upper_bound,
                    min_relax_on_erasure,
                    clauses_buf,
                    erase_subsumed_clauses,
                );
            }
            LinearInfo::Direct(direct_encoding) => {
                let ub_for_this_term = direct_encoding.domain_max();

                for i in 0..(direct_encoding.domain_size() - 1) {
                    let value = direct_encoding.domain(i);
                    let next_ub = upper_bound - ub_for_this_term + value;
                    let next_min_relax = min_relax_on_erasure
                        .unwrap_or(CheckedInt::max_value())
                        .min(ub_for_this_term - value);
                    clause.push(!direct_encoding.equals(i));
                    encode_sub(
                        info,
                        clause,
                        idx + 1,
                        next_ub,
                        Some(next_min_relax),
                        clauses_buf,
                        erase_subsumed_clauses,
                    );
                    clause.pop();
                }

                encode_sub(
                    info,
                    clause,
                    idx + 1,
                    upper_bound,
                    min_relax_on_erasure,
                    clauses_buf,
                    erase_subsumed_clauses,
                );
            }
        }
    }

    let mut upper_bound = constant;
    for linear in info {
        upper_bound += match linear {
            LinearInfo::Order(order_encoding) => order_encoding.domain_max(),
            LinearInfo::Direct(direct_encoding) => direct_encoding.domain_max(),
        };
    }

    let mut clauses_buf = ClauseSet::new();
    encode_sub(
        info,
        &mut vec![],
        0,
        upper_bound,
        None,
        &mut clauses_buf,
        erase_subsumed_clauses,
    );

    clauses_buf
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::test_utils::assert_erase_subsumed_clauses;
    use super::super::tests::{linear_sum, EncoderTester};
    use crate::arithmetic::CmpOp;
    use crate::domain::Domain;
    use crate::norm_csp::{BoolLit, LinearLit};

    fn check_erase_subsumed_clauses(
        domains: &[Vec<i32>],
        coefs: &[i32],
        direct_encoding: &[bool],
        constant: i32,
    ) {
        assert_eq!(domains.len(), coefs.len());
        assert_eq!(domains.len(), direct_encoding.len());

        let mut tester = EncoderTester::new();
        let vars = domains
            .iter()
            .zip(direct_encoding.iter().copied())
            .map(|(domain, is_direct)| {
                tester.add_int_var(Domain::enumerative(domain.clone()), is_direct)
            })
            .collect::<Vec<_>>();
        let terms = vars
            .iter()
            .copied()
            .zip(coefs.iter().copied())
            .collect::<Vec<_>>();
        let sum = linear_sum(&terms, constant);

        let env = tester.env();
        let info = sum
            .iter()
            .map(|(&var, &coef)| {
                let encoding = env.map.int_map[var].as_ref().unwrap();
                if let Some(order_encoding) = &encoding.order_encoding {
                    LinearInfo::Order(LinearInfoForOrderEncoding::new(coef, order_encoding))
                } else {
                    LinearInfo::Direct(LinearInfoForDirectEncoding::new(
                        coef,
                        encoding.as_direct_encoding(),
                    ))
                }
            })
            .collect::<Vec<_>>();

        let erased = encode_linear_ge_mixed_from_info(&info, sum.constant, true);
        let all = encode_linear_ge_mixed_from_info(&info, sum.constant, false);
        let instance = format!(
            "domains={domains:?}, coefs={coefs:?}, direct_encoding={direct_encoding:?}, constant={constant}"
        );
        assert_erase_subsumed_clauses(erased, all, &instance);
    }

    #[test]
    fn test_encode_linear_ge_mixed() {
        for mask in 0..8 {
            let mut tester = EncoderTester::new();

            let x = tester.add_int_var(Domain::range(0, 5), (mask & 4) != 0);
            let y = tester.add_int_var(Domain::range(2, 6), (mask & 2) != 0);
            let z = tester.add_int_var(Domain::range(-1, 4), (mask & 1) != 0);

            let lit = LinearLit::new(linear_sum(&[(x, 3), (y, -4), (z, 2)], -1), CmpOp::Ge);
            {
                let clause_set = encode_linear_ge_mixed(&tester.env(), &lit.sum);
                tester.add_clause_set(clause_set);
            }

            tester.add_constraint_linear_lit(lit);

            tester.run_check();
        }
    }

    #[test]
    fn test_encode_linear_ge_mixed_binary() {
        for mask in 0..8 {
            let mut tester = EncoderTester::new();

            let x = tester.add_bool_var();
            let a = tester.add_int_var(Domain::range(0, 5), (mask & 4) != 0);
            let b = tester.add_int_var(Domain::range(1, 6), (mask & 2) != 0);
            let c = tester.add_int_var_binary(
                BoolLit::new(x, true),
                CheckedInt::new(0),
                CheckedInt::new(2),
                (mask & 1) != 0,
            );

            let lit = LinearLit::new(linear_sum(&[(a, 3), (b, -4), (c, 2)], -1), CmpOp::Ge);
            {
                let clause_set = encode_linear_ge_mixed(&tester.env(), &lit.sum);
                tester.add_clause_set(clause_set);
            }

            tester.add_constraint_linear_lit(lit);

            tester.run_check();
        }
    }

    #[test]
    fn test_encode_linear_ge_mixed_erase_subsumed_clauses() {
        let instances = [
            (
                vec![vec![0, 2], vec![0, 6], vec![0, 1, 2]],
                vec![1, 1, 1],
                vec![true, true, false],
                -5,
            ),
            (
                vec![vec![-2, 0], vec![-6, 0], vec![-2, -1, 0]],
                vec![-1, -1, -1],
                vec![true, true, false],
                -5,
            ),
            (
                vec![vec![0, 2], vec![-6, 0], vec![0, 1, 2]],
                vec![1, -1, 1],
                vec![true, true, false],
                -5,
            ),
            (
                vec![vec![0, 2], vec![0, 3], vec![0, 10], vec![0, 1, 2, 3]],
                vec![1, 1, 1, 1],
                vec![true, false, true, false],
                -10,
            ),
        ];

        for (domains, coefs, direct_encoding, constant) in instances {
            check_erase_subsumed_clauses(&domains, &coefs, &direct_encoding, constant);
        }
    }
}
