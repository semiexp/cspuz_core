use std::collections::VecDeque;

use super::direct::{
    encode_linear_eq_direct_from_info, DirectEncoding, LinearInfoForDirectEncoding,
};
use super::mixed::{encode_linear_eq_mixed_from_info, encode_linear_ge_mixed_from_info};
use super::order::{LinearInfoForOrderEncoding, OrderEncoding};
use super::{new_var, new_vars_as_lits, ClauseSet, EncoderEnv, LinearInfo, LinearLit};
use crate::arithmetic::{CheckedInt, CmpOp, Range};
use crate::norm_csp::{IntVar, IntVarRepresentation, LinearSum};
use crate::sat::{Lit, SAT};

/// Representation of a log-encoded variable.
///
/// The value of the variable equals lits[0] * 2^0 + lits[1] * 2^1 + ... + lits[n-1] * 2^(n-1) + offset.
/// `range` represents the range of the actual values after applying the offset.
pub(super) struct LogEncoding {
    pub lits: Vec<Lit>,
    pub range: Range,
    pub offset: CheckedInt,
}

pub fn encode_var_log(sat: &mut SAT, repr: &IntVarRepresentation) -> LogEncoding {
    match repr {
        IntVarRepresentation::Domain(domain) => {
            let low = domain.lower_bound_checked();
            let high = domain.upper_bound_checked();

            // Calculate offset to handle negative values
            let offset = if low < 0 { low } else { CheckedInt::new(0) };
            let shifted_low = low - offset;
            let shifted_high = high - offset;

            let n_bits = (32 - shifted_high.get().leading_zeros()) as usize;
            let lits = new_vars_as_lits!(sat, n_bits, "{}.log", var.id());

            // Add constraints for the lower bound
            for i in 0..n_bits {
                if ((shifted_low.get() >> i) & 1) != 0 {
                    let mut clause = vec![lits[i]];
                    for j in (i + 1)..n_bits {
                        clause.push(if (shifted_low.get() >> j) & 1 != 0 {
                            !lits[j]
                        } else {
                            lits[j]
                        });
                    }
                    sat.add_clause(&clause);
                }
            }

            // Add constraints for the upper bound
            for i in 0..n_bits {
                if (shifted_high.get() >> i) & 1 == 0 {
                    let mut clause = vec![!lits[i]];
                    for j in (i + 1)..n_bits {
                        clause.push(if (shifted_high.get() >> j) & 1 != 0 {
                            !lits[j]
                        } else {
                            lits[j]
                        });
                    }
                    sat.add_clause(&clause);
                }
            }

            // Add constraints to exclude gaps in the domain
            let domain = domain.enumerate();
            for i in 1..domain.len() {
                let gap_low = domain[i - 1].get() + 1;
                let gap_high = domain[i].get();
                for n in gap_low..gap_high {
                    let shifted_n = n - offset.get();
                    if shifted_n >= 0 {
                        let mut clause = vec![];
                        for j in 0..n_bits {
                            clause.push(if (shifted_n >> j) & 1 != 0 {
                                !lits[j]
                            } else {
                                lits[j]
                            });
                        }
                        sat.add_clause(&clause);
                    }
                }
            }

            LogEncoding {
                lits,
                range: Range::new(low, high),
                offset,
            }
        }
        IntVarRepresentation::Binary { .. } => {
            todo!();
        }
    }
}

