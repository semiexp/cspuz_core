use super::config::Config;
use super::csp::{BoolExpr, BoolVar, BoolVarStatus, CSPVars, IntExpr, IntVar, Stmt, CSP};
use super::norm_csp::BoolLit as NBoolLit;
use super::norm_csp::IntVar as NIntVar;
use super::norm_csp::{Constraint, ExtraConstraint, LinearLit, LinearSum, NormCSP};
use crate::arithmetic::{CheckedInt, CmpOp};
use crate::domain::Domain;
use crate::norm_csp::IntVarRepresentation;
use crate::util::ConvertMap;

#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
fn new_hash_map<K, V>() -> HashMap<K, V> {
    HashMap::new()
}

#[cfg(target_arch = "wasm32")]
mod deterministic_hash_map {
    pub struct DetState;

    impl std::hash::BuildHasher for DetState {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    pub fn new_hash_map<K, V>() -> std::collections::HashMap<K, V, DetState> {
        std::collections::HashMap::with_hasher(DetState)
    }
}

#[cfg(target_arch = "wasm32")]
type HashMap<K, V> = std::collections::HashMap<K, V, deterministic_hash_map::DetState>;

#[cfg(target_arch = "wasm32")]
use deterministic_hash_map::new_hash_map;

#[derive(Clone, Copy, Default)]
pub(crate) enum ConvertedBoolVar {
    Lit(NBoolLit),
    Removed, // Variable is removed during constant folding

    #[default]
    NotConverted,
}

impl ConvertedBoolVar {
    #[allow(unused)]
    fn is_removed(&self) -> bool {
        matches!(self, ConvertedBoolVar::Removed)
    }

    #[allow(unused)]
    fn is_lit(&self) -> bool {
        matches!(self, ConvertedBoolVar::Lit(_))
    }

    fn is_not_converted(&self) -> bool {
        matches!(self, ConvertedBoolVar::NotConverted)
    }
}

pub struct NormalizeMap {
    bool_map: ConvertMap<BoolVar, ConvertedBoolVar>,
    int_map: ConvertMap<IntVar, Option<NIntVar>>,
    int_expr_equivalence: HashMap<IntExpr, NIntVar>,
}

impl NormalizeMap {
    pub fn new() -> NormalizeMap {
        NormalizeMap {
            bool_map: ConvertMap::new(),
            int_map: ConvertMap::new(),
            int_expr_equivalence: new_hash_map(),
        }
    }

    fn convert_bool_var(
        &mut self,
        _csp_vars: &CSPVars,
        norm: &mut NormCSP,
        var: BoolVar,
    ) -> NBoolLit {
        match self.bool_map[var] {
            ConvertedBoolVar::Lit(x) => x,
            ConvertedBoolVar::Removed => panic!(),
            ConvertedBoolVar::NotConverted => {
                let ret = NBoolLit::new(norm.new_bool_var(), false);
                self.bool_map[var] = ConvertedBoolVar::Lit(ret);
                ret
            }
        }
    }

    fn mark_removed(&mut self, var: BoolVar) {
        if let ConvertedBoolVar::Lit(_) = self.bool_map[var] {
            panic!();
        }

        self.bool_map[var] = ConvertedBoolVar::Removed;
    }

    fn convert_int_var(&mut self, csp_vars: &CSPVars, norm: &mut NormCSP, var: IntVar) -> NIntVar {
        match self.int_map[var] {
            Some(x) => x,
            None => {
                let var_desc = csp_vars.int_var(var);
                let ret = norm.new_int_var(var_desc.domain.clone());
                self.int_map[var] = Some(ret);
                ret
            }
        }
    }

    pub fn get_bool_var(&self, var: BoolVar) -> Option<NBoolLit> {
        match self.bool_map[var] {
            ConvertedBoolVar::Lit(x) => Some(x),
            ConvertedBoolVar::Removed => panic!(),
            ConvertedBoolVar::NotConverted => None,
        }
    }

    pub(crate) fn get_bool_var_raw(&self, var: BoolVar) -> ConvertedBoolVar {
        self.bool_map[var]
    }

    pub fn get_int_var(&self, var: IntVar) -> Option<NIntVar> {
        self.int_map[var]
    }
}

struct NormalizerEnv<'a, 'b, 'c, 'd> {
    csp_vars: &'a mut CSPVars,
    norm: &'b mut NormCSP,
    map: &'c mut NormalizeMap,
    config: &'d Config,
}

impl NormalizerEnv<'_, '_, '_, '_> {
    fn convert_bool_var(&mut self, var: BoolVar) -> NBoolLit {
        self.map.convert_bool_var(self.csp_vars, self.norm, var)
    }

