use super::{ClauseSet, EncodeMap, EncoderEnv, LinearLit};
use crate::arithmetic::{CheckedInt, Range};
use crate::norm_csp::{IntVarRepresentation, LinearSum, NormCSPVars};
use crate::sat::{Lit, SAT};

pub(super) struct DirectEncoding {
    pub domain: Vec<CheckedInt>,
    pub lits: Vec<Lit>,
}

impl DirectEncoding {
    pub fn range(&self) -> Range {
        if self.domain.is_empty() {
            Range::empty()
        } else {
            Range::new(self.domain[0], self.domain[self.domain.len() - 1])
        }
    }
}

pub struct LinearInfoForDirectEncoding<'a> {
    pub coef: CheckedInt,
    pub encoding: &'a DirectEncoding,
}

impl<'a> LinearInfoForDirectEncoding<'a> {
    pub fn new(coef: CheckedInt, encoding: &'a DirectEncoding) -> LinearInfoForDirectEncoding<'a> {
        LinearInfoForDirectEncoding { coef, encoding }
    }

    pub fn domain_size(&self) -> usize {
        self.encoding.domain.len()
    }

    pub fn domain(&self, j: usize) -> CheckedInt {
        if self.coef > 0 {
            self.encoding.domain[j] * self.coef
        } else {
            self.encoding.domain[self.encoding.domain.len() - 1 - j] * self.coef
        }
    }

    pub fn domain_min(&self) -> CheckedInt {
        self.domain(0)
    }

    pub fn domain_max(&self) -> CheckedInt {
        self.domain(self.domain_size() - 1)
    }

    // The literal asserting that (the value) equals `domain(j)`.
    pub fn equals(&self, j: usize) -> Lit {
        if self.coef > 0 {
            self.encoding.lits[j]
        } else {
            self.encoding.lits[self.domain_size() - 1 - j]
        }
    }

    /// The literal asserting (x == val), or `None` if `val` is not in the domain.
    fn equals_val(&self, val: CheckedInt) -> Option<Lit> {
        let mut left = 0;
        let mut right = self.domain_size() - 1;

        while left < right {
            let mid = (left + right) / 2;
            if val <= self.domain(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if self.domain(left) == val {
            Some(self.equals(left))
        } else {
            None
        }
    }
}

pub(super) fn encode_var_direct(
    encode_map: &mut EncodeMap,
    norm_vars: &NormCSPVars,
    sat: &mut SAT,
    repr: &IntVarRepresentation,
) -> DirectEncoding {
    match repr {
        IntVarRepresentation::Domain(domain) => {
            let domain = domain.enumerate();
            assert_ne!(domain.len(), 0);
            let lits;
            #[cfg(feature = "sat-analyzer")]
            {
                let mut tmp = vec![];
                for i in 0..domain.len() {
                    tmp.push(new_var!(sat, "{}.dir=={}", var.id(), domain[i].get()).as_lit(false));
                }
                lits = tmp;
            }
            #[cfg(not(feature = "sat-analyzer"))]
            {
                lits = sat.new_vars_as_lits(domain.len());
            }
            sat.add_clause(&lits);
            for i in 1..lits.len() {
                for j in 0..i {
                    sat.add_clause(&[!lits[i], !lits[j]]);
                }
            }

            DirectEncoding { domain, lits }
        }
        &IntVarRepresentation::Binary(cond, f, t) => {
            assert!(f < t);
            let c = encode_map.convert_bool_lit(norm_vars, sat, cond);
            let domain = vec![f, t];
            let lits = vec![!c, c];
            DirectEncoding { domain, lits }
        }
    }
}

// Return Some(clause) where `clause` encodes `lit` (the truth value of `clause` is equal to that of `lit`),
// or None when `lit` always holds.
pub(super) fn encode_simple_linear_direct_encoding(
    env: &mut EncoderEnv,
    lit: &LinearLit,
) -> Option<Vec<Lit>> {
    let op = lit.op;
    assert_eq!(lit.sum.len(), 1);
    let (&var, &coef) = lit.sum.iter().next().unwrap();

    let encoding = env.map.int_map[var].as_ref().unwrap().as_direct_encoding();
    let mut oks = vec![];
    let mut ngs = vec![];
    for i in 0..encoding.domain.len() {
        let lhs = encoding.domain[i] * coef + lit.sum.constant;
        if op.compare(lhs, CheckedInt::new(0)) {
            oks.push(encoding.lits[i]);
        } else {
            ngs.push(!encoding.lits[i]);
        }
    }

    if oks.len() == encoding.domain.len() {
        None
    } else if ngs.len() == 1 {
        Some(ngs)
    } else {
        Some(oks)
    }
}

pub(super) fn encode_linear_eq_direct(env: &EncoderEnv, sum: &LinearSum) -> ClauseSet {
    let mut info = vec![];
    for (&var, &coef) in sum.iter() {
        let encoding = env.map.int_map[var].as_ref().unwrap();

        let direct_encoding = encoding.as_direct_encoding();
        info.push(LinearInfoForDirectEncoding::new(coef, direct_encoding));
    }
    info.sort_by(|encoding1, encoding2| {
        encoding1
            .encoding
            .lits
            .len()
            .cmp(&encoding2.encoding.lits.len())
    });
    encode_linear_eq_direct_from_info(env, &info, sum.constant)
}

fn encode_linear_eq_direct_two_terms(
    info: &[LinearInfoForDirectEncoding],
    constant: CheckedInt,
) -> ClauseSet {
    assert_eq!(info.len(), 2);

    let mut ret = ClauseSet::new();

    for u in 0..2 {
        let v = u ^ 1;

        for i in 0..info[u].domain_size() {
            let mut clause = vec![!info[u].equals(i)];
            clause.extend(info[v].equals_val(-constant - info[u].domain(i)));
            ret.push(&clause);
        }
    }

    ret
}

pub(super) fn encode_linear_eq_direct_from_info(
    _env: &EncoderEnv,
    info: &[LinearInfoForDirectEncoding],
    constant: CheckedInt,
) -> ClauseSet {
    if info.len() == 2 {
        return encode_linear_eq_direct_two_terms(info, constant);
    }

    fn encode_sub(
        info: &[LinearInfoForDirectEncoding],
        clause: &mut Vec<Lit>,
        idx: usize,
        lower_bound: CheckedInt,
        upper_bound: CheckedInt,
        min_relax_for_lb: Option<CheckedInt>,
        min_relax_for_ub: Option<CheckedInt>,
        clauses_buf: &mut ClauseSet,
    ) {
        if lower_bound > 0 || upper_bound < 0 {
            let mut cannot_prune = true;
            if lower_bound > 0
                && min_relax_for_lb
                    .map(|m| lower_bound - m <= 0)
                    .unwrap_or(true)
            {
                cannot_prune = true;
            }
            if upper_bound < 0
                && min_relax_for_ub
                    .map(|m| upper_bound + m >= 0)
                    .unwrap_or(true)
            {
                cannot_prune = true;
            }
            if cannot_prune {
                clauses_buf.push(clause);
            }
            return;
        }
        if idx == info.len() {
            return;
        }
        if idx == info.len() - 1 {
            let direct_encoding = &info[idx];
            let lb_for_this_term = direct_encoding.domain_min();
            let ub_for_this_term = direct_encoding.domain_max();

            let prev_lb = lower_bound - lb_for_this_term;
            let prev_ub = upper_bound - ub_for_this_term;

            let mut possible_cand = vec![];

            for i in 0..direct_encoding.domain_size() {
                let value = direct_encoding.domain(i);

                if prev_ub + value < 0 || 0 < prev_lb + value {
                    continue;
                }
                possible_cand.push(direct_encoding.equals(i));
            }

            if possible_cand.len() == direct_encoding.domain_size() {
                return;
            }
            let n_possible_cand = possible_cand.len();
            clause.append(&mut possible_cand);
            clauses_buf.push(clause);
            clause.truncate(clause.len() - n_possible_cand);
            return;
        }

        let direct_encoding = &info[idx];
        let lb_for_this_term = direct_encoding.domain_min();
        let ub_for_this_term = direct_encoding.domain_max();

        for i in 0..direct_encoding.domain_size() {
            let value = direct_encoding.domain(i);
            let next_lb = lower_bound - lb_for_this_term + value;
            let next_ub = upper_bound - ub_for_this_term + value;
            let next_min_relax_for_lb = Some(
                min_relax_for_lb
                    .unwrap_or(CheckedInt::max_value())
                    .min(value - lb_for_this_term),
            );
            let next_min_relax_for_ub = Some(
                min_relax_for_ub
                    .unwrap_or(CheckedInt::max_value())
                    .min(ub_for_this_term - value),
            );
            clause.push(!direct_encoding.equals(i));
            encode_sub(
                info,
                clause,
                idx + 1,
                next_lb,
                next_ub,
                next_min_relax_for_lb,
                next_min_relax_for_ub,
                clauses_buf,
            );
            clause.pop();
        }

        encode_sub(
            info,
            clause,
            idx + 1,
            lower_bound,
            upper_bound,
            min_relax_for_lb,
            min_relax_for_ub,
            clauses_buf,
        );
    }

    let mut lower_bound = constant;
    let mut upper_bound = constant;
    for direct_encoding in info {
        lower_bound += direct_encoding.domain_min();
        upper_bound += direct_encoding.domain_max();
    }

    let mut clauses_buf = ClauseSet::new();
    encode_sub(
        info,
        &mut vec![],
        0,
        lower_bound,
        upper_bound,
        None,
        None,
        &mut clauses_buf,
    );

    clauses_buf
}

pub(super) fn encode_linear_ne_direct(env: &EncoderEnv, sum: &LinearSum) -> ClauseSet {
    let mut info = vec![];
    for (&var, &coef) in sum.iter() {
        let encoding = env.map.int_map[var].as_ref().unwrap();

        let direct_encoding = encoding.as_direct_encoding();
        info.push(LinearInfoForDirectEncoding::new(coef, direct_encoding));
    }

    fn encode_sub(
        info: &[LinearInfoForDirectEncoding],
        clause: &mut Vec<Lit>,
        idx: usize,
        lower_bound: CheckedInt,
        upper_bound: CheckedInt,
        clauses_buf: &mut ClauseSet,
    ) {
        if lower_bound > 0 || upper_bound < 0 {
            return;
        }
        if idx == info.len() {
            assert_eq!(lower_bound, upper_bound);
            if lower_bound == 0 {
                clauses_buf.push(clause);
            }
            return;
        }
        if idx == info.len() - 1 {
            let direct_encoding = &info[idx];
            let lb_for_this_term = direct_encoding.domain_min();
            let ub_for_this_term = direct_encoding.domain_max();

            assert_eq!(
                lower_bound - lb_for_this_term,
                upper_bound - ub_for_this_term
            );
            let prev_val = lower_bound - lb_for_this_term;

            let mut forbidden = None;
            for i in 0..direct_encoding.domain_size() {
                let value = direct_encoding.domain(i);

                if prev_val + value == 0 {
                    assert!(forbidden.is_none());
                    forbidden = Some(direct_encoding.equals(i));
                }
            }

            if let Some(forbidden) = forbidden {
                clause.push(!forbidden);
                clauses_buf.push(clause);
                clause.pop();
            }
            return;
        }

        let direct_encoding = &info[idx];
        let lb_for_this_term = direct_encoding.domain_min();
        let ub_for_this_term = direct_encoding.domain_max();

        for i in 0..direct_encoding.domain_size() {
            let value = direct_encoding.domain(i);
            let next_lb = lower_bound - lb_for_this_term + value;
            let next_ub = upper_bound - ub_for_this_term + value;
            clause.push(!direct_encoding.equals(i));
            encode_sub(info, clause, idx + 1, next_lb, next_ub, clauses_buf);
            clause.pop();
        }
    }

    let mut lower_bound = sum.constant;
    let mut upper_bound = sum.constant;
    for direct_encoding in &info {
        lower_bound += direct_encoding.domain_min();
        upper_bound += direct_encoding.domain_max();
    }

    let mut clauses_buf = ClauseSet::new();
    encode_sub(
        &info,
        &mut vec![],
        0,
        lower_bound,
        upper_bound,
        &mut clauses_buf,
    );

    clauses_buf
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::tests::{linear_sum, EncoderTester};
    use crate::arithmetic::CmpOp;
    use crate::domain::Domain;
    use crate::norm_csp::LinearLit;

    #[test]
    fn test_encode_simple_linear_direct_encoding() {
        for op in [
            CmpOp::Eq,
            CmpOp::Ne,
            CmpOp::Le,
            CmpOp::Lt,
            CmpOp::Ge,
            CmpOp::Gt,
        ] {
            let mut tester = EncoderTester::new();

            let x = tester.add_int_var(Domain::range(-2, 5), true);
            let lits = [LinearLit::new(linear_sum(&[(x, 1)], 1), op)];
            {
                let clause = encode_simple_linear_direct_encoding(&mut tester.env(), &lits[0]);
                if let Some(clause) = clause {
                    tester.add_clause(&clause);
                }
            }
            tester.run_check(&lits);
        }
    }

    #[test]
    fn test_encode_linear_eq_direct_two_terms() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var(Domain::range(0, 5), true);
        let y = tester.add_int_var(Domain::range(2, 6), true);

        let lits = [LinearLit::new(linear_sum(&[(x, 2), (y, -1)], 1), CmpOp::Eq)];
        {
            let clause_set = encode_linear_eq_direct(&tester.env(), &lits[0].sum);
            tester.add_clause_set(clause_set);
        }
        tester.run_check(&lits);
    }

    #[test]
    fn test_encode_linear_eq_direct() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var(Domain::range(0, 5), true);
        let y = tester.add_int_var(Domain::range(2, 6), true);
        let z = tester.add_int_var(Domain::range(-1, 4), true);

        let lits = [LinearLit::new(
            linear_sum(&[(x, 1), (y, -1), (z, 2)], -1),
            CmpOp::Eq,
        )];
        {
            let clause_set = encode_linear_eq_direct(&tester.env(), &lits[0].sum);
            tester.add_clause_set(clause_set);
        }
        tester.run_check(&lits);
    }

    #[test]
    fn test_encode_linear_ne_direct() {
        let mut tester = EncoderTester::new();

        let x = tester.add_int_var(Domain::range(0, 5), true);
        let y = tester.add_int_var(Domain::range(2, 6), true);
        let z = tester.add_int_var(Domain::range(-1, 4), true);

        let lits = [LinearLit::new(
            linear_sum(&[(x, 1), (y, -1), (z, 2)], -1),
            CmpOp::Ne,
        )];
        {
            let clause_set = encode_linear_ne_direct(&tester.env(), &lits[0].sum);
            tester.add_clause_set(clause_set);
        }
        tester.run_check(&lits);
    }
}