pub fn decompose_linear_lit_log(env: &mut EncoderEnv, lit: &LinearLit) -> Vec<LinearLit> {
    assert!(lit.op == CmpOp::Ge || lit.op == CmpOp::Eq || lit.op == CmpOp::Ne);
    let op_for_aux_lits = if lit.op == CmpOp::Ge {
        CmpOp::Ge
    } else {
        CmpOp::Eq
    };

    let mut queue_positive = VecDeque::new();
    let mut queue_negative = VecDeque::new();
    for (&var, &coef) in &lit.sum.term {
        if coef > 0 {
            queue_positive.push_back((var, coef));
        } else if coef < 0 {
            queue_negative.push_back((var, coef));
        } else {
            panic!();
        }
    }

    let mut ret = vec![];

    const N_MAX_TERM: usize = 6;
    while queue_positive.len() + queue_negative.len() > N_MAX_TERM {
        let target_queue;
        let another_queue;
        let selecting_negative;
        if queue_positive.len() > queue_negative.len() {
            target_queue = &mut queue_positive;
            another_queue = &mut queue_negative;
            selecting_negative = false;
        } else {
            target_queue = &mut queue_negative;
            another_queue = &mut queue_positive;
            selecting_negative = true;
        }

        let n_pack = N_MAX_TERM.min(target_queue.len());

        let mut aux_sum = LinearSum::new();
        for _ in 0..n_pack {
            let (var, coef) = target_queue.pop_front().unwrap();
            aux_sum.add_coef(var, coef);
        }
        let mut aux_dom = env.norm_vars.get_domain_linear_sum(&aux_sum);

        let mut rem_sum = LinearSum::new();
        for &(var, coef) in target_queue.iter() {
            rem_sum.add_coef(var, coef);
        }
        for &(var, coef) in another_queue.iter() {
            rem_sum.add_coef(var, coef);
        }
        let rem_dom = env.norm_vars.get_domain_linear_sum(&rem_sum);
        aux_dom.refine_upper_bound(-(lit.sum.constant + rem_dom.lower_bound_checked()));
        aux_dom.refine_lower_bound(-(lit.sum.constant + rem_dom.upper_bound_checked()));
        if selecting_negative {
            aux_dom = aux_dom * CheckedInt::new(-1);
        }

        let aux_var = env
            .norm_vars
            .new_int_var(IntVarRepresentation::Domain(aux_dom));
        env.map
            .convert_int_var_log_encoding(env.norm_vars, env.sat, aux_var);

        aux_sum.add_coef(
            aux_var,
            CheckedInt::new(if selecting_negative { 1 } else { -1 }),
        );
        ret.push(LinearLit::new(aux_sum, op_for_aux_lits));

        target_queue.push_back((
            aux_var,
            CheckedInt::new(if selecting_negative { -1 } else { 1 }),
        ));
    }

    let mut sum = LinearSum::constant(lit.sum.constant);
    for &(var, coef) in &queue_positive {
        sum.add_coef(var, coef);
    }
    for &(var, coef) in &queue_negative {
        sum.add_coef(var, coef);
    }
    ret.push(LinearLit::new(sum, lit.op));

    ret
}