    fn convert_int_var(&mut self, var: IntVar) -> NIntVar {
        self.map.convert_int_var(self.csp_vars, self.norm, var)
    }
}

/// Normalize constraints in `csp`. Existing constraints in `csp` are cleared.
pub fn normalize(csp: &mut CSP, norm: &mut NormCSP, map: &mut NormalizeMap, config: &Config) {
    let mut env = NormalizerEnv {
        csp_vars: &mut csp.vars,
        norm,
        map,
        config,
    };

    if config.merge_equivalent_variables {
        for constr in &csp.constraints {
            if let Stmt::Expr(e) = constr {
                if let BoolExpr::Iff(x, y) = e {
                    if let (Some(x), Some(y)) = (x.as_var(), y.as_var()) {
                        match (env.map.get_bool_var(x), env.map.get_bool_var(y)) {
                            (Some(_), Some(_)) => (),
                            (Some(xl), None) => {
                                assert!(env.map.bool_map[y].is_not_converted());
                                env.map.bool_map[y] = ConvertedBoolVar::Lit(xl);
                            }
                            (None, Some(yl)) => {
                                assert!(env.map.bool_map[x].is_not_converted());
                                env.map.bool_map[x] = ConvertedBoolVar::Lit(yl);
                            }
                            (None, None) => {
                                let xl = env.convert_bool_var(x);
                                assert!(env.map.bool_map[y].is_not_converted());
                                env.map.bool_map[y] = ConvertedBoolVar::Lit(xl);
                            }
                        }
                    }
                } else if let BoolExpr::Xor(x, y) = e {
                    if let (Some(x), Some(y)) = (x.as_var(), y.as_var()) {
                        match (env.map.get_bool_var(x), env.map.get_bool_var(y)) {
                            (Some(_), Some(_)) => (),
                            (Some(xl), None) => {
                                assert!(env.map.bool_map[y].is_not_converted());
                                env.map.bool_map[y] = ConvertedBoolVar::Lit(!xl);
                            }
                            (None, Some(yl)) => {
                                assert!(env.map.bool_map[x].is_not_converted());
                                env.map.bool_map[x] = ConvertedBoolVar::Lit(!yl);
                            }
                            (None, None) => {
                                let xl = env.convert_bool_var(x);
                                env.map.bool_map[y] = ConvertedBoolVar::Lit(!xl);
                            }
                        }
                    }
                }
            }
        }
    }

    for var in env.csp_vars.bool_vars_iter() {
        let data = &env.csp_vars[var];
        if let BoolVarStatus::Fixed(_) = data.get_status() {
            env.map.mark_removed(var);
        }
    }

    for &var in &csp.prenormalize_vars {
        let var = env.convert_bool_var(var).var;
        env.norm.add_prenormalize_var(var);
    }

    let mut stmts = vec![];
    std::mem::swap(&mut stmts, &mut csp.constraints);

    for stmt in stmts {
        normalize_stmt(&mut env, stmt);
    }
}

fn equivalent_bool_lit(env: &mut NormalizerEnv, expr: BoolExpr) -> NBoolLit {
    let simplified = match &expr {
        BoolExpr::Var(v) => Some(env.convert_bool_var(*v)),
        BoolExpr::Not(e) => match e.as_ref() {
            BoolExpr::Var(v) => Some(!env.convert_bool_var(*v)),
            _ => None,
        },
        _ => None,
    };
    if let Some(l) = simplified {
        l
    } else {
        let aux = env.norm.new_bool_var();
        normalize_and_register_expr(env, BoolExpr::NVar(aux).iff(expr));
        NBoolLit::new(aux, false)
    }
}

fn equivalent_int_var(env: &mut NormalizerEnv, expr: &IntExpr) -> NIntVar {
    match expr {
        IntExpr::Var(v) => env.convert_int_var(*v),
        IntExpr::NVar(v) => *v,
        _ => {
            let x = normalize_int_expr(env, expr);
            let dom = env.norm.get_domain_linear_sum(&x);
            let xvar = env.norm.new_int_var(dom);
            {
                let mut c = Constraint::new();
                c.add_linear(LinearLit::new(x - LinearSum::singleton(xvar), CmpOp::Eq));
                env.norm.add_constraint(c);
            }
            xvar
        }
    }
}

fn normalize_stmt(env: &mut NormalizerEnv, stmt: Stmt) {
    if env.config.verbose {
        let mut buf = Vec::<u8>::new();
        stmt.pretty_print(&mut buf).unwrap();
        eprintln!("{}", String::from_utf8(buf).unwrap());
    }

    let num_constrs_before_norm = env.norm.constraints.len();

    match stmt {
        Stmt::Expr(expr) => normalize_and_register_expr(env, expr),
        Stmt::AllDifferent(_exprs) => {
            for i in 0.._exprs.len() {
                for j in (i + 1).._exprs.len() {
                    let diff_expr = _exprs[i].clone().ne(_exprs[j].clone());
                    normalize_and_register_expr(env, diff_expr);
                }
            }
            let is_all_var = _exprs.iter().all(|e| matches!(e, IntExpr::Var(_)));
            if env.config.alldifferent_bijection_constraints && is_all_var && !_exprs.is_empty() {
                let mut domain: Option<Vec<CheckedInt>> = None;
                let mut isok = true;
                for e in &_exprs {
                    if let IntExpr::Var(v) = e {
                        let var_data = env.csp_vars.int_var(*v);
                        let d = var_data.domain.enumerate();
                        if let Some(domain) = &domain {
                            if domain != &d {
                                isok = false;
                                break;
                            }
                        } else {
                            domain = Some(d);
                        }
                    }
                }
                if isok {
                    let domain = domain.unwrap();
                    if domain.len() == _exprs.len() {
                        for &value in &domain {
                            let e = _exprs
                                .iter()
                                .map(|e| {
                                    Box::new(BoolExpr::Cmp(
                                        CmpOp::Eq,
                                        Box::new(e.clone()),
                                        Box::new(IntExpr::Const(value.get())),
                                    ))
                                })
                                .collect::<Vec<_>>();
                            normalize_and_register_expr(env, BoolExpr::Or(e));
                        }
                    }
                }
            }
        }
        Stmt::ActiveVerticesConnected(vertices, edges) => {
            let vertices_converted = vertices
                .into_iter()
                .map(|e| equivalent_bool_lit(env, e))
                .collect::<Vec<_>>();
            env.norm
                .add_extra_constraint(ExtraConstraint::ActiveVerticesConnected(
                    vertices_converted,
                    edges,
                ));
        }
        Stmt::Circuit(exprs) => {
            let exprs_converted = exprs
                .into_iter()
                .map(|e| equivalent_int_var(env, &e))
                .collect::<Vec<_>>();
            normalize_circuit(env, exprs_converted)
        }
        Stmt::ExtensionSupports(exprs, supports) => {
            let exprs_converted = exprs
                .into_iter()
                .map(|e| equivalent_int_var(env, &e))
                .collect::<Vec<_>>();
            normalize_extension_supports(env, exprs_converted, supports)
        }
        Stmt::GraphDivision(sizes, edges, edge_lits, opts) => {
            let sizes = sizes
                .into_iter()
                .map(|e| e.map(|e| equivalent_int_var(env, &e)))
                .collect::<Vec<_>>();

            let edge_lits_converted = edge_lits
                .into_iter()
                .map(|e| equivalent_bool_lit(env, e))
                .collect::<Vec<_>>();
            env.norm
                .add_extra_constraint(ExtraConstraint::GraphDivision(
                    sizes,
                    edges,
                    edge_lits_converted,
                    opts,
                ));
        }
        Stmt::CustomConstraint(inputs, constr) => {
            let inputs_as_lit = inputs
                .into_iter()
                .map(|e| equivalent_bool_lit(env, e))
                .collect::<Vec<_>>();
            env.norm
                .add_extra_constraint(ExtraConstraint::CustomConstraint(inputs_as_lit, constr));
        }
    }
    if env.config.verbose {
        for i in num_constrs_before_norm..env.norm.constraints.len() {
            let mut buf = Vec::<u8>::new();
            env.norm.constraints[i].pretty_print(&mut buf).unwrap();
            eprintln!("{}", String::from_utf8(buf).unwrap());
        }
    }
}

fn normalize_and_register_expr(env: &mut NormalizerEnv, mut expr: BoolExpr) {
    let mut exprs = vec![];
    tseitin_transformation_bool(env, &mut exprs, &mut expr, false);
    exprs.push(expr);
    for expr in exprs {
        let constraints = normalize_bool_expr(env, &expr, false);
        for c in constraints {
            env.norm.add_constraint(c);
        }
    }
}

/// Apply Tseitin transformation for `expr` to avoid the exponential explosion of constraints caused by Iff/Xor.
fn tseitin_transformation_bool(
    env: &mut NormalizerEnv,
    extra: &mut Vec<BoolExpr>,
    expr: &mut BoolExpr,
    transform: bool,
) {
    match expr {
        BoolExpr::Const(_) | BoolExpr::Var(_) | BoolExpr::NVar(_) => (),
        BoolExpr::And(es) | BoolExpr::Or(es) => {
            for e in es {
                tseitin_transformation_bool(env, extra, e, transform);
            }
        }
        BoolExpr::Xor(e1, e2) | BoolExpr::Iff(e1, e2) => {
            if transform {
                let v1 = env.norm.new_bool_var();
                let v2 = env.norm.new_bool_var();

                let mut f1 = BoolExpr::NVar(v1);
                std::mem::swap(e1.as_mut(), &mut f1);
                let mut f2 = BoolExpr::NVar(v2);
                std::mem::swap(e2.as_mut(), &mut f2);

                // TODO: use cache
                tseitin_transformation_bool(env, extra, &mut f1, true);
                extra.push(BoolExpr::Iff(Box::new(f1), Box::new(BoolExpr::NVar(v1))));

                tseitin_transformation_bool(env, extra, &mut f2, true);
                extra.push(BoolExpr::Iff(Box::new(f2), Box::new(BoolExpr::NVar(v2))));
            } else {
                tseitin_transformation_bool(env, extra, e1, true);
                tseitin_transformation_bool(env, extra, e2, true);
            }
        }
        BoolExpr::Not(e) => tseitin_transformation_bool(env, extra, e, transform),
        BoolExpr::Imp(e1, e2) => {
            tseitin_transformation_bool(env, extra, e1, transform);
            tseitin_transformation_bool(env, extra, e2, transform);
        }
        BoolExpr::Cmp(_, e1, e2) => {
            tseitin_transformation_int(env, extra, e1);
            tseitin_transformation_int(env, extra, e2);
        }
    }
}

fn tseitin_transformation_int(
    env: &mut NormalizerEnv,
    extra: &mut Vec<BoolExpr>,
    expr: &mut IntExpr,
) {
    match expr {
        IntExpr::Const(_) | IntExpr::Var(_) | IntExpr::NVar(_) => (),
        IntExpr::Linear(terms) => terms
            .iter_mut()
            .for_each(|term| tseitin_transformation_int(env, extra, &mut term.0)),
        IntExpr::If(c, t, f) => {
            tseitin_transformation_bool(env, extra, c, true);
            tseitin_transformation_int(env, extra, t);
            tseitin_transformation_int(env, extra, f);
        }
        IntExpr::Abs(x) => tseitin_transformation_int(env, extra, x),
        IntExpr::Mul(x, y) => {
            tseitin_transformation_int(env, extra, x);
            tseitin_transformation_int(env, extra, y);
        }
    }
}

/// Normalize `expr` into a set of `Constraint`s. If `neg` is `true`, not(`expr`) is normalized instead.
fn normalize_bool_expr(env: &mut NormalizerEnv, expr: &BoolExpr, neg: bool) -> Vec<Constraint> {
    match (expr, neg) {
        (&BoolExpr::Const(b), neg) => {
            if b ^ neg {
                vec![]
            } else {
                vec![Constraint::new()]
            }
        }
        (&BoolExpr::Var(v), neg) => {
            let nv = env.convert_bool_var(v);
            let mut constraint = Constraint::new();
            constraint.add_bool(nv.negate_if(neg));
            vec![constraint]
        }
        (&BoolExpr::NVar(v), neg) => {
            let mut constraint = Constraint::new();
            constraint.add_bool(NBoolLit::new(v, neg));
            vec![constraint]
        }
        (BoolExpr::And(es), false) | (BoolExpr::Or(es), true) => normalize_conjunction(
            es.iter()
                .map(|e| normalize_bool_expr(env, e, neg))
                .collect(),
        ),
        (BoolExpr::And(es), true) | (BoolExpr::Or(es), false) => {
            let constrs = es
                .iter()
                .map(|e| normalize_bool_expr(env, e, neg))
                .collect();
            normalize_disjunction(env, constrs)
        }
        (BoolExpr::Not(e), neg) => normalize_bool_expr(env, e, !neg),
        (BoolExpr::Xor(e1, e2), false) | (BoolExpr::Iff(e1, e2), true) => {
            let sub1 = vec![
                normalize_bool_expr(env, e1, false),
                normalize_bool_expr(env, e2, false),
            ];
            let sub2 = vec![
                normalize_bool_expr(env, e1, true),
                normalize_bool_expr(env, e2, true),
            ];
            normalize_conjunction(vec![
                normalize_disjunction(env, sub1),
                normalize_disjunction(env, sub2),
            ])
        }
        (BoolExpr::Xor(e1, e2), true) | (BoolExpr::Iff(e1, e2), false) => {
            let sub1 = vec![
                normalize_bool_expr(env, e1, false),
                normalize_bool_expr(env, e2, true),
            ];
            let sub2 = vec![
                normalize_bool_expr(env, e1, true),
                normalize_bool_expr(env, e2, false),
            ];
            normalize_conjunction(vec![
                normalize_disjunction(env, sub1),
                normalize_disjunction(env, sub2),
            ])
        }
        (BoolExpr::Imp(e1, e2), false) => {
            let constrs = vec![
                normalize_bool_expr(env, e1, true),
                normalize_bool_expr(env, e2, false),
            ];
            normalize_disjunction(env, constrs)
        }
        (BoolExpr::Imp(e1, e2), true) => {
            let constrs = vec![
                normalize_bool_expr(env, e1, false),
                normalize_bool_expr(env, e2, true),
            ];
            normalize_conjunction(constrs)
        }
        (BoolExpr::Cmp(op, e1, e2), _) => {
            let op = if neg { op.negate() } else { *op };

            let v1 = normalize_int_expr(env, e1);
            let v2 = normalize_int_expr(env, e2);

            let mut constraint = Constraint::new();
            constraint.add_linear(LinearLit::new(v1 - v2, op));
            vec![constraint]
        }
    }
}

fn normalize_conjunction(constrs: Vec<Vec<Constraint>>) -> Vec<Constraint> {
    let mut ret = vec![];
    for constr in constrs {
        ret.extend(constr);
    }
    ret
}

fn normalize_disjunction(
    env: &mut NormalizerEnv,
    constrs: Vec<Vec<Constraint>>,
) -> Vec<Constraint> {
    let mut constrs = constrs;
    if constrs.is_empty() {
        vec![]
    } else if constrs.len() == 1 {
        constrs.remove(0)
    } else {
        let mut ret = vec![];
        let mut aux = Constraint::new();

        if constrs.iter().any(|constr| constr.is_empty()) {
            return vec![];
        }

        let mut complex_constr = vec![];
        for mut constr in constrs {
            if constr.is_empty() {
                unreachable!();
            } else if constr.len() == 1 {
                let c = constr.remove(0);
                aux.bool_lit.extend(c.bool_lit);
                aux.linear_lit.extend(c.linear_lit);
            } else {
                complex_constr.push(constr);
            }
        }
        if complex_constr.len() == 2 && aux.bool_lit.is_empty() && aux.linear_lit.is_empty() {
            let v = env.norm.new_bool_var();
            for (i, constr) in complex_constr.into_iter().enumerate() {
                for mut con in constr {
                    con.add_bool(NBoolLit::new(v, i == 0));
                    ret.push(con);
                }
            }
            return ret;
        }
        if complex_constr.len() == 1 && aux.bool_lit.len() <= 1 && aux.linear_lit.is_empty() {
            for constr in complex_constr {
                for mut con in constr {
                    for &lit in &aux.bool_lit {
                        con.add_bool(lit);
                    }
                    ret.push(con);
                }
            }
            return ret;
        }
        for constr in complex_constr {
            let v = env.norm.new_bool_var();
            aux.add_bool(NBoolLit::new(v, false));
            for mut con in constr {
                con.add_bool(NBoolLit::new(v, true));
                ret.push(con);
            }
        }

        ret.push(aux);
        ret
    }
}

fn normalize_int_expr(env: &mut NormalizerEnv, expr: &IntExpr) -> LinearSum {
    match expr {
        &IntExpr::Const(c) => LinearSum::constant(CheckedInt::new(c)),
        &IntExpr::Var(v) => {
            let nv = env.convert_int_var(v);
            LinearSum::singleton(nv)
        }
        &IntExpr::NVar(v) => LinearSum::singleton(v),
        IntExpr::Linear(es) => {
            let mut ret = LinearSum::new();
            for (e, coef) in es {
                ret += normalize_int_expr(env, e) * CheckedInt::new(*coef);
            }
            ret
        }
        IntExpr::If(c, t, f) => {
            if let Some(&v) = env.map.int_expr_equivalence.get(expr) {
                return LinearSum::singleton(v);
            }

            let t = normalize_int_expr(env, t);
            let f = normalize_int_expr(env, f);

            if t.is_constant() && f.is_constant() {
                let val_true = t.constant;
                let val_false = f.constant;
                match *c.as_ref() {
                    BoolExpr::Var(c) => {
                        let c = env.convert_bool_var(c);
                        let v = env.norm.new_binary_int_var(c, val_true, val_false);
                        return LinearSum::singleton(v);
                    }
                    BoolExpr::NVar(c) => {
                        let v = env.norm.new_binary_int_var(
                            NBoolLit::new(c, false),
                            val_true,
                            val_false,
                        );
                        return LinearSum::singleton(v);
                    }
                    _ => {
                        let b = env.norm.new_bool_var();
                        let constr =
                            normalize_bool_expr(env, &BoolExpr::NVar(b).iff(*c.clone()), false);
                        for c in constr {
                            env.norm.add_constraint(c);
                        }

                        let v = env.norm.new_binary_int_var(
                            NBoolLit::new(b, false),
                            val_true,
                            val_false,
                        );
                        return LinearSum::singleton(v);
                    }
                }
            }

            let dom = env.norm.get_domain_linear_sum(&t) | env.norm.get_domain_linear_sum(&f);
            let v = env.norm.new_int_var(dom);

            let mut constr1 = normalize_bool_expr(env, c, false);
            {
                let mut c = Constraint::new();
                c.add_linear(LinearLit::new(t - LinearSum::singleton(v), CmpOp::Eq));
                constr1.push(c);
            }

            let mut constr2 = normalize_bool_expr(env, c, true);
            {
                let mut c = Constraint::new();
                c.add_linear(LinearLit::new(f - LinearSum::singleton(v), CmpOp::Eq));
                constr2.push(c);
            }

            for c in normalize_disjunction(env, vec![constr1, constr2]) {
                env.norm.add_constraint(c);
            }

            assert!(env
                .map
                .int_expr_equivalence
                .insert(expr.clone(), v)
                .is_none());

            LinearSum::singleton(v)
        }
        IntExpr::Abs(x) => {
            let xvar = equivalent_int_var(env, x);
            let aux_expr = IntExpr::If(
                Box::new(IntExpr::NVar(xvar).ge(IntExpr::Const(0))),
                Box::new(IntExpr::NVar(xvar)),
                Box::new(IntExpr::NVar(xvar) * -1),
            );
            normalize_int_expr(env, &aux_expr)
        }
        IntExpr::Mul(x, y) => {
            let x = normalize_int_expr(env, x);
            let y = normalize_int_expr(env, y);

            if x.is_constant() {
                return y * x.constant;
            }
            if y.is_constant() {
                return x * y.constant;
            }

            let xdom = env.norm.get_domain_linear_sum(&x);
            let xvar;
            if let Some(v) = x.as_singleton() {
                xvar = *v;
            } else {
                xvar = env.norm.new_int_var(xdom.clone());
                let mut c = Constraint::new();
                c.add_linear(LinearLit::new(x - LinearSum::singleton(xvar), CmpOp::Eq));
                env.norm.add_constraint(c);
            }

            let ydom = env.norm.get_domain_linear_sum(&y);
            let yvar;
            if let Some(v) = y.as_singleton() {
                yvar = *v;
            } else {
                yvar = env.norm.new_int_var(ydom.clone());
                let mut c = Constraint::new();
                c.add_linear(LinearLit::new(y - LinearSum::singleton(yvar), CmpOp::Eq));
                env.norm.add_constraint(c);
            }

            let xdom_low;
            let xdom_high;
            match env.norm.vars.int_var(xvar) {
                IntVarRepresentation::Domain(dom) => {
                    xdom_low = dom.lower_bound_checked();
                    xdom_high = dom.upper_bound_checked();
                }
                IntVarRepresentation::Binary {
                    cond: _,
                    v_false,
                    v_true,
                } => {
                    xdom_low = *v_false;
                    xdom_high = *v_true;
                }
            }
            assert!(xdom_low <= xdom_high);
            let ydom_low;
            let ydom_high;
            match env.norm.vars.int_var(yvar) {
                IntVarRepresentation::Domain(dom) => {
                    ydom_low = dom.lower_bound_checked();
                    ydom_high = dom.upper_bound_checked();
                }
                IntVarRepresentation::Binary {
                    cond: _,
                    v_false,
                    v_true,
                } => {
                    ydom_low = *v_false;
                    ydom_high = *v_true;
                }
            }
            assert!(ydom_low <= ydom_high);
            let edges = [
                xdom_low * ydom_low,
                xdom_low * ydom_high,
                xdom_high * ydom_low,
                xdom_high * ydom_high,
            ];
            let zdom_low = *edges.iter().min().unwrap();
            let zdom_high = *edges.iter().max().unwrap();
            let zvar = env
                .norm
                .new_int_var(Domain::range_from_checked(zdom_low, zdom_high));

            env.norm
                .add_extra_constraint(ExtraConstraint::Mul(xvar, yvar, zvar));
            LinearSum::singleton(zvar)
        }
    }
}

#[cfg(not(feature = "csp-extra-constraints"))]
fn normalize_circuit(_: &mut NormalizerEnv, _: Vec<NIntVar>) {
    panic!("feature not enabled");
}

#[cfg(feature = "csp-extra-constraints")]
fn normalize_circuit(env: &mut NormalizerEnv, vars: Vec<NIntVar>) {
    let n = vars.len();

    let mut edges = vec![];
    let mut out_edges: Vec<Vec<usize>> = vec![vec![]; n];
    let mut in_edges: Vec<Vec<usize>> = vec![vec![]; n];
    let mut self_edge: Vec<Option<NBoolLit>> = vec![None; n];

    for i in 0..n {
        let mut valid_domain = vec![];
        let mut has_out_of_range = false;
        match env.norm.vars.int_var(vars[i]) {
            IntVarRepresentation::Domain(domain) => {
                for v in domain.enumerate() {
                    if v >= 0 && v < n as i32 {
                        valid_domain.push(v);
                    } else {
                        has_out_of_range = true;
                    }
                }
            }
            IntVarRepresentation::Binary {
                cond: _,
                v_false,
                v_true,
            } => {
                if *v_true >= 0 && *v_true < n as i32 {
                    valid_domain.push(*v_true);
                } else {
                    has_out_of_range = true;
                }
                if *v_false >= 0 && *v_false < n as i32 {
                    valid_domain.push(*v_false);
                } else {
                    has_out_of_range = true;
                }
            }
        }
        if has_out_of_range {
            normalize_and_register_expr(env, IntExpr::NVar(vars[i]).ge(IntExpr::Const(0)));
            normalize_and_register_expr(
                env,
                IntExpr::NVar(vars[i]).le(IntExpr::Const(n as i32 - 1)),
            );
        }
        for j in valid_domain {
            let j = j.get() as usize;

            let v = env.norm.new_bool_var();
            let lit = NBoolLit::new(v, false);
            normalize_and_register_expr(
                env,
                BoolExpr::NVar(v).iff(IntExpr::NVar(vars[i]).eq(IntExpr::Const(j as i32))),
            );

            if i != j {
                out_edges[i].push(edges.len());
                in_edges[j].push(edges.len());
                edges.push((i, j, lit));
            } else {
                self_edge[i] = Some(lit);
            }
        }
    }

    for i in 0..n {
        let mut in_lits = in_edges[i].iter().map(|&e| edges[e].2).collect::<Vec<_>>();
        in_lits.extend(self_edge[i]);

        for p in 0..in_lits.len() {
            for q in (p + 1)..in_lits.len() {
                let c = Constraint {
                    bool_lit: vec![!in_lits[p], !in_lits[q]],
                    linear_lit: vec![],
                };
                env.norm.add_constraint(c)
            }
        }
        let c = Constraint {
            bool_lit: in_lits,
            linear_lit: vec![],
        };
        env.norm.add_constraint(c);
    }

    let mut edges_undir = edges
        .iter()
        .map(|&(s, t, l)| ((s.min(t), s.max(t)), l))
        .collect::<Vec<_>>();
    edges_undir.sort_by(|(p, _), (q, _)| p.cmp(q));

    let mut edges_undir_dedup = vec![];
    {
        let mut i = 0;
        while i < edges_undir.len() {
            if i + 1 < edges_undir.len() && edges_undir[i].0 == edges_undir[i + 1].0 {
                let lit1 = edges_undir[i].1;
                assert!(!lit1.negated);
                let lit2 = edges_undir[i + 1].1;
                assert!(!lit2.negated);
                let v = env.norm.new_bool_var();
                normalize_and_register_expr(
                    env,
                    BoolExpr::NVar(v).iff(BoolExpr::NVar(lit1.var) | BoolExpr::NVar(lit2.var)),
                );
                edges_undir_dedup.push((edges_undir[i].0, NBoolLit::new(v, false)));
                i += 2;
            } else {
                edges_undir_dedup.push(edges_undir[i]);
                i += 1;
            }
        }
    }
    let mut adj_edges: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, &((u, v), _)) in edges_undir_dedup.iter().enumerate() {
        adj_edges[u].push(i);
        adj_edges[v].push(i);
    }
    let mut line_graph_edges: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        for j in 0..adj_edges[i].len() {
            for k in 0..j {
                line_graph_edges.push((adj_edges[i][j], adj_edges[i][k]));
            }
        }
    }
    let line_graph_vertices = edges_undir_dedup
        .iter()
        .map(|&(_, l)| l)
        .collect::<Vec<_>>();

    env.norm
        .add_extra_constraint(ExtraConstraint::ActiveVerticesConnected(
            line_graph_vertices,
            line_graph_edges,
        ));
}

