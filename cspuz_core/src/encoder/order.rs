use super::{EncodeMap, EncoderEnv, ExtendedLit};
use crate::arithmetic::{CheckedInt, Range};
use crate::norm_csp::{IntVarRepresentation, LinearSum, NormCSPVars};
use crate::sat::{Lit, SAT};

/// Order encoding of an integer variable with domain of `domain`.
/// `vars[i]` is the logical variable representing (the value of this int variable) >= `domain[i+1]`.
pub(super) struct OrderEncoding {
    pub domain: Vec<CheckedInt>,
    pub lits: Vec<Lit>,
}

impl OrderEncoding {
    pub fn range(&self) -> Range {
        if self.domain.is_empty() {
            Range::empty()
        } else {
            Range::new(self.domain[0], self.domain[self.domain.len() - 1])
        }
    }
}

/// Helper struct for encoding linear constraints on variables represented in order encoding.
/// With this struct, all coefficients can be virtually treated as 1.
pub(super) struct LinearInfoForOrderEncoding<'a> {
    pub coef: CheckedInt,
    pub encoding: &'a OrderEncoding,
}

impl<'a> LinearInfoForOrderEncoding<'a> {
    pub fn new(coef: CheckedInt, encoding: &'a OrderEncoding) -> LinearInfoForOrderEncoding<'a> {
        LinearInfoForOrderEncoding { coef, encoding }
    }

    pub fn domain_size(&self) -> usize {
        self.encoding.domain.len()
    }

    /// j-th smallest domain value after normalizing negative coefficients
    pub fn domain(&self, j: usize) -> CheckedInt {
        if self.coef > 0 {
            self.encoding.domain[j] * self.coef
        } else {
            self.encoding.domain[self.encoding.domain.len() - 1 - j] * self.coef
        }
    }

    #[allow(unused)]
    fn domain_min(&self) -> CheckedInt {
        self.domain(0)
    }

    pub fn domain_max(&self) -> CheckedInt {
        self.domain(self.domain_size() - 1)
    }

    /// The literal asserting that (the value) is at least `domain(i, j)`.
    pub fn at_least(&self, j: usize) -> Lit {
        assert!(0 < j && j < self.encoding.domain.len());
        if self.coef > 0 {
            self.encoding.lits[j - 1]
        } else {
            !self.encoding.lits[self.encoding.domain.len() - 1 - j]
        }
    }

    /// The literal asserting (x >= val) under the assumption that x is in the domain.
    pub fn at_least_val(&self, val: CheckedInt) -> ExtendedLit {
        let dom_size = self.domain_size();

        if val <= self.domain(0) {
            ExtendedLit::True
        } else if val > self.domain(dom_size - 1) {
            ExtendedLit::False
        } else {
            // compute the largest j such that val <= domain[j]
            let mut left = 0;
            let mut right = dom_size - 1;

            while left < right {
                let mid = (left + right) / 2;
                if val <= self.domain(mid) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            ExtendedLit::Lit(self.at_least(left))
        }
    }
}

pub(super) fn encode_var_order(
    encode_map: &mut EncodeMap,
    norm_vars: &NormCSPVars,
    sat: &mut SAT,
    repr: &IntVarRepresentation,
) -> OrderEncoding {
    match repr {
        IntVarRepresentation::Domain(domain) => {
            let domain = domain.enumerate();
            assert_ne!(domain.len(), 0);
            let lits;
            #[cfg(feature = "sat-analyzer")]
            {
                let mut tmp = vec![];
                for i in 0..domain.len() - 1 {
                    tmp.push(
                        new_var!(sat, "{}.ord>={}", var.id(), domain[i + 1].get()).as_lit(false),
                    );
                }
                lits = tmp;
            }
            #[cfg(not(feature = "sat-analyzer"))]
            {
                lits = sat.new_vars_as_lits(domain.len() - 1);
            }
            for i in 1..lits.len() {
                // vars[i] implies vars[i - 1]
                sat.add_clause(&[!lits[i], lits[i - 1]]);
            }

            OrderEncoding { domain, lits }
        }
        &IntVarRepresentation::Binary {
            cond,
            v_false,
            v_true,
        } => {
            assert!(v_false < v_true);
            let domain = vec![v_false, v_true];
            let lits = vec![encode_map.convert_bool_lit(norm_vars, sat, cond)];
            OrderEncoding { domain, lits }
        }
    }
}

pub(super) fn is_ge_order_encoding_native_applicable(env: &EncoderEnv, sum: &LinearSum) -> bool {
    for (&var, _) in sum.iter() {
        if env.map.int_map[var]
            .as_ref()
            .unwrap()
            .order_encoding
            .is_none()
        {
            return false;
        }
    }
    if sum.len() > env.config.native_linear_encoding_terms {
        return false;
    }
    let mut domain_product = 1usize;
    for (&var, _) in sum.iter() {
        domain_product *= env.map.int_map[var]
            .as_ref()
            .unwrap()
            .as_order_encoding()
            .domain
            .len();
    }
    domain_product >= env.config.native_linear_encoding_domain_product_threshold
}

pub(super) fn encode_linear_ge_order_encoding_native(env: &mut EncoderEnv, sum: &LinearSum) {
    let mut info = vec![];
    for (&v, &c) in sum.iter() {
        assert_ne!(c, 0);
        info.push(LinearInfoForOrderEncoding::new(
            c,
            env.map.int_map[v].as_ref().unwrap().as_order_encoding(),
        ));
    }

    let mut lits = vec![];
    let mut domain = vec![];
    let mut coefs = vec![];
    let constant = sum.constant.get();

    for i in 0..info.len() {
        let mut lits_r = vec![];
        let mut domain_r = vec![];
        for j in 0..info[i].domain_size() {
            if j > 0 {
                lits_r.push(info[i].at_least(j));
            }
            domain_r.push(info[i].domain(j).get());
        }
        lits.push(lits_r);
        domain.push(domain_r);
        coefs.push(1);
    }

    env.sat.add_order_encoding_linear(
        lits,
        domain,
        coefs,
        constant,
        env.config.order_encoding_linear_mode,
    );
}

#[cfg(test)]
mod tests {
    use crate::sat::OrderEncodingLinearMode;

    use super::*;

    use super::super::tests::{linear_sum, EncoderTester};
    use crate::arithmetic::CmpOp;
    use crate::domain::Domain;
    use crate::norm_csp::{Constraint, LinearLit};

    #[test]
    fn test_encode_linear_ge_order_encoding_native() {
        for mode in [
            OrderEncodingLinearMode::Cpp,
            OrderEncodingLinearMode::Rust,
            OrderEncodingLinearMode::RustOptimized,
        ] {
            let mut tester = EncoderTester::new();
            tester.config.order_encoding_linear_mode = mode;

            let x = tester.add_int_var(Domain::range(0, 5), false);
            let y = tester.add_int_var(Domain::range(2, 6), false);
            let z = tester.add_int_var(Domain::range(-1, 4), false);

            let lits = [LinearLit::new(
                linear_sum(&[(x, 3), (y, -4), (z, 2)], -1),
                CmpOp::Ge,
            )];
            encode_linear_ge_order_encoding_native(&mut tester.env(), &lits[0].sum);

            tester.add_constraint(Constraint {
                bool_lit: vec![],
                linear_lit: lits.to_vec(),
            });
            tester.run_check();
        }
    }
}