pub(super) fn encode_linear_log(env: &mut EncoderEnv, sum: &LinearSum, op: CmpOp) -> ClauseSet {
    // TODO: some clauses should be directly added to `env`
    if op == CmpOp::Eq {
        let mut values = vec![];
        let mut constant = sum.constant;

        for (&var, &coef) in sum.iter() {
            let encoding = env.map.int_map[var].as_ref().unwrap();
            let log_encoding = encoding.log_encoding.as_ref().unwrap();

            // Accumulate offset contributions
            constant += coef * log_encoding.offset;

            if coef > 0 {
                let mut coef = coef.get() as u32;
                for i in 0usize.. {
                    if (coef & 1) == 1 {
                        for j in 0..log_encoding.lits.len() {
                            values.push((i + j, CheckedInt::new(1), log_encoding.lits[j]));
                        }
                    }
                    coef >>= 1;
                    if coef == 0 {
                        break;
                    }
                }
            } else {
                let mut coef = (-coef).get() as u32;
                for i in 0usize.. {
                    if (coef & 1) == 1 {
                        for j in 0..log_encoding.lits.len() {
                            values.push((i + j, CheckedInt::new(-1), log_encoding.lits[j]));
                        }
                    }
                    coef >>= 1;
                    if coef == 0 {
                        break;
                    }
                }
            }
        }
        return log_encoding_adder2(env, values, constant);
    }

    let mut values_positive = vec![];
    let mut values_negative = vec![];
    let mut constant = sum.constant;

    for (&var, &coef) in sum.iter() {
        let encoding = env.map.int_map[var].as_ref().unwrap();
        let log_encoding = encoding.log_encoding.as_ref().unwrap();

        // Accumulate offset contributions
        constant += coef * log_encoding.offset;

        if coef > 0 {
            let mut coef = coef.get() as u32;
            for i in 0usize.. {
                if (coef & 1) == 1 {
                    values_positive.push((i, log_encoding.lits.clone()));
                }
                coef >>= 1;
                if coef == 0 {
                    break;
                }
            }
        } else {
            assert!(coef < 0);
            let mut coef = -coef.get() as u32;
            for i in 0usize.. {
                if (coef & 1) == 1 {
                    values_negative.push((i, log_encoding.lits.clone()));
                }
                coef >>= 1;
                if coef == 0 {
                    break;
                }
            }
        }
    }

    let (aux_clauses1, sum_positive) = log_encoding_adder(
        env,
        values_positive,
        vec![constant.max(CheckedInt::new(0))],
        vec![],
    );
    let (aux_clauses2, sum_negative) = log_encoding_adder(
        env,
        values_negative,
        vec![(-constant).max(CheckedInt::new(0))],
        vec![],
    );

    let mut clause_set = ClauseSet::new();
    clause_set.append(aux_clauses1);
    clause_set.append(aux_clauses2);

    match op {
        CmpOp::Eq => {
            for i in 0..(sum_positive.len().max(sum_negative.len())) {
                if i >= sum_positive.len() {
                    clause_set.push(&[!sum_negative[i]]);
                } else if i >= sum_negative.len() {
                    clause_set.push(&[!sum_positive[i]]);
                } else {
                    let p = sum_positive[i];
                    let n = sum_negative[i];

                    clause_set.push(&[p, !n]);
                    clause_set.push(&[!p, n]);
                }
            }
        }
        CmpOp::Ne => {
            let mut clause = vec![];
            for i in 0..(sum_positive.len().max(sum_negative.len())) {
                if i >= sum_positive.len() {
                    clause.push(sum_negative[i]);
                } else if i >= sum_negative.len() {
                    clause.push(sum_positive[i]);
                } else {
                    let aux = new_var!(env.sat).as_lit(false);
                    clause.push(aux);

                    let p = sum_positive[i];
                    let n = sum_negative[i];
                    // aux <=> (p ^ n)
                    // aux <=> ((p | n) & (!p | !n))
                    clause_set.push(&[!aux, p, n]);
                    clause_set.push(&[!aux, !p, !n]);
                    clause_set.push(&[aux, p, !n]);
                    clause_set.push(&[aux, !p, n]);
                }
            }
            clause_set.push(&clause);
        }
        CmpOp::Ge => {
            let mut sub: Option<Lit> = None;
            for i in 0..(sum_positive.len().min(sum_negative.len())) {
                let sub_next = new_var!(env.sat).as_lit(false);
                let p = sum_positive[i];
                let n = sum_negative[i];

                if let Some(sub) = sub {
                    // sub_next <=> (p & !n) | (p & n & sub) | (!p & !n & sub)
                    // sub_next <=> (!n | sub) & (p | !n) & (p | sub)
                    clause_set.push(&[!sub_next, !n, sub]);
                    clause_set.push(&[!sub_next, p, !n]);
                    clause_set.push(&[!sub_next, p, sub]);
                    clause_set.push(&[!p, n, sub_next]);
                    clause_set.push(&[!p, !n, !sub, sub_next]);
                    clause_set.push(&[p, n, !sub, sub_next]);
                } else {
                    // sub_next <=> p | !n
                    clause_set.push(&[!sub_next, p, !n]);
                    clause_set.push(&[!p, sub_next]);
                    clause_set.push(&[n, sub_next]);
                }
                sub = Some(sub_next);
            }

            if sum_positive.len() <= sum_negative.len() {
                if let Some(sub) = sub {
                    clause_set.push(&[sub]);
                }
                for i in sum_positive.len()..sum_negative.len() {
                    clause_set.push(&[!sum_negative[i]]);
                }
            } else {
                let mut clause = vec![];
                if let Some(sub) = sub {
                    clause.push(sub);
                }
                for i in sum_negative.len()..sum_positive.len() {
                    clause.push(sum_positive[i]);
                }
                clause_set.push(&clause);
            }
        }
        CmpOp::Gt | CmpOp::Le | CmpOp::Lt => panic!(),
    }

    clause_set
}

