mod direct;
#[cfg(feature = "csp-extra-constraints")]
mod log;
mod mixed;
mod order;

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::ops::Index;

use super::config::Config;
use super::norm_csp::{
    BoolLit, BoolVar, Constraint, ExtraConstraint, IntVar, IntVarRepresentation, LinearLit,
    LinearSum, NormCSP, NormCSPVars,
};
use super::sat::{Backend, Lit, SATModel, SAT};
use crate::arithmetic::{CheckedInt, CmpOp, Range};
use crate::util::ConvertMap;

struct ClauseSet {
    data: Vec<Lit>,
    indices: Vec<usize>,
}

impl ClauseSet {
    fn new() -> ClauseSet {
        ClauseSet {
            data: vec![],
            indices: vec![0],
        }
    }

    fn len(&self) -> usize {
        self.indices.len() - 1
    }

    fn push(&mut self, clause: &[Lit]) {
        self.indices.push(self.data.len() + clause.len());
        for &l in clause {
            self.data.push(l);
        }
    }

    fn append(&mut self, mut other: ClauseSet) {
        let offset = self.data.len();
        self.data.append(&mut other.data);
        for i in 1..other.indices.len() {
            self.indices.push(other.indices[i] + offset);
        }
    }
}

impl Index<usize> for ClauseSet {
    type Output = [Lit];

    fn index(&self, index: usize) -> &Self::Output {
        let start = self.indices[index];
        let end = self.indices[index + 1];
        &self.data[start..end]
    }
}

#[cfg(feature = "csp-extra-constraints")]
use log::LogEncoding;

#[cfg(not(feature = "csp-extra-constraints"))]
struct LogEncoding;

struct Encoding {
    order_encoding: Option<order::OrderEncoding>,
    direct_encoding: Option<direct::DirectEncoding>,
    log_encoding: Option<LogEncoding>,
}

impl Encoding {
    fn order_encoding(enc: order::OrderEncoding) -> Encoding {
        Encoding {
            order_encoding: Some(enc),
            direct_encoding: None,
            log_encoding: None,
        }
    }

    fn direct_encoding(enc: direct::DirectEncoding) -> Encoding {
        Encoding {
            order_encoding: None,
            direct_encoding: Some(enc),
            log_encoding: None,
        }
    }

    #[allow(unused)]
    fn log_encoding(enc: LogEncoding) -> Encoding {
        Encoding {
            order_encoding: None,
            direct_encoding: None,
            log_encoding: Some(enc),
        }
    }

    fn as_order_encoding(&self) -> &order::OrderEncoding {
        self.order_encoding.as_ref().unwrap()
    }

    fn as_direct_encoding(&self) -> &direct::DirectEncoding {
        self.direct_encoding.as_ref().unwrap()
    }

    fn is_direct_encoding(&self) -> bool {
        self.direct_encoding.is_some()
    }

