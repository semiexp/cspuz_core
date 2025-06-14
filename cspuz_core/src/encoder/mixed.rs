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

    encode_linear_ge_mixed_from_info(&info, sum.constant)
}

#[allow(unused)]
/// Encode the equation "sum(info) + constant == 0" using encode_linear_ge_mixed_from_info twice.
pub(super) fn encode_linear_eq_mixed_from_info(
    mut info: Vec<LinearInfo>,
    constant: CheckedInt,
) -> ClauseSet {
    let mut ret = encode_linear_ge_mixed_from_info(&info, constant);
    for x in &mut info {
        match x {
            LinearInfo::Direct(x) => x.coef *= CheckedInt::new(-1),
            LinearInfo::Order(x) => x.coef *= CheckedInt::new(-1),
        }
    }
    ret.append(encode_linear_ge_mixed_from_info(&info, -constant));
    ret
}

pub(super) fn encode_linear_ge_mixed_from_info(
    info: &[LinearInfo],
    constant: CheckedInt,
) -> ClauseSet {
    fn encode_sub(
        info: &[LinearInfo],
        clause: &mut Vec<Lit>,
        idx: usize,
        upper_bound: CheckedInt,
        min_relax_on_erasure: Option<CheckedInt>,
        clauses_buf: &mut ClauseSet,
    ) {
        if upper_bound < 0 {
            if let Some(min_relax_on_erasure) = min_relax_on_erasure {
                if upper_bound + min_relax_on_erasure < 0 {
                    return;
                }
            }
            clauses_buf.push(clause);
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
                    encode_sub(info, clause, idx + 1, next_ub, None, clauses_buf);
                    clause.pop();
                }

                encode_sub(
                    info,
                    clause,
                    idx + 1,
                    upper_bound,
                    min_relax_on_erasure,
                    clauses_buf,
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
    encode_sub(info, &mut vec![], 0, upper_bound, None, &mut clauses_buf);

    clauses_buf
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::tests::{linear_sum, EncoderTester};
    use crate::arithmetic::CmpOp;
    use crate::domain::Domain;
    use crate::norm_csp::{Constraint, LinearLit};

    #[test]
    fn test_encode_linear_ge_mixed() {
        for mask in 0..8 {
            let mut tester = EncoderTester::new();

            let x = tester.add_int_var(Domain::range(0, 5), (mask & 4) != 0);
            let y = tester.add_int_var(Domain::range(2, 6), (mask & 2) != 0);
            let z = tester.add_int_var(Domain::range(-1, 4), (mask & 1) != 0);

            let lits = [LinearLit::new(
                linear_sum(&[(x, 3), (y, -4), (z, 2)], -1),
                CmpOp::Ge,
            )];
            {
                let clause_set = encode_linear_ge_mixed(&tester.env(), &lits[0].sum);
                tester.add_clause_set(clause_set);
            }

            tester.add_constraint(Constraint {
                bool_lit: vec![],
                linear_lit: lits.to_vec(),
            });

            tester.run_check();
        }
    }
}