fn log_encoding_adder(
    env: &mut EncoderEnv,
    values: Vec<(usize, Vec<Lit>)>,
    constant: Vec<CheckedInt>,
    result: Vec<Lit>,
) -> (ClauseSet, Vec<Lit>) {
    let mut pos_vars: Vec<Vec<Lit>> = vec![vec![]; constant.len()];
    let mut pos_constant: Vec<CheckedInt> = constant;
    for (ofs, value) in values {
        while pos_vars.len() < ofs + value.len() {
            pos_vars.push(vec![]);
            pos_constant.push(CheckedInt::new(0));
        }
        for i in 0..value.len() {
            pos_vars[i + ofs].push(value[i]);
        }
    }
    assert_eq!(pos_vars.len(), pos_constant.len());
    {
        let mut i = 0;
        while i < pos_constant.len() {
            assert!(pos_constant[i] >= 0);
            if pos_constant[i] >= 2 {
                if i + 1 == pos_constant.len() {
                    pos_vars.push(vec![]);
                    pos_constant.push(pos_constant[i].div_floor(CheckedInt::new(2)));
                } else {
                    let v = pos_constant[i].div_floor(CheckedInt::new(2));
                    pos_constant[i + 1] += v;
                }
            }
            let v = pos_constant[i].get() & 1;
            pos_constant[i] = CheckedInt::new(v);
            i += 1;
        }
    }

    let mut clause_set = ClauseSet::new();
    let mut result = result;

    let mut i = 0;
    let mut carry: Vec<Lit> = vec![];
    while i < pos_vars.len() {
        let mut infos = vec![];
        let mut encoding = vec![];

        let cnt = pos_constant[i]
            + CheckedInt::new(pos_vars[i].len() as i32)
            + CheckedInt::new(carry.len() as i32);
        for &lit in &pos_vars[i] {
            encoding.push(OrderEncoding {
                domain: vec![CheckedInt::new(0), CheckedInt::new(1)],
                lits: vec![lit],
            });
        }
        for e in &encoding {
            infos.push(LinearInfo::Order(LinearInfoForOrderEncoding {
                coef: CheckedInt::new(1),
                encoding: e,
            }));
        }

        let mut carry_domain = vec![];
        for j in 0..=(carry.len() as i32) {
            carry_domain.push(CheckedInt::new(j));
        }
        let carry_encoding = OrderEncoding {
            domain: carry_domain,
            lits: carry,
        };
        infos.push(LinearInfo::Order(LinearInfoForOrderEncoding {
            coef: CheckedInt::new(1),
            encoding: &carry_encoding,
        }));

        let mut carry_next_domain = vec![];
        for j in 0..=(cnt.get() / 2) {
            carry_next_domain.push(CheckedInt::new(j));
        }
        let mut carry_next = vec![];
        for _ in 0..(cnt.get() / 2) {
            let var = new_var!(env.sat);
            carry_next.push(var.as_lit(false));
        }
        let carry_next_encoding = OrderEncoding {
            domain: carry_next_domain,
            lits: carry_next.clone(),
        };
        infos.push(LinearInfo::Order(LinearInfoForOrderEncoding {
            coef: CheckedInt::new(-2),
            encoding: &carry_next_encoding,
        }));

        while i >= result.len() {
            result.push(new_var!(env.sat).as_lit(false));
        }
        let ret_encoding = OrderEncoding {
            domain: vec![CheckedInt::new(0), CheckedInt::new(1)],
            lits: vec![result[i]],
        };
        infos.push(LinearInfo::Order(LinearInfoForOrderEncoding {
            coef: CheckedInt::new(-1),
            encoding: &ret_encoding,
        }));

        {
            let c = encode_linear_ge_mixed_from_info(&infos, pos_constant[i]);
            clause_set.append(c);
        }
        {
            for info in &mut infos {
                match info {
                    LinearInfo::Order(ord) => ord.coef *= CheckedInt::new(-1),
                    _ => unreachable!(),
                }
            }
            let c = encode_linear_ge_mixed_from_info(&infos, -pos_constant[i]);
            clause_set.append(c);
        }
        carry = carry_next;
        if !carry.is_empty() && i + 1 == pos_vars.len() {
            pos_vars.push(vec![]);
            pos_constant.push(CheckedInt::new(0));
        }

        i += 1;
    }

    (clause_set, result)
}