#[cfg(not(feature = "csp-extra-constraints"))]
fn normalize_extension_supports(_: &mut NormalizerEnv, _: Vec<NIntVar>, _: Vec<Vec<Option<i32>>>) {
    panic!("feature not enabled");
}

#[cfg(feature = "csp-extra-constraints")]
fn normalize_extension_supports(
    env: &mut NormalizerEnv,
    vars: Vec<NIntVar>,
    supports: Vec<Vec<Option<i32>>>,
) {
    if env.config.use_native_extension_supports {
        let supports = supports
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|x| x.map(CheckedInt::new))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        env.norm
            .add_extra_constraint(ExtraConstraint::ExtensionSupports(vars, supports));
        return;
    }

    fn linear_eq(var: NIntVar, constant: i32) -> LinearLit {
        LinearLit::new(
            LinearSum::singleton(var) - LinearSum::constant(CheckedInt::new(constant)),
            CmpOp::Eq,
        )
    }
    fn linear_ne(var: NIntVar, constant: i32) -> LinearLit {
        LinearLit::new(
            LinearSum::singleton(var) - LinearSum::constant(CheckedInt::new(constant)),
            CmpOp::Ne,
        )
    }

    let mut has_star = false;
    for s in &supports {
        for v in s {
            if v.is_none() {
                has_star = true;
                break;
            }
        }
        if has_star {
            break;
        }
    }

    if has_star {
        // TODO: support efficient coding with stars
        let mut exprs = vec![];
        for s in &supports {
            let mut c = vec![];
            assert_eq!(s.len(), vars.len());
            for i in 0..vars.len() {
                if let Some(n) = s[i] {
                    c.push(Box::new(IntExpr::NVar(vars[i]).eq(IntExpr::Const(n))));
                }
            }
            exprs.push(Box::new(BoolExpr::And(c)));
        }
        normalize_stmt(env, Stmt::Expr(BoolExpr::Or(exprs)));
        return;
    }

    let domains = vars
        .iter()
        .map(|&v| {
            env.norm
                .vars
                .int_var(v)
                .enumerate()
                .into_iter()
                .map(|x| x.get())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for domain in &domains {
        for i in 1..domain.len() {
            assert!(domain[i - 1] < domain[i]);
        }
    }
    let mut supports = supports
        .into_iter()
        .map(|row| row.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    supports.sort();

    let mut supports_idx = vec![];
    for support in supports {
        let mut row = vec![];
        for i in 0..vars.len() {
            let idx = domains[i].binary_search(&support[i]);
            if let Ok(idx) = idx {
                row.push(idx);
            } else {
                break;
            }
        }
        if row.len() == vars.len() {
            supports_idx.push(row);
        }
    }

    for n_prefix in 0..vars.len() {
        let mut left = 0;
        while left < supports_idx.len() {
            let mut right = left + 1;
            while right < supports_idx.len()
                && supports_idx[left][0..n_prefix] == supports_idx[right][0..n_prefix]
            {
                right += 1;
            }

            for vi in n_prefix..vars.len() {
                let mut has_val = vec![false; domains[vi].len()];
                for i in left..right {
                    has_val[supports_idx[i][vi]] = true;
                }

                if has_val.iter().all(|&x| x) {
                    continue;
                }

                let mut constraint = Constraint::new();
                for i in 0..n_prefix {
                    constraint.add_linear(linear_ne(vars[i], domains[i][supports_idx[left][i]]));
                }
                for i in 0..has_val.len() {
                    if has_val[i] {
                        constraint.add_linear(linear_eq(vars[vi], domains[vi][i]));
                    }
                }
                env.norm.add_constraint(constraint);
            }

            left = right;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::super::csp;
    use super::super::domain::Domain;
    use super::super::norm_csp;
    use super::super::norm_csp::BoolVar as NBoolVar;
    use super::super::norm_csp::IntVarRepresentation;
    use super::*;
    use crate::test_utils;

    struct NormalizerTester {
        original_constr: Vec<Stmt>,
        csp: CSP,
        norm: NormCSP,
        bool_vars: Vec<BoolVar>,
        int_vars: Vec<(IntVar, Domain)>,
        map: NormalizeMap,
        config: Config,
    }

    impl NormalizerTester {
        fn new() -> NormalizerTester {
            NormalizerTester {
                original_constr: vec![],
                csp: CSP::new(),
                norm: NormCSP::new(),
                bool_vars: vec![],
                int_vars: vec![],
                map: NormalizeMap::new(),
                config: Config::default(),
            }
        }

        fn new_bool_var(&mut self) -> BoolVar {
            let ret = self.csp.new_bool_var();
            self.bool_vars.push(ret);
            ret
        }

        fn new_int_var(&mut self, domain: Domain) -> IntVar {
            let ret = self.csp.new_int_var(domain.clone());
            self.int_vars.push((ret, domain));
            ret
        }

        fn add_expr(&mut self, expr: BoolExpr) {
            self.add_constraint(Stmt::Expr(expr));
        }

        fn add_constraint(&mut self, stmt: Stmt) {
            let cloned = crate::csp::test_utils::clone_stmt(&stmt);
            self.original_constr.push(cloned);
            self.csp.add_constraint(stmt);
        }

        fn check(&mut self) {
            let csp_assignments = crate::csp::test_utils::csp_all_assignments(&self.csp);

            normalize(&mut self.csp, &mut self.norm, &mut self.map, &self.config);

            let mut norm_csp_assignments = crate::norm_csp::test_utils::norm_csp_all_assignments(&self.norm);

            let mut csp_assignments_converted = csp_assignments
                .into_iter()
                .map(|a| {
                    let mut n_assignment = norm_csp::Assignment::new();
                    for &v in &self.bool_vars {
                        match self.map.get_bool_var_raw(v) {
                            ConvertedBoolVar::Lit(lit) => {
                                n_assignment.set_bool(lit.var, a.get_bool(v).unwrap() ^ lit.negated);
                            }
                            ConvertedBoolVar::NotConverted => (),
                            ConvertedBoolVar::Removed => (),
                        }
                    }
                    for (v, _) in &self.int_vars {
                        n_assignment.set_int(
                            self.map.get_int_var(*v).unwrap(),
                            a.get_int(*v).unwrap(),
                        );
                    }
                    n_assignment
                })
                .collect::<Vec<_>>();
            csp_assignments_converted.sort();
            csp_assignments_converted.dedup();

            let mut converted_bool_vars = BTreeSet::<NBoolVar>::new();
            for v in &self.bool_vars {
                match self.map.get_bool_var(*v) {
                    Some(v) => {
                        converted_bool_vars.insert(v.var);
                    }
                    None => (),
                }
            }
            let mut converted_int_vars = BTreeSet::<NIntVar>::new();
            for (v, _) in &self.int_vars {
                match self.map.get_int_var(*v) {
                    Some(v) => {
                        converted_int_vars.insert(v);
                    }
                    None => (),
                }
            }

            for a in &mut norm_csp_assignments {
                a.bool_val.retain(|var, _| converted_bool_vars.contains(var));
                a.int_val.retain(|var, _| converted_int_vars.contains(var));
            }

            norm_csp_assignments.sort();
            norm_csp_assignments.dedup();

            assert_eq!(csp_assignments_converted, norm_csp_assignments);
        }
    }

    #[test]
    fn test_normalization_empty() {
        let mut tester = NormalizerTester::new();

        tester.check();
    }

    #[test]
    fn test_normalization_and() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        tester.add_expr(x.expr() & y.expr());

        tester.check();
    }

    #[test]
    fn test_normalization_or() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        tester.add_expr(x.expr() | y.expr());

        tester.check();
    }

    #[test]
    fn test_normalization_not() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        tester.add_expr(!x.expr());

        tester.check();
    }

    #[test]
    fn test_normalization_imp() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        tester.add_expr(x.expr().imp(y.expr()));

        tester.check();
    }

    #[test]
    fn test_normalization_xor1() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        tester.add_expr(x.expr() ^ y.expr());

        tester.check();
    }

    #[test]
    fn test_normalization_xor2() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        let z = tester.new_bool_var();
        let w = tester.new_bool_var();
        tester.add_expr(x.expr() ^ y.expr() ^ z.expr() ^ w.expr());

        tester.check();
    }

    #[test]
    fn test_normalization_iff() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        tester.add_expr(x.expr().iff(y.expr()));

        tester.check();
    }

    #[test]
    fn test_normalization_many_xor() {
        let mut csp = CSP::new();

        let mut expr = csp.new_bool_var().expr();
        for _ in 0..20 {
            expr = csp.new_bool_var().expr() ^ expr;
        }
        csp.add_constraint(Stmt::Expr(expr));

        let mut norm_csp = NormCSP::new();
        let mut map = NormalizeMap::new();
        normalize(&mut csp, &mut norm_csp, &mut map, &Config::default());
        assert!(norm_csp.constraints.len() <= 200);
    }

    #[test]
    fn test_normalization_xor_constant() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        tester.add_expr(x.expr() ^ BoolExpr::Const(false));

        tester.check();
    }

    #[test]
    fn test_normalization_logic_complex1() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        let z = tester.new_bool_var();
        tester.add_expr(
            (x.expr() & y.expr() & !z.expr()) | (!x.expr() & !y.expr()) | !(y.expr() & z.expr()),
        );

        tester.check();
    }

    #[test]
    fn test_normalization_logic_complex2() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        let z = tester.new_bool_var();
        let v = tester.new_bool_var();
        let w = tester.new_bool_var();
        tester.add_expr(
            x.expr().iff((y.expr() & w.expr()).imp(z.expr())) ^ (z.expr() | (v.expr() ^ w.expr())),
        );

        tester.check();
    }

    #[test]
    fn test_normalization_numeral() {
        let mut tester = NormalizerTester::new();

        let a = tester.new_int_var(Domain::range(0, 2));
        let b = tester.new_int_var(Domain::range(0, 2));
        tester.add_expr(a.expr().ge(b.expr()));

        tester.check();
    }

    #[test]
    fn test_normalization_linear_1() {
        let mut tester = NormalizerTester::new();

        let a = tester.new_int_var(Domain::range(0, 2));
        let b = tester.new_int_var(Domain::range(0, 2));
        tester.add_expr((a.expr() + b.expr()).ge(a.expr() * 2 - b.expr()));

        tester.check();
    }

    #[test]
    fn test_normalization_linear_2() {
        let mut tester = NormalizerTester::new();

        let a = tester.new_int_var(Domain::range(0, 2));
        let b = tester.new_int_var(Domain::range(0, 2));
        tester.add_expr((a.expr() + b.expr()).ge(IntExpr::Const(3)));

        tester.check();
    }
    #[test]
    fn test_normalization_if() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_bool_var();
        let a = tester.new_int_var(Domain::range(0, 2));
        let b = tester.new_int_var(Domain::range(0, 2));
        let c = tester.new_int_var(Domain::range(0, 2));
        tester.add_expr(
            x.expr()
                .ite(a.expr(), b.expr() + c.expr())
                .le(a.expr() - b.expr()),
        );

        tester.check();
    }

    #[test]
    fn test_normalization_abs() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_int_var(Domain::range(-5, 6));
        let b = tester.new_int_var(Domain::range(-1, 7));
        tester.add_expr(b.expr().ge(x.expr().abs()));

        tester.check();
    }

    #[test]
    fn test_normalization_mul() {
        let mut tester = NormalizerTester::new();

        let x = tester.new_int_var(Domain::range(-3, 3));
        let y = tester.new_int_var(Domain::range(-2, 2));
        let z = tester.new_int_var(Domain::range(-4, 4));
        tester.add_expr(z.expr().eq(x.expr() * y.expr()));

        tester.check();
    }

    #[test]
    fn test_normalization_removed_variables() {
        let mut csp = CSP::new();
        let mut norm_csp = NormCSP::new();
        let mut map = NormalizeMap::new();
        let config = Config::default();

        let v = csp.new_bool_var();
        let w = csp.new_bool_var();
        let x = csp.new_bool_var();
        let y = csp.new_bool_var();
        let z = csp.new_bool_var();
        csp.add_constraint(Stmt::Expr(!w.expr()));
        csp.add_constraint(Stmt::Expr(w.expr() | x.expr()));
        csp.add_constraint(Stmt::Expr(y.expr() | z.expr()));

        csp.optimize(true, false);

        normalize(&mut csp, &mut norm_csp, &mut map, &config);

        assert!(map.bool_map[v].is_not_converted());
        assert!(map.bool_map[w].is_removed());
        assert!(map.bool_map[x].is_removed());
        assert!(map.bool_map[y].is_lit());
        assert!(map.bool_map[z].is_lit());
    }

    #[test]
    fn test_normalization_equiv_optimization() {
        let mut tester = NormalizerTester::new();

        let v = tester.new_bool_var();
        let w = tester.new_bool_var();
        let x = tester.new_bool_var();
        let y = tester.new_bool_var();
        let z = tester.new_bool_var();
        tester.add_expr(v.expr().iff(w.expr()));
        tester.add_expr(w.expr() ^ x.expr());
        tester.add_expr(x.expr() ^ y.expr());
        tester.add_expr(y.expr().iff(z.expr()));

        tester.config.merge_equivalent_variables = true;
        tester.check();
        assert_eq!(
            tester.map.get_bool_var(v).unwrap().var,
            tester.map.get_bool_var(z).unwrap().var,
        );
    }

    #[test]
    fn test_normalization_alldifferent() {
        let mut tester = NormalizerTester::new();

        let a = tester.new_int_var(Domain::range(0, 3));
        let b = tester.new_int_var(Domain::range(0, 3));
        let c = tester.new_int_var(Domain::range(0, 3));
        tester.add_constraint(Stmt::AllDifferent(vec![a.expr(), b.expr(), c.expr()]));
        tester.check();
    }

    #[cfg(feature = "csp-extra-constraints")]
    #[test]
    fn test_normalization_extension_supports_1() {
        for use_native in [false, true] {
            let mut tester = NormalizerTester::new();
            tester.config.use_native_extension_supports = use_native;

            let a = tester.new_int_var(Domain::range(0, 2));
            let b = tester.new_int_var(Domain::range(0, 2));
            let c = tester.new_int_var(Domain::range(0, 2));
            tester.add_constraint(Stmt::ExtensionSupports(
                vec![a.expr(), b.expr(), c.expr()],
                vec![
                    vec![Some(0), Some(0), Some(1)],
                    vec![Some(0), Some(1), Some(1)],
                    vec![Some(0), Some(1), Some(2)],
                    vec![Some(1), Some(1), Some(0)],
                    vec![Some(1), Some(2), Some(1)],
                    vec![Some(2), Some(0), Some(2)],
                ],
            ));
            tester.check();
        }
    }

    #[cfg(feature = "csp-extra-constraints")]
    #[test]
    fn test_normalization_extension_supports_2() {
        for use_native in [false, true] {
            let mut tester = NormalizerTester::new();
            tester.config.use_native_extension_supports = use_native;

            let a = tester.new_int_var(Domain::range(0, 3));
            let b = tester.new_int_var(Domain::range(0, 3));
            let c = tester.new_int_var(Domain::range(0, 3));
            tester.add_constraint(Stmt::ExtensionSupports(
                vec![a.expr(), b.expr(), c.expr()],
                vec![
                    vec![None, Some(0), Some(1)],
                    vec![Some(0), Some(1), Some(1)],
                    vec![Some(0), Some(1), Some(2)],
                    vec![Some(1), None, Some(0)],
                    vec![Some(1), Some(2), None],
                    vec![Some(2), Some(0), Some(2)],
                ],
            ));
            tester.check();
        }
    }

    #[cfg(feature = "csp-extra-constraints")]
    #[test]
    fn test_normalization_circuit_1() {
        let mut tester = NormalizerTester::new();

        let a = tester.new_int_var(Domain::range(0, 2));
        let b = tester.new_int_var(Domain::range(0, 2));
        let c = tester.new_int_var(Domain::range(-1, 2));
        tester.add_constraint(Stmt::Circuit(vec![a.expr(), b.expr(), c.expr()]));
        tester.check();
    }
}