    fn is_direct_or_order_encoding(&self) -> bool {
        self.order_encoding.is_some() || self.direct_encoding.is_some()
    }
    fn range(&self) -> Range {
        #[allow(unused)]
        if let Some(order_encoding) = &self.order_encoding {
            order_encoding.range()
        } else if let Some(direct_encoding) = &self.direct_encoding {
            direct_encoding.range()
        } else if let Some(log_encoding) = &self.log_encoding {
            #[cfg(feature = "csp-extra-constraints")]
            {
                log_encoding.range
            }

            #[cfg(not(feature = "csp-extra-constraints"))]
            {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[allow(unused)]
    fn repr_literals(&self) -> Vec<Lit> {
        let mut ret = vec![];
        if let Some(order_encoding) = &self.order_encoding {
            ret.extend_from_slice(&order_encoding.lits);
        }
        if let Some(direct_encoding) = &self.direct_encoding {
            ret.extend_from_slice(&direct_encoding.lits);
        }

        #[cfg(feature = "csp-extra-constraints")]
        if let Some(log_encoding) = &self.log_encoding {
            ret.extend_from_slice(&log_encoding.lits);
        }
        ret
    }
}

#[cfg(feature = "sat-analyzer")]
macro_rules! new_var {
    ( $sat:expr, $( $x:expr ),* ) => {
        {
            let name = format!($($x,)*);
            $sat.new_var(&name)
        }
    };

    ( $sat:expr ) => {
        {
            let name = format!("aux_{}", $sat.num_var());
            $sat.new_var(&name)
        }
    }
}

#[cfg(not(feature = "sat-analyzer"))]
macro_rules! new_var {
    ( $sat:expr, $( $x:expr ),* ) => {
        $sat.new_var()
    };

    ( $sat:expr ) => {
        $sat.new_var()
    };
}

#[allow(unused)]
#[cfg(feature = "sat-analyzer")]
macro_rules! new_vars_as_lits {
    ( $sat:expr, $n:expr, $( $x:expr ),* ) => {
        {
            let name = format!($($x,)*);
            $sat.new_vars_as_lits($n, &name)
        }
    };
}

#[allow(unused)]
#[cfg(not(feature = "sat-analyzer"))]
macro_rules! new_vars_as_lits {
    ( $sat:expr, $n:expr, $( $x:expr ),* ) => {
        $sat.new_vars_as_lits($n)
    };
}

use new_var;
#[allow(unused)]
use new_vars_as_lits;

pub struct EncodeMap {
    bool_map: ConvertMap<BoolVar, Option<Lit>>, // mapped to Lit rather than Var so that further optimization can be done
    int_map: ConvertMap<IntVar, Option<Encoding>>,
}

impl EncodeMap {
    pub fn new() -> EncodeMap {
        EncodeMap {
            bool_map: ConvertMap::new(),
            int_map: ConvertMap::new(),
        }
    }

    fn convert_bool_var(&mut self, _norm_vars: &NormCSPVars, sat: &mut SAT, var: BoolVar) -> Lit {
        match self.bool_map[var] {
            Some(x) => x,
            None => {
                let ret = new_var!(sat, "{}", var.id()).as_lit(false);
                self.bool_map[var] = Some(ret);
                ret
            }
        }
    }

    fn convert_bool_lit(&mut self, norm_vars: &NormCSPVars, sat: &mut SAT, lit: BoolLit) -> Lit {
        let var_lit = self.convert_bool_var(norm_vars, sat, lit.var);
        if lit.negated {
            !var_lit
        } else {
            var_lit
        }
    }

    fn convert_int_var_order_encoding(
        &mut self,
        norm_vars: &NormCSPVars,
        sat: &mut SAT,
        var: IntVar,
    ) {
        assert!(self.int_map[var].is_none());
        self.int_map[var] = Some(Encoding::order_encoding(order::encode_var_order(
            self,
            norm_vars,
            sat,
            norm_vars.int_var(var),
        )));
    }

    fn convert_int_var_direct_encoding(
        &mut self,
        norm_vars: &NormCSPVars,
        sat: &mut SAT,
        var: IntVar,
    ) {
        assert!(self.int_map[var].is_none());
        self.int_map[var] = Some(Encoding::direct_encoding(direct::encode_var_direct(
            self,
            norm_vars,
            sat,
            norm_vars.int_var(var),
        )));
    }

    #[cfg(not(feature = "csp-extra-constraints"))]
    fn convert_int_var_log_encoding(&mut self, _: &NormCSPVars, _: &mut SAT, _: IntVar) {
        panic!("feature not enabled");
    }

    #[cfg(feature = "csp-extra-constraints")]
    fn convert_int_var_log_encoding(
        &mut self,
        norm_vars: &NormCSPVars,
        sat: &mut SAT,
        var: IntVar,
    ) {
        assert!(self.int_map[var].is_none());
        self.int_map[var] = Some(Encoding::log_encoding(log::encode_var_log(
            sat,
            norm_vars.int_var(var),
        )));
    }

    pub fn get_bool_var(&self, var: BoolVar) -> Option<Lit> {
        self.bool_map[var]
    }

    pub fn get_bool_lit(&self, lit: BoolLit) -> Option<Lit> {
        self.bool_map[lit.var].map(|l| if lit.negated { !l } else { l })
    }

    pub(crate) fn get_int_value_checked(
        &self,
        model: &SATModel,
        var: IntVar,
    ) -> Option<CheckedInt> {
        if self.int_map[var].is_none() {
            return None;
        }
        let encoding = self.int_map[var].as_ref().unwrap();

        #[allow(unused)]
        if let Some(encoding) = &encoding.order_encoding {
            // Find the number of true value in `encoding.vars`
            let mut left = 0;
            let mut right = encoding.lits.len();
            while left < right {
                let mid = (left + right + 1) / 2;
                if model.assignment_lit(encoding.lits[mid - 1]) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            Some(encoding.domain[left])
        } else if let Some(encoding) = &encoding.direct_encoding {
            let mut ret = None;
            for i in 0..encoding.lits.len() {
                if model.assignment_lit(encoding.lits[i]) {
                    assert!(
                        ret.is_none(),
                        "multiple indicator bits are set for a direct-encoded variable"
                    );
                    ret = Some(encoding.domain[i]);
                }
            }
            assert!(
                ret.is_some(),
                "no indicator bits are set for a direct-encoded variable"
            );
            ret
        } else if let Some(encoding) = &encoding.log_encoding {
            #[cfg(feature = "csp-extra-constraints")]
            {
                let mut ret = 0;
                for i in 0..encoding.lits.len() {
                    if model.assignment_lit(encoding.lits[i]) {
                        ret |= 1 << i;
                    }
                }
                Some(CheckedInt::new(ret))
            }

            #[cfg(not(feature = "csp-extra-constraints"))]
            {
                panic!("feature not enabled");
            }
        } else {
            panic!();
        }
    }

    pub fn get_int_value(&self, model: &SATModel, var: IntVar) -> Option<i32> {
        self.get_int_value_checked(model, var).map(CheckedInt::get)
    }
}

struct EncoderEnv<'a, 'b, 'c, 'd> {
    norm_vars: &'a mut NormCSPVars,
    sat: &'b mut SAT,
    map: &'c mut EncodeMap,
    config: &'d Config,
}

impl EncoderEnv<'_, '_, '_, '_> {
    fn convert_bool_lit(&mut self, lit: BoolLit) -> Lit {
        self.map.convert_bool_lit(self.norm_vars, self.sat, lit)
    }
}

pub fn encode(norm: &mut NormCSP, sat: &mut SAT, map: &mut EncodeMap, config: &Config) {
    let new_vars = norm.unencoded_int_vars().collect::<Vec<_>>();
    let constrs = std::mem::replace(&mut norm.constraints, vec![]);
    let extra_constrs = std::mem::replace(&mut norm.extra_constraints, vec![]);

    let scheme =
        decide_encode_schemes(config, &norm.vars, map, &new_vars, &constrs, &extra_constrs);

    for &var in &new_vars {
        match scheme.get(&var).unwrap() {
            EncodeScheme::Direct => map.convert_int_var_direct_encoding(&norm.vars, sat, var),
            EncodeScheme::Order => map.convert_int_var_order_encoding(&norm.vars, sat, var),
            EncodeScheme::Log => map.convert_int_var_log_encoding(&norm.vars, sat, var),
        }
    }

    let mut env = EncoderEnv {
        norm_vars: &mut norm.vars,
        sat,
        map,
        config,
    };

    for &var in &norm.prenormalize_vars {
        env.convert_bool_lit(BoolLit::new(var, false));
    }

    for constr in constrs {
        encode_constraint(&mut env, constr);
    }

    for constr in extra_constrs {
        match constr {
            ExtraConstraint::ActiveVerticesConnected(vertices, edges) => {
                let lits = vertices
                    .into_iter()
                    .map(|l| env.convert_bool_lit(l))
                    .collect::<Vec<_>>();
                // TODO: handle failure of addition of constraint
                env.sat.add_active_vertices_connected(lits, edges);
            }
            #[cfg(feature = "csp-extra-constraints")]
            ExtraConstraint::Mul(x, y, m) => {
                let x_log = env.map.int_map[x].as_ref().unwrap().log_encoding.is_some();
                let y_log = env.map.int_map[y].as_ref().unwrap().log_encoding.is_some();
                let m_log = env.map.int_map[m].as_ref().unwrap().log_encoding.is_some();

                if x_log && y_log && m_log {
                    let clauses = log::encode_mul_log(&mut env, x, y, m);
                    for i in 0..clauses.len() {
                        env.sat.add_clause(&clauses[i]);
                    }
                } else {
                    // TODO: constrain the domain of m if m is encoded by order or direct
                    encode_mul_naive(&mut env, x, y, m);
                }
            }
            #[cfg(not(feature = "csp-extra-constraints"))]
            ExtraConstraint::Mul(_, _, _) => {
                panic!("feature not enabled");
            }
            #[cfg(feature = "csp-extra-constraints")]
            ExtraConstraint::ExtensionSupports(vars, supports) => {
                let encodings = vars
                    .iter()
                    .map(|&v| {
                        env.map.int_map[v]
                            .as_ref()
                            .unwrap()
                            .direct_encoding
                            .as_ref()
                            .unwrap()
                    })
                    .collect::<Vec<_>>();

                let mut vars_encoded = vec![];
                let mut supports_encoded = vec![];

                assert_eq!(vars.len(), encodings.len());
                for enc in &encodings {
                    vars_encoded.push(enc.lits.clone());
                }
                for support in &supports {
                    let mut encoded = vec![];
                    let mut out_of_domain = false;

                    assert_eq!(vars.len(), support.len());
                    for i in 0..vars.len() {
                        if let Some(x) = support[i] {
                            if let Ok(idx) = encodings[i].domain.binary_search(&x) {
                                encoded.push(Some(idx));
                            } else {
                                out_of_domain = true;
                                break;
                            }
                        } else {
                            encoded.push(None);
                        }
                    }

                    if !out_of_domain {
                        supports_encoded.push(encoded);
                    }
                }

                // TODO: handle failure of addition of constraint
                env.sat
                    .add_direct_encoding_extension_supports(&vars_encoded, &supports_encoded);
            }
            #[cfg(not(feature = "csp-extra-constraints"))]
            ExtraConstraint::ExtensionSupports(_, _) => {
                panic!("feature not enabled");
            }
            ExtraConstraint::GraphDivision(sizes, edges, edge_lits, opts) => {
                let mut domains = vec![];
                let mut dom_lits = vec![];

                for i in 0..sizes.len() {
                    if let Some(v) = sizes[i] {
                        let encoding = env.map.int_map[v].as_ref().unwrap().as_order_encoding();
                        domains.push(encoding.domain.iter().map(|x| x.get()).collect());
                        dom_lits.push(encoding.lits.clone());
                    } else {
                        domains.push(vec![]);
                        dom_lits.push(vec![]);
                    }
                }

                let edge_lits = edge_lits
                    .into_iter()
                    .map(|l| env.convert_bool_lit(l))
                    .collect::<Vec<_>>();

                env.sat.add_graph_division(
                    &domains,
                    &dom_lits,
                    &edges,
                    &edge_lits,
                    config.graph_division_mode,
                    &opts,
                );
            }
            ExtraConstraint::CustomConstraint(lits, constr) => {
                let lits = lits
                    .into_iter()
                    .map(|l| env.convert_bool_lit(l))
                    .collect::<Vec<_>>();
                if env.sat.get_backend() != Backend::Glucose {
                    todo!("custom constraints are only supported with Glucose backend");
                }
                env.sat.add_custom_constraint(lits, constr);
            }
        }
    }
    norm.num_encoded_vars = norm.vars.num_int_vars();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EncodeScheme {
    Order,
    Direct,
    Log,
}

fn decide_encode_schemes(
    config: &Config,
    norm_vars: &NormCSPVars,
    _map: &EncodeMap,
    new_vars: &[IntVar],
    new_constraints: &[Constraint],
    #[allow(unused)] new_ext_constraints: &[ExtraConstraint],
) -> BTreeMap<IntVar, EncodeScheme> {
    // TODO: consider already encoded variables
    // TODO: ExtensionSupports requires direct encoding for efficient propagation

    #[cfg(feature = "csp-extra-constraints")]
    if config.force_use_log_encoding {
        let mut ret = BTreeMap::new();
        for &var in new_vars {
            ret.insert(var, EncodeScheme::Log);
        }
        return ret;
    }

    let mut scheme = BTreeMap::new();

    #[cfg(feature = "csp-extra-constraints")]
    if config.use_log_encoding {
        // Values with large domain must be log-encoded
        let mut complex_constraints_vars = BTreeSet::new();

        for constraint in new_constraints {
            for lit in &constraint.linear_lit {
                if lit.sum.len() < 3 {
                    continue;
                }
                for (&var, _) in lit.sum.iter() {
                    complex_constraints_vars.insert(var);
                }
            }
        }
        for ext_constraint in new_ext_constraints {
            match ext_constraint {
                &ExtraConstraint::Mul(x, y, m) => {
                    complex_constraints_vars.insert(x);
                    complex_constraints_vars.insert(y);
                    complex_constraints_vars.insert(m);
                }
                ExtraConstraint::ActiveVerticesConnected(_, _) => (),
                ExtraConstraint::ExtensionSupports(_, _) => (),
                ExtraConstraint::GraphDivision(_, _, _, _) => (),
                ExtraConstraint::CustomConstraint(_, _) => (),
            }
        }

        for &var in new_vars {
            let repr = norm_vars.int_var(var);
            if let IntVarRepresentation::Domain(domain) = repr {
                if domain.num_candidates() > 500 && complex_constraints_vars.contains(&var) {
                    // TODO: make this configurable
                    scheme.insert(var, EncodeScheme::Log);
                }
            }
        }

        // Values cooccurring with log-encoded ones in some constraints must be log-encoded
        loop {
            let mut updated = false;

            for constraint in new_constraints {
                for lit in &constraint.linear_lit {
                    let has_log = lit
                        .sum
                        .iter()
                        .any(|(var, _)| scheme.get(var).map_or(false, |&x| x == EncodeScheme::Log));
                    if has_log {
                        for (var, _) in lit.sum.iter() {
                            if !scheme.contains_key(var) {
                                scheme.insert(*var, EncodeScheme::Log);
                                updated = true;
                            }
                        }
                    }
                }
            }
            for ext_constraint in new_ext_constraints {
                match ext_constraint {
                    ExtraConstraint::ActiveVerticesConnected(_, _) => (),
                    ExtraConstraint::Mul(a, b, m) => {
                        let vars = [*a, *b, *m];
                        let has_log = vars
                            .iter()
                            .any(|var| scheme.get(var).map_or(false, |&x| x == EncodeScheme::Log));
                        if has_log {
                            for var in &vars {
                                if !scheme.contains_key(var) {
                                    scheme.insert(*var, EncodeScheme::Log);
                                    updated = true;
                                }
                            }
                        }
                    }
                    ExtraConstraint::ExtensionSupports(_, _) => (),
                    ExtraConstraint::GraphDivision(_, _, _, _) => (),
                    ExtraConstraint::CustomConstraint(_, _) => (),
                }
            }

            if !updated {
                break;
            }
        }
    }

    if config.use_direct_encoding {
        let mut direct_encoding_vars = BTreeSet::<IntVar>::new();
        for &var in new_vars {
            let maybe_direct_encoding = match norm_vars.int_var(var) {
                IntVarRepresentation::Domain(_) => true,
                IntVarRepresentation::Binary { .. } => config.direct_encoding_for_binary_vars,
            };
            if maybe_direct_encoding && !scheme.get(&var).map_or(false, |&x| x == EncodeScheme::Log)
            {
                direct_encoding_vars.insert(var);
            }
        }
        for constr in new_constraints {
            for lit in &constr.linear_lit {
                // TODO: use direct encoding for more complex cases
                let is_simple = (lit.op == CmpOp::Eq || lit.op == CmpOp::Ne) && lit.sum.len() <= 2;
                if !is_simple {
                    for (v, _) in lit.sum.iter() {
                        direct_encoding_vars.remove(v);
                    }
                }
            }
        }
        for ext in new_ext_constraints {
            // GraphDivision requires size variables to be order-encoded
            if let ExtraConstraint::GraphDivision(sizes, _, _, _) = ext {
                for v in sizes.iter().flatten() {
                    direct_encoding_vars.remove(v);
                }
            }
        }

        for &var in &direct_encoding_vars {
            let repr = norm_vars.int_var(var);
            let use_direct_encoding = match repr {
                IntVarRepresentation::Domain(domain) => domain.num_candidates() <= 500,
                _ => true,
            };
            if use_direct_encoding {
                scheme.insert(var, EncodeScheme::Direct);
            }
        }
    }

    let mut ret = BTreeMap::new();
    for &var in new_vars {
        ret.insert(
            var,
            scheme.get(&var).cloned().unwrap_or(EncodeScheme::Order),
        );
    }

    ret
}

fn is_unsatisfiable_linear(env: &EncoderEnv, linear_lit: &LinearLit) -> bool {
    let mut range = Range::constant(linear_lit.sum.constant);
    for (&var, &coef) in linear_lit.sum.iter() {
        let encoding = env.map.int_map[var].as_ref().unwrap();
        let var_range = encoding.range();
        range = range + var_range * coef;
    }
    match linear_lit.op {
        CmpOp::Eq => range.low > 0 || range.high < 0,
        CmpOp::Ne => range.low == 0 && range.high == 0,
        CmpOp::Le => range.low > 0,
        CmpOp::Lt => range.low >= 0,
        CmpOp::Ge => range.high < 0,
        CmpOp::Gt => range.high <= 0,
    }
}

fn encode_constraint(env: &mut EncoderEnv, constr: Constraint) {
    let mut bool_lits = constr
        .bool_lit
        .into_iter()
        .map(|lit| env.convert_bool_lit(lit))
        .collect::<Vec<_>>();
    if constr.linear_lit.is_empty() {
        env.sat.add_clause(&bool_lits);
        return;
    }

    // A conjunction (||) of disjunctions (&&) of linear literals which is equivalent to `constr`.
    let mut simplified_linears: Vec<Vec<LinearLit>> = vec![];
    for linear_lit in constr.linear_lit {
        if is_unsatisfiable_linear(env, &linear_lit) {
            continue;
        }

        match suggest_encoder(env, &linear_lit) {
            EncoderKind::MixedGe => {
                if linear_lit.op == CmpOp::Ne {
                    // `ne` is decomposed to a disjunction of 2 linear literals and handled separately
                    simplified_linears.extend(decompose_linear_lit(
                        env,
                        &LinearLit::new(linear_lit.sum.clone() * (-1) + (-1), CmpOp::Ge),
                    ));
                    simplified_linears.extend(decompose_linear_lit(
                        env,
                        &LinearLit::new(linear_lit.sum.clone() + (-1), CmpOp::Ge),
                    ));
                } else {
                    let simplified_sums = match linear_lit.op {
                        CmpOp::Eq => {
                            vec![linear_lit.sum.clone(), linear_lit.sum.clone() * -1]
                        }
                        CmpOp::Ne => unreachable!(),
                        CmpOp::Le => vec![linear_lit.sum * -1],
                        CmpOp::Lt => vec![linear_lit.sum * -1 + (-1)],
                        CmpOp::Ge => vec![linear_lit.sum],
                        CmpOp::Gt => vec![linear_lit.sum + (-1)],
                    };
                    let mut decomposed = vec![];
                    let mut is_unsat = false;
                    for sum in simplified_sums {
                        let lits = decompose_linear_lit(env, &LinearLit::new(sum, CmpOp::Ge));

                        if let Some(lits) = lits {
                            decomposed.extend(lits);
                        } else {
                            is_unsat = true;
                            break;
                        }
                    }
                    if !is_unsat {
                        simplified_linears.push(decomposed);
                    }
                }
            }
            EncoderKind::DirectSimple => {
                simplified_linears.push(vec![linear_lit]);
            }
            EncoderKind::DirectEqNe => {
                assert!(linear_lit.op == CmpOp::Eq || linear_lit.op == CmpOp::Ne);
                simplified_linears.extend(decompose_linear_lit(env, &linear_lit));
            }
            EncoderKind::Log => {
                #[cfg(feature = "csp-extra-constraints")]
                {
                    let normalized = match linear_lit.op {
                        CmpOp::Eq | CmpOp::Ne | CmpOp::Ge => linear_lit,
                        CmpOp::Le => LinearLit::new(linear_lit.sum * -1, CmpOp::Ge),
                        CmpOp::Lt => LinearLit::new(linear_lit.sum * -1 + (-1), CmpOp::Ge),
                        CmpOp::Gt => LinearLit::new(linear_lit.sum + (-1), CmpOp::Ge),
                    };
                    simplified_linears.push(log::decompose_linear_lit_log(env, &normalized));
                }

                #[cfg(not(feature = "csp-extra-constraints"))]
                {
                    panic!("bug: log encoding is not enabled but is suggested");
                }
            }
        }
    }

    if simplified_linears.is_empty() {
        env.sat.add_clause(&bool_lits);
        return;
    }

    // Vec<Lit>: a clause
    // ClauseSet: list clauses whose disjunction is equivalent to a linear literal
    // Vec<ClauseSet>: the above for each linear literal
    let mut encoded_lits: Vec<ClauseSet> = vec![];
    let maybe_order_encoding_native_applicable =
        simplified_linears.len() == 1 && bool_lits.is_empty();
    let mut is_order_encoding_native_applied = false;

    for linear_lits in simplified_linears {
        let mut encoded_conjunction: ClauseSet = ClauseSet::new();
        for linear_lit in linear_lits {
            match suggest_encoder(env, &linear_lit) {
                EncoderKind::MixedGe => {
                    if maybe_order_encoding_native_applicable
                        && order::is_ge_order_encoding_native_applicable(env, &linear_lit.sum)
                    {
                        is_order_encoding_native_applied = true;
                        order::encode_linear_ge_order_encoding_native(env, &linear_lit.sum);
                    } else {
                        let encoded = mixed::encode_linear_ge_mixed(env, &linear_lit.sum);
                        encoded_conjunction.append(encoded);
                    }
                }
                EncoderKind::DirectSimple => {
                    let encoded = direct::encode_simple_linear_direct_encoding(env, &linear_lit);
                    if let Some(encoded) = encoded {
                        encoded_conjunction.push(&encoded);
                    }
                }
                EncoderKind::DirectEqNe => {
                    assert!(linear_lit.op == CmpOp::Eq || linear_lit.op == CmpOp::Ne);
                    let encoded = if linear_lit.op == CmpOp::Eq {
                        direct::encode_linear_eq_direct(env, &linear_lit.sum)
                    } else {
                        direct::encode_linear_ne_direct(env, &linear_lit.sum)
                    };
                    encoded_conjunction.append(encoded);
                }
                #[cfg(feature = "csp-extra-constraints")]
                EncoderKind::Log => {
                    assert!(
                        linear_lit.op == CmpOp::Eq
                            || linear_lit.op == CmpOp::Ne
                            || linear_lit.op == CmpOp::Ge
                    );
                    let encoded = log::encode_linear_log(env, &linear_lit.sum, linear_lit.op);
                    encoded_conjunction.append(encoded);
                }
                #[cfg(not(feature = "csp-extra-constraints"))]
                EncoderKind::Log => {
                    panic!("feature not enabled");
                }
            }
        }

        if encoded_conjunction.len() == 0 {
            // This constraint always holds
            // We can safely return here even if `encode_linear_ge_order_encoding_native` is called,
            // because in this case simplified_linears.len() == 1 and bool_lits is empty.
            return;
        }
        if encoded_conjunction.len() == 1 {
            bool_lits.extend_from_slice(&encoded_conjunction[0]);
            continue;
        }
        encoded_lits.push(encoded_conjunction);
    }

    if is_order_encoding_native_applied {
        assert!(bool_lits.is_empty());
    }

    if encoded_lits.is_empty() {
        env.sat.add_clause(&bool_lits);
    } else if encoded_lits.len() == 1 {
        // TODO: a channeling literal may be needed if `bool_lits` contains too many literals
        let clauses = encoded_lits.remove(0);
        let mut buffer = vec![];
        for i in 0..clauses.len() {
            buffer.clear();
            buffer.extend_from_slice(&clauses[i]);
            buffer.extend_from_slice(&bool_lits);
            env.sat.add_clause(&buffer);
        }
    } else {
        let mut channeling_lits = vec![];
        if encoded_lits.len() == 2 && bool_lits.is_empty() {
            let v = new_var!(env.sat);
            channeling_lits.push(v.as_lit(false));
            channeling_lits.push(v.as_lit(true));
        } else {
            for _ in 0..encoded_lits.len() {
                let v = new_var!(env.sat);
                channeling_lits.push(v.as_lit(true));
                bool_lits.push(v.as_lit(false));
            }
            env.sat.add_clause(&bool_lits);
        }
        for (i, clauses) in encoded_lits.into_iter().enumerate() {
            let channeling_lit = channeling_lits[i];
            let mut buffer = vec![];
            for i in 0..clauses.len() {
                buffer.clear();
                buffer.extend_from_slice(&clauses[i]);
                buffer.push(channeling_lit);
                env.sat.add_clause(&buffer);
            }
        }
    }
}

enum EncoderKind {
    MixedGe,
    DirectSimple,
    DirectEqNe,
    Log,
}

fn suggest_encoder(env: &EncoderEnv, linear_lit: &LinearLit) -> EncoderKind {
    if linear_lit.sum.len() == 1
        && env.map.int_map[*linear_lit.sum.iter().next().unwrap().0]
            .as_ref()
            .unwrap()
            .is_direct_encoding()
    {
        return EncoderKind::DirectSimple;
    }
    let is_all_direct_encoded = linear_lit
        .sum
        .iter()
        .all(|(&v, _)| env.map.int_map[v].as_ref().unwrap().is_direct_encoding());
    if (linear_lit.op == CmpOp::Eq || linear_lit.op == CmpOp::Ne) && is_all_direct_encoded {
        return EncoderKind::DirectEqNe;
    }
    let is_all_order_or_direct = linear_lit.sum.iter().all(|(&v, _)| {
        env.map.int_map[v]
            .as_ref()
            .unwrap()
            .is_direct_or_order_encoding()
    });
    if is_all_order_or_direct {
        return EncoderKind::MixedGe;
    }
    let is_all_log = linear_lit
        .sum
        .iter()
        .all(|(&v, _)| env.map.int_map[v].as_ref().unwrap().log_encoding.is_some());
    if is_all_log {
        return EncoderKind::Log;
    }
    panic!("no encoder is applicable");
}

enum ExtendedLit {
    True,
    False,
    Lit(Lit),
}

enum LinearInfo<'a> {
    Order(order::LinearInfoForOrderEncoding<'a>),
    Direct(direct::LinearInfoForDirectEncoding<'a>),
}

/// Given a LinearLit `lit`, compute a set of LinearLit's whose conjunction is equivalent to `lit`.
/// If `None` is returned, it means that `lit` is not satisfiable.
///
/// This function may introduce auxiliary variables. These variables are added to `env`,
/// but any encoded clauses are not directly added to `env`.
/// It is ensured that just calling this function does not affect the satisfiability of the current SAT instance,
/// unless the returned literals are encoded and added to the SAT.
fn decompose_linear_lit(env: &mut EncoderEnv, lit: &LinearLit) -> Option<Vec<LinearLit>> {
    assert!(lit.op == CmpOp::Ge || lit.op == CmpOp::Eq || lit.op == CmpOp::Ne);
    let op_for_aux_lits = if lit.op == CmpOp::Ge {
        CmpOp::Ge
    } else {
        CmpOp::Eq
    };

    let mut heap = BinaryHeap::new();
    for (&var, &coef) in &lit.sum.term {
        let encoding = env.map.int_map[var].as_ref().unwrap();
        let dom_size = if let Some(order_encoding) = &encoding.order_encoding {
            order_encoding.domain.len()
        } else if let Some(direct_encoding) = &encoding.direct_encoding {
            direct_encoding.domain.len()
        } else {
            panic!();
        };
        heap.push(Reverse((dom_size, var, coef)));
    }

    let mut ret = vec![];

    let mut pending: Vec<(usize, IntVar, CheckedInt)> = vec![];
    let mut dom_product = 1usize;
    while let Some(&Reverse(top)) = heap.peek() {
        let (dom_size, _, _) = top;
        if dom_product * dom_size >= env.config.domain_product_threshold
            && pending.len() >= 2
            && heap.len() >= 2
        {
            // Introduce auxiliary variable which aggregates current pending terms
            let mut aux_sum = LinearSum::new();
            for &(_, var, coef) in &pending {
                aux_sum.add_coef(var, coef);
            }
            let mut aux_dom = env.norm_vars.get_domain_linear_sum(&aux_sum);

            let mut rem_sum = LinearSum::new();
            for &Reverse((_, var, coef)) in &heap {
                rem_sum.add_coef(var, coef);
            }
            let rem_dom = env.norm_vars.get_domain_linear_sum(&rem_sum);
            if lit.op == CmpOp::Eq {
                aux_dom.refine_upper_bound(-(lit.sum.constant + rem_dom.lower_bound_checked()));
            }
            if lit.op == CmpOp::Eq || lit.op == CmpOp::Ge {
                aux_dom.refine_lower_bound(-(lit.sum.constant + rem_dom.upper_bound_checked()));
            }

            if aux_dom.is_empty() {
                return None;
            }

            let aux_var = env
                .norm_vars
                .new_int_var(IntVarRepresentation::Domain(aux_dom));
            env.map
                .convert_int_var_order_encoding(env.norm_vars, env.sat, aux_var);

            // aux_sum >= aux_var
            aux_sum.add_coef(aux_var, CheckedInt::new(-1));
            ret.push(LinearLit::new(aux_sum, op_for_aux_lits));

            pending.clear();
            let dom_size = env.map.int_map[aux_var]
                .as_ref()
                .unwrap()
                .as_order_encoding()
                .domain
                .len();
            heap.push(Reverse((dom_size, aux_var, CheckedInt::new(1))));
            dom_product = 1;
            continue;
        }
        dom_product *= dom_size;
        pending.push(top);
        heap.pop();
    }

    let mut sum = LinearSum::constant(lit.sum.constant);
    for &(_, var, coef) in &pending {
        sum.add_coef(var, coef);
    }
    ret.push(LinearLit::new(sum, lit.op));
    Some(ret)
}

#[cfg(feature = "csp-extra-constraints")]
fn encode_mul_naive(env: &mut EncoderEnv, x: IntVar, y: IntVar, m: IntVar) {
    let x_range = env.map.int_map[x].as_ref().unwrap().range();
    let y_range = env.map.int_map[y].as_ref().unwrap().range();

    for i in x_range.low.get()..=x_range.high.get() {
        let i = CheckedInt::new(i);
        for j in y_range.low.get()..=y_range.high.get() {
            let j = CheckedInt::new(j);

            let mut c = Constraint::new();
            c.add_linear(LinearLit::new(
                LinearSum::singleton(x) - LinearSum::constant(i),
                CmpOp::Ne,
            ));
            c.add_linear(LinearLit::new(
                LinearSum::singleton(y) - LinearSum::constant(j),
                CmpOp::Ne,
            ));
            c.add_linear(LinearLit::new(
                LinearSum::singleton(m) - LinearSum::constant(i * j),
                CmpOp::Eq,
            ));

            encode_constraint(env, c);
        }
    }
}

// TODO: add tests for ClauseSet
#[cfg(test)]
mod tests {
    use super::super::{
        config::Config, domain::Domain, norm_csp::IntVarRepresentation, norm_csp::NormCSPVars,
        sat::SAT,
    };
    use super::*;

    pub(super) struct EncoderTester {
        norm_vars: NormCSPVars,
        sat: SAT,
        map: EncodeMap,
        pub config: Config,
    }

    impl EncoderTester {
        pub fn new() -> EncoderTester {
            EncoderTester {
                norm_vars: NormCSPVars::new(),
                sat: SAT::new(),
                map: EncodeMap::new(),
                config: Config::default(),
            }
        }

        pub fn env(&mut self) -> EncoderEnv {
            EncoderEnv {
                norm_vars: &mut self.norm_vars,
                sat: &mut self.sat,
                map: &mut self.map,
                config: &self.config,
            }
        }

        pub fn add_clause(&mut self, clause: &[Lit]) {
            self.sat.add_clause(clause);
        }

        pub fn add_clause_set(&mut self, clause_set: ClauseSet) {
            for i in 0..clause_set.len() {
                self.sat.add_clause(&clause_set[i]);
            }
        }

        pub fn add_int_var(&mut self, domain: Domain, is_direct_encoding: bool) -> IntVar {
            let v = self
                .norm_vars
                .new_int_var(IntVarRepresentation::Domain(domain));

            if is_direct_encoding {
                self.map
                    .convert_int_var_direct_encoding(&self.norm_vars, &mut self.sat, v);
            } else {
                self.map
                    .convert_int_var_order_encoding(&self.norm_vars, &mut self.sat, v);
            }

            v
        }

        #[allow(unused)]
        pub fn add_int_var_log_encoding(&mut self, domain: Domain) -> IntVar {
            let v = self
                .norm_vars
                .new_int_var(IntVarRepresentation::Domain(domain));

            self.map
                .convert_int_var_log_encoding(&self.norm_vars, &mut self.sat, v);

            v
        }

        pub fn enumerate_valid_assignments_by_sat(
            &mut self,
            int_vars: &[IntVar],
        ) -> Vec<Vec<CheckedInt>> {
            let mut sat_vars_set = std::collections::HashSet::new();
            for var in int_vars {
                for lit in self.map.int_map[*var].as_ref().unwrap().repr_literals() {
                    sat_vars_set.insert(lit.var());
                }
            }
            let sat_vars = sat_vars_set.into_iter().collect::<Vec<_>>();

            let sat = &mut self.sat;
            let map = &self.map;

            let mut ret = vec![];
            while let Some(model) = sat.solve() {
                let values = int_vars
                    .iter()
                    .map(|&v| map.get_int_value_checked(&model, v).unwrap())
                    .collect::<Vec<_>>();
                ret.push(values);

                let refutation_clause = sat_vars
                    .iter()
                    .map(|&v| v.as_lit(model.assignment(v)))
                    .collect::<Vec<_>>();
                sat.add_clause(&refutation_clause);
            }

            ret
        }

        pub fn enumerate_valid_assignments_by_literals(
            &self,
            lits: &[LinearLit],
            mul: &[(IntVar, IntVar, IntVar)],
            int_vars: &[IntVar],
        ) -> Vec<Vec<CheckedInt>> {
            let domains = int_vars
                .iter()
                .map(|&v| self.norm_vars.int_var(v).enumerate())
                .collect::<Vec<_>>();

            let all_assignments = crate::test_util::product_multi(&domains);
            let valid_assignments = all_assignments
                .into_iter()
                .filter(|assignment| {
                    for lit in lits {
                        let sum = &lit.sum;
                        let mut value = sum.constant;
                        for (&var, &coef) in sum.iter() {
                            let idx = int_vars.iter().position(|&v| v == var).unwrap();
                            value += assignment[idx] * coef;
                        }
                        if !lit.op.compare(value, CheckedInt::new(0)) {
                            return false;
                        }
                    }
                    for &(x, y, m) in mul {
                        let xi = int_vars.iter().position(|&v| v == x).unwrap();
                        let yi = int_vars.iter().position(|&v| v == y).unwrap();
                        let mi = int_vars.iter().position(|&v| v == m).unwrap();
                        if assignment[xi] * assignment[yi] != assignment[mi] {
                            return false;
                        }
                    }
                    true
                })
                .collect();
            valid_assignments
        }

        pub fn run_check(self, lits: &[LinearLit]) {
            self.run_check_with_mul(lits, &[]);
        }

        #[allow(unused)]
        pub fn run_check_with_mul(mut self, lits: &[LinearLit], mul: &[(IntVar, IntVar, IntVar)]) {
            let mut related_vars_set = std::collections::HashSet::new();
            for lit in lits {
                for (v, _) in lit.sum.iter() {
                    related_vars_set.insert(*v);
                }
            }
            for (x, y, m) in mul {
                related_vars_set.insert(*x);
                related_vars_set.insert(*y);
                related_vars_set.insert(*m);
            }
            let related_vars = related_vars_set.into_iter().collect::<Vec<_>>();

            let mut result_by_literals =
                self.enumerate_valid_assignments_by_literals(lits, mul, &related_vars);
            result_by_literals.sort();
            let mut result_by_sat = self.enumerate_valid_assignments_by_sat(&related_vars);
            result_by_sat.sort();

            assert_eq!(result_by_literals, result_by_sat);
        }
    }

    pub(super) fn linear_sum(terms: &[(IntVar, i32)], constant: i32) -> LinearSum {
        let mut ret = LinearSum::constant(CheckedInt::new(constant));
        for &(var, coef) in terms {
            ret.add_coef(var, CheckedInt::new(coef));
        }
        ret
    }

    #[test]
    fn test_encode_large_literals() {
        for c in [-1, -3, 5] {
            for op in [CmpOp::Eq, CmpOp::Ne, CmpOp::Ge] {
                let mut tester = EncoderTester::new();

                let mut terms = vec![];
                for _ in 0..12 {
                    let var = tester.add_int_var(Domain::range(0, 1), false);
                    terms.push((var, 1));
                }

                let lits = vec![LinearLit::new(linear_sum(&terms, c), op)];
                let env = &mut tester.env();
                let is_unsat = is_unsatisfiable_linear(env, &lits[0]);

                encode_constraint(
                    env,
                    Constraint {
                        bool_lit: vec![],
                        linear_lit: lits.clone(),
                    },
                );

                // If the literal is unsatisfiable, the encoding does not take place
                if !is_unsat {
                    // Ensure that auxiliary variables are created, or this test is meaningless
                    assert!(tester.norm_vars.int_vars_iter().count() > terms.len());
                }

                tester.run_check(&lits);
            }
        }
    }

    #[test]
    fn test_decompose_linear_lit() {
        for c in [-1, -3, 5] {
            for op in [CmpOp::Eq, CmpOp::Ne, CmpOp::Ge] {
                let mut tester = EncoderTester::new();

                let mut terms = vec![];
                for _ in 0..12 {
                    let var = tester.add_int_var(Domain::range(0, 1), false);
                    terms.push((var, 1));
                }

                let lit = LinearLit::new(linear_sum(&terms, c), op);
                let decomposed = decompose_linear_lit(&mut tester.env(), &lit);

                let lit_vars = lit.sum.iter().map(|(v, _)| *v).collect::<Vec<_>>();
                let lit_domains = lit_vars
                    .iter()
                    .map(|&v| tester.norm_vars.int_var(v).enumerate())
                    .collect::<Vec<_>>();

                let mut other_vars = vec![];
                if let Some(decomposed) = &decomposed {
                    for lit in decomposed {
                        let vars = lit
                            .sum
                            .iter()
                            .map(|(v, _)| *v)
                            .filter(|&v| !lit_vars.contains(&v))
                            .collect::<Vec<_>>();
                        other_vars.extend(vars);
                    }
                }

                let is_unsat = is_unsatisfiable_linear(&tester.env(), &lit);
                if !is_unsat {
                    assert!(other_vars.len() > 0);
                }

                let other_vars_domains = other_vars
                    .iter()
                    .map(|&v| tester.norm_vars.int_var(v).enumerate())
                    .collect::<Vec<_>>();

                let assignments = crate::test_util::product_multi(&lit_domains);
                let other_assignments = crate::test_util::product_multi(&other_vars_domains);

                let is_satisfiable =
                    |lit: &LinearLit,
                     assignment: &[CheckedInt],
                     other_assignment: &[CheckedInt]| {
                        let mut value = lit.sum.constant;
                        for (&var, &coef) in lit.sum.iter() {
                            if let Some(idx) = lit_vars.iter().position(|&v| v == var) {
                                value += assignment[idx] * coef;
                            } else {
                                value += other_assignment
                                    [other_vars.iter().position(|&v| v == var).unwrap()]
                                    * coef;
                            }
                        }
                        lit.op.compare(value, CheckedInt::new(0))
                    };

                for a in &assignments {
                    let is_sat = is_satisfiable(&lit, a, &[]);

                    if decomposed.is_none() {
                        assert!(!is_sat);
                        continue;
                    }

                    let decomposed = decomposed.as_ref().unwrap();
                    let mut is_sat_decomposed_any = false;
                    for b in &other_assignments {
                        is_sat_decomposed_any |=
                            decomposed.iter().all(|lit| is_satisfiable(lit, a, b));
                    }

                    assert_eq!(is_sat, is_sat_decomposed_any);
                }
            }
        }
    }
}