fn log_encoding_adder2(
    env: &mut EncoderEnv,
    values: Vec<(usize, CheckedInt, Lit)>,
    constant: CheckedInt,
) -> ClauseSet {
    if values.is_empty() {
        let mut ret = ClauseSet::new();
        if constant != 0 {
            ret.push(&[]);
        }
        return ret;
    }

    let max_ofs = values.iter().map(|(ofs, _, _)| *ofs).max().unwrap() + 1;
    assert!(max_ofs < i32::BITS as usize);

    let mut lits_by_ofs: Vec<Vec<(CheckedInt, Lit)>> = vec![vec![]; max_ofs];
    for (ofs, coef, lit) in values {
        lits_by_ofs[ofs].push((coef, lit));
    }

    let mut clause_set = ClauseSet::new();

    let mut carry_low = CheckedInt::new(0);
    let mut carry_high = CheckedInt::new(0);
    let mut carry_lits: Vec<Lit> = vec![];

    for i in 0..max_ofs {
        let mut low = carry_low;
        let mut high = carry_high;

        let mut order_encodings = vec![];

        for &(coef, lit) in &lits_by_ofs[i] {
            if coef < 0 {
                low += coef;
                order_encodings.push(OrderEncoding {
                    domain: vec![coef, CheckedInt::new(0)],
                    lits: vec![!lit],
                });
            } else if coef > 0 {
                high += coef;
                order_encodings.push(OrderEncoding {
                    domain: vec![CheckedInt::new(0), coef],
                    lits: vec![lit],
                });
            }
        }

        assert_eq!(carry_high - carry_low, carry_lits.len() as i32);
        {
            let domain = (carry_low.get()..=carry_high.get())
                .map(CheckedInt::new)
                .collect::<Vec<_>>();
            order_encodings.push(OrderEncoding {
                domain,
                lits: carry_lits.clone(),
            });
        }

        let target = if i + 1 == max_ofs {
            CheckedInt::new(constant.get() >> i)
        } else {
            CheckedInt::new((constant.get() >> i) & 1)
        };

        let new_carry_low;
        let new_carry_high;

        if i + 1 == max_ofs {
            new_carry_low = CheckedInt::new(0);
            new_carry_high = CheckedInt::new(0);
        } else {
            new_carry_low = (low + target).div_ceil(CheckedInt::new(2));
            new_carry_high = (high + target).div_floor(CheckedInt::new(2));
            if new_carry_low > new_carry_high {
                let mut ret = ClauseSet::new();
                ret.push(&[]);
                return ret;
            }
        }

        let mut new_carry_lits = vec![];
        for _ in 0..(new_carry_high - new_carry_low).get() {
            new_carry_lits.push(new_var!(env.sat).as_lit(false));
        }
        for i in 1..new_carry_lits.len() {
            env.sat
                .add_clause(&[new_carry_lits[i - 1], !new_carry_lits[i]]);
        }

        {
            let domain = (new_carry_low.get()..=new_carry_high.get())
                .rev()
                .map(|x| CheckedInt::new(x) * CheckedInt::new(-2))
                .collect::<Vec<_>>();
            let lits = new_carry_lits.iter().rev().map(|x| !*x).collect();
            order_encodings.push(OrderEncoding { domain, lits });
        }

        let mut infos = vec![];
        for encoding in &order_encodings {
            infos.push(LinearInfo::Order(LinearInfoForOrderEncoding {
                coef: CheckedInt::new(1),
                encoding,
            }));
        }

        let c = encode_linear_eq_mixed_from_info(infos, target);
        clause_set.append(c);

        carry_low = new_carry_low;
        carry_high = new_carry_high;
        carry_lits = new_carry_lits;
    }

    clause_set
}

#[allow(unused)]
fn log_encoding_adder2_direct(
    env: &mut EncoderEnv,
    values: Vec<(usize, CheckedInt, Lit)>,
    constant: CheckedInt,
) -> ClauseSet {
    if values.is_empty() {
        let mut ret = ClauseSet::new();
        if constant != 0 {
            ret.push(&[]);
        }
        return ret;
    }

    let max_ofs = values.iter().map(|(ofs, _, _)| *ofs).max().unwrap() + 1;
    assert!(max_ofs < i32::BITS as usize);

    let mut lits_by_ofs: Vec<Vec<(CheckedInt, Lit)>> = vec![vec![]; max_ofs];
    for (ofs, coef, lit) in values {
        lits_by_ofs[ofs].push((coef, lit));
    }

    let mut clause_set = ClauseSet::new();

    let mut carry_low = CheckedInt::new(0);
    let mut carry_high = CheckedInt::new(0);
    let t = new_var!(env.sat).as_lit(false);
    env.sat.add_clause(&[t]);
    let mut carry_lits: Vec<Lit> = vec![t];

    for i in 0..max_ofs {
        let mut low = carry_low;
        let mut high = carry_high;

        let mut direct_encodings = vec![];

        for &(coef, lit) in &lits_by_ofs[i] {
            if coef < 0 {
                low += coef;
                direct_encodings.push(DirectEncoding {
                    domain: vec![coef, CheckedInt::new(0)],
                    lits: vec![lit, !lit],
                });
            } else if coef > 0 {
                high += coef;
                direct_encodings.push(DirectEncoding {
                    domain: vec![CheckedInt::new(0), coef],
                    lits: vec![!lit, lit],
                });
            }
        }

        assert_eq!(carry_high - carry_low, carry_lits.len() as i32 - 1);
        {
            let domain = (carry_low.get()..=carry_high.get())
                .map(CheckedInt::new)
                .collect::<Vec<_>>();
            direct_encodings.push(DirectEncoding {
                domain,
                lits: carry_lits.clone(),
            });
        }

        let target = if i + 1 == max_ofs {
            CheckedInt::new(constant.get() >> i)
        } else {
            CheckedInt::new((constant.get() >> i) & 1)
        };

        let new_carry_low;
        let new_carry_high;

        if i + 1 == max_ofs {
            new_carry_low = CheckedInt::new(0);
            new_carry_high = CheckedInt::new(0);
        } else {
            new_carry_low = (low + target).div_ceil(CheckedInt::new(2));
            new_carry_high = (high + target).div_floor(CheckedInt::new(2));
            if new_carry_low > new_carry_high {
                let mut ret = ClauseSet::new();
                ret.push(&[]);
                return ret;
            }
        }

        let mut new_carry_lits = vec![];
        for _ in 0..=(new_carry_high - new_carry_low).get() {
            new_carry_lits.push(new_var!(env.sat).as_lit(false));
        }
        env.sat.add_clause(&new_carry_lits);
        for i in 1..new_carry_lits.len() {
            for j in 0..i {
                env.sat
                    .add_clause(&[!new_carry_lits[j], !new_carry_lits[i]]);
            }
        }

        {
            let domain = (new_carry_low.get()..=new_carry_high.get())
                .rev()
                .map(|x| CheckedInt::new(x) * CheckedInt::new(-2))
                .collect::<Vec<_>>();
            let lits = new_carry_lits.iter().rev().copied().collect();
            direct_encodings.push(DirectEncoding { domain, lits });
        }

        let mut infos = vec![];
        for encoding in &direct_encodings {
            infos.push(LinearInfoForDirectEncoding {
                coef: CheckedInt::new(1),
                encoding,
            });
        }

        let c = encode_linear_eq_direct_from_info(env, &infos, target);
        clause_set.append(c);

        carry_low = new_carry_low;
        carry_high = new_carry_high;
        carry_lits = new_carry_lits;
    }

    clause_set
}

pub(super) fn encode_mul_log(env: &mut EncoderEnv, x: IntVar, y: IntVar, m: IntVar) -> ClauseSet {
    let x_repr = env.map.int_map[x]
        .as_ref()
        .unwrap()
        .log_encoding
        .as_ref()
        .unwrap()
        .lits
        .clone();
    let y_repr = env.map.int_map[y]
        .as_ref()
        .unwrap()
        .log_encoding
        .as_ref()
        .unwrap()
        .lits
        .clone();
    let m_repr = env.map.int_map[m]
        .as_ref()
        .unwrap()
        .log_encoding
        .as_ref()
        .unwrap()
        .lits
        .clone();
    let m_repr_len = m_repr.len();

    let (mut clause_set, m_all) = log_encoding_multiplier(env, x_repr, y_repr, m_repr);

    for i in m_repr_len..m_all.len() {
        clause_set.push(&[!m_all[i]]);
    }
    clause_set
}

fn log_encoding_multiplier(
    env: &mut EncoderEnv,
    value1: Vec<Lit>,
    value2: Vec<Lit>,
    result: Vec<Lit>,
) -> (ClauseSet, Vec<Lit>) {
    let mut clause_set = ClauseSet::new();

    let mut sum_values = vec![];
    for (i, &x) in value1.iter().enumerate() {
        let mut row = vec![];
        #[allow(unused)]
        for (j, &y) in value2.iter().enumerate() {
            let m = new_var!(env.sat, "mul.{}.{}.{}", env.sat.num_var(), i, j).as_lit(false);
            row.push(m);

            // m <=> (x & y)
            clause_set.push(&[!m, x]);
            clause_set.push(&[!m, y]);
            clause_set.push(&[!x, !y, m]);
        }
        sum_values.push((i, row));
    }

    let (new_clause_set, ret) = log_encoding_adder(env, sum_values, vec![], result);
    clause_set.append(new_clause_set);
    (clause_set, ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::encode_constraint;
    use super::super::tests::{linear_sum, EncoderTester};
    use crate::domain::Domain;
    use crate::norm_csp::{Constraint, ExtraConstraint, LinearLit};

    #[test]
    fn test_encode_log_var() {
        let mut tester = EncoderTester::new();

        let _ = tester.add_int_var_log_encoding(Domain::range(2, 11));

        tester.run_check();
    }

    #[test]
    fn test_encode_log_var_with_negative_values() {
        let mut tester = EncoderTester::new();

        let _ = tester.add_int_var_log_encoding(Domain::range(-5, 3));

        tester.run_check();
    }

    #[test]
    fn test_encode_log_var_all_negative() {
        let mut tester = EncoderTester::new();

        let _ = tester.add_int_var_log_encoding(Domain::range(-10, -2));

        tester.run_check();
    }

    #[test]
    fn test_encode_linear_eq_log_encoding_with_negative_values() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::range(-5, 5));
        let y = tester.add_int_var_log_encoding(Domain::range(-7, 2));
        let z = tester.add_int_var_log_encoding(Domain::range(-1, 8));

        let lit = LinearLit::new(linear_sum(&[(x, 2), (y, -1), (z, 1)], 3), CmpOp::Eq);
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Eq);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_log_var_sparse_domain() {
        let mut tester = EncoderTester::new();

        // Test sparse domain with positive values
        let _x = tester.add_int_var_log_encoding(Domain::enumerative(vec![1, 3, 7, 15]));

        tester.run_check();
    }

    #[test]
    fn test_encode_log_var_sparse_domain_with_negatives() {
        let mut tester = EncoderTester::new();

        // Test sparse domain with negative and positive values
        let _x = tester.add_int_var_log_encoding(Domain::enumerative(vec![-10, -5, 0, 3, 8]));

        tester.run_check();
    }

    #[test]
    fn test_encode_linear_eq_log_encoding_sparse_domain() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::enumerative(vec![1, 3, 5, 7]));
        let y = tester.add_int_var_log_encoding(Domain::enumerative(vec![2, 4, 6]));
        let z = tester.add_int_var_log_encoding(Domain::enumerative(vec![5, 8, 11, 14]));

        // Test linear constraint: x + y = z
        let lit = LinearLit::new(linear_sum(&[(x, 1), (y, 1), (z, -1)], 0), CmpOp::Eq);
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Eq);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_linear_eq_log_encoding_sparse_domain_with_negatives() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::enumerative(vec![-5, -2, 1, 4]));
        let y = tester.add_int_var_log_encoding(Domain::enumerative(vec![-3, 0, 2]));

        // Test linear constraint: 2*x + y = 0
        let lit = LinearLit::new(linear_sum(&[(x, 2), (y, 1)], 0), CmpOp::Eq);
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Eq);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_linear_eq_log_encoding_1() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::range(2, 11));
        let y = tester.add_int_var_log_encoding(Domain::range(3, 8));
        let z = tester.add_int_var_log_encoding(Domain::range(1, 22));

        let lit = LinearLit::new(linear_sum(&[(x, 1), (y, 2), (z, -1)], 0), CmpOp::Eq);
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Eq);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_linear_eq_log_encoding_2() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::range(17, 98));
        let y = tester.add_int_var_log_encoding(Domain::range(35, 80));
        let z = tester.add_int_var_log_encoding(Domain::range(90, 257));

        let lit = LinearLit::new(linear_sum(&[(x, 1), (y, 2), (z, -1)], -1), CmpOp::Eq);
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Eq);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_linear_eq_log_encoding_3() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::range(7, 23));
        let y = tester.add_int_var_log_encoding(Domain::range(5, 19));
        let z = tester.add_int_var_log_encoding(Domain::range(3, 13));
        let w = tester.add_int_var_log_encoding(Domain::range(2, 17));

        let lit = LinearLit::new(
            linear_sum(&[(x, 1033), (y, 254), (z, 516), (w, -2231)], 0),
            CmpOp::Eq,
        );
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Eq);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_linear_ne_log_encoding() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::range(2, 7));
        let y = tester.add_int_var_log_encoding(Domain::range(3, 8));
        let z = tester.add_int_var_log_encoding(Domain::range(1, 5));

        let lit = LinearLit::new(linear_sum(&[(x, 1), (y, 2), (z, -3)], 0), CmpOp::Ne);
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Ne);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_linear_ge_log_encoding_1() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::range(2, 11));
        let y = tester.add_int_var_log_encoding(Domain::range(3, 8));
        let z = tester.add_int_var_log_encoding(Domain::range(1, 22));

        let lit = LinearLit::new(linear_sum(&[(x, 1), (y, 2), (z, -1)], 0), CmpOp::Ge);
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Ge);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_linear_ge_log_encoding_2() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::range(17, 28));
        let y = tester.add_int_var_log_encoding(Domain::range(35, 50));
        let z = tester.add_int_var_log_encoding(Domain::range(90, 107));

        let lit = LinearLit::new(linear_sum(&[(x, 1), (y, 2), (z, -1)], -1), CmpOp::Ge);
        {
            let clause_set = encode_linear_log(&mut tester.env(), &lit.sum, CmpOp::Ge);
            tester.add_clause_set(clause_set);
        }

        tester.add_constraint_linear_lit(lit);
        tester.run_check();
    }

    #[test]
    fn test_encode_linear_log_encoding_operators() {
        for op in [CmpOp::Gt, CmpOp::Le, CmpOp::Lt] {
            let mut tester = EncoderTester::new();

            let x = tester.add_int_var_log_encoding(Domain::range(2, 11));
            let y = tester.add_int_var_log_encoding(Domain::range(3, 8));
            let z = tester.add_int_var_log_encoding(Domain::range(1, 22));

            let lits = vec![LinearLit::new(
                linear_sum(&[(x, 1), (y, 2), (z, -1)], 0),
                op,
            )];
            let constraint = Constraint {
                bool_lit: vec![],
                linear_lit: lits.clone(),
            };
            encode_constraint(&mut tester.env(), constraint.clone());

            tester.add_constraint(constraint);
            tester.run_check();
        }
    }

    #[test]
    fn test_encode_mul_log() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var_log_encoding(Domain::range(19, 33));
        let y = tester.add_int_var_log_encoding(Domain::range(31, 37));
        let z = tester.add_int_var_log_encoding(Domain::range(1000, 1030));

        {
            let clause_set = encode_mul_log(&mut tester.env(), x, y, z);
            tester.add_clause_set(clause_set);
        }

        tester.add_extra_constraint(ExtraConstraint::Mul(x, y, z));
        tester.run_check();
    }
}
