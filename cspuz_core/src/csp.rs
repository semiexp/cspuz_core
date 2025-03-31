use crate::arithmetic::CheckedInt;
use crate::domain::Domain;
use crate::util::{ConvertMapIndex, UpdateStatus};
use std::collections::{btree_map, BTreeMap};
use std::ops::{Index, IndexMut};

pub use super::csp_repr::{BoolExpr, BoolVar, IntExpr, IntVar, Stmt};

pub enum BoolVarStatus {
    Infeasible,
    Fixed(bool),
    Unfixed,
}

pub(super) struct BoolVarData {
    possibility_mask: u8,
}

impl BoolVarData {
    fn new() -> BoolVarData {
        BoolVarData {
            possibility_mask: 3,
        }
    }

    pub(super) fn get_status(&self) -> BoolVarStatus {
        match self.possibility_mask {
            0 => BoolVarStatus::Infeasible,
            1 => BoolVarStatus::Fixed(false),
            2 => BoolVarStatus::Fixed(true),
            3 => BoolVarStatus::Unfixed,
            _ => panic!(),
        }
    }

    #[allow(dead_code)]
    fn is_feasible(&self, b: bool) -> bool {
        (self.possibility_mask & (if b { 2 } else { 1 })) != 0
    }

    #[allow(dead_code)]
    fn is_unsatisfiable(&self) -> bool {
        self.possibility_mask == 0
    }

    #[allow(dead_code)]
    fn set_infeasible(&mut self, b: bool) -> UpdateStatus {
        let res = self.is_feasible(b);
        self.possibility_mask &= if b { 1 } else { 2 };
        if res {
            if self.is_unsatisfiable() {
                UpdateStatus::Unsatisfiable
            } else {
                UpdateStatus::Updated
            }
        } else {
            UpdateStatus::NotUpdated
        }
    }
}

pub(super) struct IntVarData {
    pub(super) domain: Domain,
}

impl IntVarData {
    fn new(domain: Domain) -> IntVarData {
        IntVarData { domain }
    }
}

pub(super) struct CSPVars {
    bool_var: Vec<BoolVarData>,
    int_var: Vec<IntVarData>,
}

impl CSPVars {
    pub(super) fn bool_vars_iter(&self) -> impl Iterator<Item = BoolVar> {
        (0..self.bool_var.len()).map(|x| BoolVar::new(x))
    }

    pub(super) fn int_vars_iter(&self) -> impl Iterator<Item = IntVar> {
        (0..self.int_var.len()).map(|x| IntVar::new(x))
    }

    pub(super) fn int_var(&self, var: IntVar) -> &IntVarData {
        &self.int_var[var.to_index()]
    }

    fn constant_folding_bool(&self, expr: &mut BoolExpr) {
        match expr {
            BoolExpr::Const(_) => (),
            BoolExpr::Var(v) => {
                let value = &self[*v];
                match value.get_status() {
                    BoolVarStatus::Fixed(b) => *expr = BoolExpr::Const(b),
                    BoolVarStatus::Infeasible => panic!(), // this should be handled when the inconsistency first occurred.
                    BoolVarStatus::Unfixed => (),
                }
            }
            BoolExpr::NVar(_) => unreachable!(),
            BoolExpr::And(exprs) => {
                exprs.iter_mut().for_each(|e| self.constant_folding_bool(e));
                if exprs.iter().any(|e| e.is_const() == Some(false)) {
                    *expr = BoolExpr::Const(false);
                } else {
                    exprs.retain(|e| e.is_const().is_none());
                    if exprs.len() == 0 {
                        *expr = BoolExpr::Const(true);
                    } else if exprs.len() == 1 {
                        *expr = *exprs.remove(0);
                    }
                }
            }
            BoolExpr::Or(exprs) => {
                exprs.iter_mut().for_each(|e| self.constant_folding_bool(e));
                if exprs.iter().any(|e| e.is_const() == Some(true)) {
                    *expr = BoolExpr::Const(true);
                } else {
                    exprs.retain(|e| e.is_const().is_none());
                    if exprs.len() == 0 {
                        *expr = BoolExpr::Const(false);
                    } else if exprs.len() == 1 {
                        *expr = *exprs.remove(0);
                    }
                }
            }
            BoolExpr::Not(e) => {
                self.constant_folding_bool(e);
                match e.is_const() {
                    Some(b) => *expr = BoolExpr::Const(!b),
                    _ => (),
                }
            }
            BoolExpr::Xor(e1, e2) => {
                self.constant_folding_bool(e1);
                self.constant_folding_bool(e2);

                match (e1.is_const(), e2.is_const()) {
                    (Some(b1), Some(b2)) => *expr = BoolExpr::Const(b1 ^ b2),
                    (Some(true), None) => {
                        let e2 = std::mem::replace(e2.as_mut(), BoolExpr::Const(false));
                        *expr = BoolExpr::Not(Box::new(e2));
                    }
                    (Some(false), None) => {
                        let e2 = std::mem::replace(e2.as_mut(), BoolExpr::Const(false));
                        *expr = e2;
                    }
                    (None, Some(true)) => {
                        let e1 = std::mem::replace(e1.as_mut(), BoolExpr::Const(false));
                        *expr = BoolExpr::Not(Box::new(e1));
                    }
                    (None, Some(false)) => {
                        let e1 = std::mem::replace(e1.as_mut(), BoolExpr::Const(false));
                        *expr = e1;
                    }
                    (None, None) => (),
                }
            }
            BoolExpr::Iff(e1, e2) => {
                self.constant_folding_bool(e1);
                self.constant_folding_bool(e2);

                match (e1.is_const(), e2.is_const()) {
                    (Some(b1), Some(b2)) => *expr = BoolExpr::Const(b1 == b2),
                    (Some(false), None) => {
                        let e2 = std::mem::replace(e2.as_mut(), BoolExpr::Const(false));
                        *expr = BoolExpr::Not(Box::new(e2));
                    }
                    (Some(true), None) => {
                        let e2 = std::mem::replace(e2.as_mut(), BoolExpr::Const(false));
                        *expr = e2;
                    }
                    (None, Some(false)) => {
                        let e1 = std::mem::replace(e1.as_mut(), BoolExpr::Const(false));
                        *expr = BoolExpr::Not(Box::new(e1));
                    }
                    (None, Some(true)) => {
                        let e1 = std::mem::replace(e1.as_mut(), BoolExpr::Const(false));
                        *expr = e1;
                    }
                    (None, None) => (),
                }
            }
            BoolExpr::Imp(e1, e2) => {
                self.constant_folding_bool(e1);
                self.constant_folding_bool(e2);

                match (e1.is_const(), e2.is_const()) {
                    (Some(b1), Some(b2)) => *expr = BoolExpr::Const(!b1 || b2),
                    (Some(false), None) | (None, Some(true)) => {
                        *expr = BoolExpr::Const(true);
                    }
                    (Some(true), None) => {
                        let e2 = std::mem::replace(e2.as_mut(), BoolExpr::Const(false));
                        *expr = e2;
                    }
                    (None, Some(false)) => {
                        let e1 = std::mem::replace(e1.as_mut(), BoolExpr::Const(false));
                        *expr = BoolExpr::Not(Box::new(e1));
                    }
                    (None, None) => (),
                }
            }
            BoolExpr::Cmp(_, t, f) => {
                self.constant_folding_int(t);
                self.constant_folding_int(f);
            }
        }
    }

    fn constant_folding_int(&self, expr: &mut IntExpr) {
        match expr {
            IntExpr::Const(_) => (),
            IntExpr::Var(v) => {
                let value = self.int_var(*v);
                if let Some(c) = value.domain.as_constant() {
                    *expr = IntExpr::Const(c.get());
                }
            }
            IntExpr::NVar(_) => unreachable!(),
            IntExpr::Linear(terms) => {
                terms
                    .iter_mut()
                    .for_each(|(e, _)| self.constant_folding_int(e));
                if terms.len() == 0 {
                    *expr = IntExpr::Const(0);
                } else if terms.len() == 1 && terms[0].1 == 1 {
                    *expr = *terms.remove(0).0;
                }
            }
            IntExpr::If(c, t, f) => {
                self.constant_folding_bool(c);
                self.constant_folding_int(t);
                self.constant_folding_int(f);

                match c.is_const() {
                    Some(true) => {
                        let t = std::mem::replace(t.as_mut(), IntExpr::Const(0));
                        *expr = t;
                    }
                    Some(false) => {
                        let f = std::mem::replace(f.as_mut(), IntExpr::Const(0));
                        *expr = f;
                    }
                    None => (),
                }
            }
            IntExpr::Abs(x) => {
                self.constant_folding_int(x);
            }
            IntExpr::Mul(x, y) => {
                self.constant_folding_int(x);
                self.constant_folding_int(y);
                // TODO: implement constant folding
            }
        }
    }

    fn constant_prop_bool(&mut self, expr: &BoolExpr, expected: bool) -> UpdateStatus {
        match expr {
            &BoolExpr::Const(c) => {
                if c == expected {
                    UpdateStatus::NotUpdated
                } else {
                    UpdateStatus::Unsatisfiable
                }
            }
            &BoolExpr::Var(v) => self[v].set_infeasible(!expected),
            BoolExpr::NVar(_) => unreachable!(),
            BoolExpr::And(exprs) => {
                if expected {
                    let mut ret = UpdateStatus::NotUpdated;
                    for e in exprs {
                        ret |= self.constant_prop_bool(e, true);
                    }
                    ret
                } else {
                    UpdateStatus::NotUpdated
                }
            }
            BoolExpr::Or(exprs) => {
                if !expected {
                    let mut ret = UpdateStatus::NotUpdated;
                    for e in exprs {
                        ret |= self.constant_prop_bool(e, false);
                    }
                    ret
                } else {
                    UpdateStatus::NotUpdated
                }
            }
            BoolExpr::Not(e) => self.constant_prop_bool(e, !expected),
            BoolExpr::Imp(e1, e2) => {
                if !expected {
                    self.constant_prop_bool(e1, true) | self.constant_prop_bool(e2, false)
                } else {
                    UpdateStatus::NotUpdated
                }
            }
            BoolExpr::Xor(_, _) | BoolExpr::Iff(_, _) | BoolExpr::Cmp(_, _, _) => {
                UpdateStatus::NotUpdated
            }
        }
    }
}

impl Index<BoolVar> for CSPVars {
    type Output = BoolVarData;

    fn index(&self, index: BoolVar) -> &Self::Output {
        &self.bool_var[index.to_index()]
    }
}

impl IndexMut<BoolVar> for CSPVars {
    fn index_mut(&mut self, index: BoolVar) -> &mut Self::Output {
        &mut self.bool_var[index.to_index()]
    }
}

impl Index<IntVar> for CSPVars {
    type Output = IntVarData;

    fn index(&self, index: IntVar) -> &Self::Output {
        &self.int_var[index.to_index()]
    }
}

impl IndexMut<IntVar> for CSPVars {
    fn index_mut(&mut self, index: IntVar) -> &mut Self::Output {
        &mut self.int_var[index.to_index()]
    }
}

pub enum IntVarStatus {
    Infeasible,
    Fixed(CheckedInt),
    Unfixed(CheckedInt), // an example of feasible value
}

pub struct CSP {
    pub(super) vars: CSPVars,
    pub(super) constraints: Vec<Stmt>,
    inconsistent: bool,
    pub(super) prenormalize_vars: Vec<BoolVar>,
}

impl CSP {
    pub fn new() -> CSP {
        CSP {
            vars: CSPVars {
                bool_var: vec![],
                int_var: vec![],
            },
            constraints: vec![],
            inconsistent: false,
            prenormalize_vars: vec![],
        }
    }

    pub fn new_bool_var(&mut self) -> BoolVar {
        let id = self.vars.bool_var.len();
        self.vars.bool_var.push(BoolVarData::new());
        BoolVar::new(id)
    }

    pub fn new_int_var(&mut self, domain: Domain) -> IntVar {
        let id = self.vars.int_var.len();
        self.vars.int_var.push(IntVarData::new(domain));
        IntVar::new(id)
    }

    pub fn add_prenormalize_var(&mut self, var: BoolVar) {
        self.prenormalize_vars.push(var);
    }

    pub fn new_int_var_from_list(&mut self, domain_list: Vec<CheckedInt>) -> IntVar {
        assert!(domain_list.len() > 0);
        let mut domain_list = domain_list;
        domain_list.sort();
        domain_list.dedup();
        let domain = Domain::enumerative_from_checked(domain_list.clone());
        let id = self.vars.int_var.len();
        self.vars.int_var.push(IntVarData::new(domain));
        IntVar::new(id)
    }

    pub fn add_constraint(&mut self, stmt: Stmt) {
        self.constraints.push(stmt);
    }

    pub fn is_inconsistent(&self) -> bool {
        self.inconsistent
    }

    pub fn get_bool_var_status(&self, var: BoolVar) -> BoolVarStatus {
        self.vars[var].get_status()
    }

    pub fn get_int_var_status(&self, var: IntVar) -> IntVarStatus {
        let data = self.vars.int_var(var);
        let domain = &data.domain;
        if domain.is_infeasible() {
            IntVarStatus::Infeasible
        } else if let Some(v) = domain.as_constant() {
            IntVarStatus::Fixed(v)
        } else {
            IntVarStatus::Unfixed(domain.lower_bound_checked())
        }
    }

    pub fn apply_constant_folding(&mut self) {
        let vars = &mut self.vars;
        for stmt in &mut self.constraints {
            match stmt {
                Stmt::Expr(e) => vars.constant_folding_bool(e),
                Stmt::AllDifferent(exprs) => {
                    exprs.iter_mut().for_each(|e| vars.constant_folding_int(e));
                }
                Stmt::ActiveVerticesConnected(vertices, _edges) => {
                    vertices
                        .iter_mut()
                        .for_each(|e| vars.constant_folding_bool(e));
                }
                Stmt::Circuit(exprs) => {
                    exprs.iter_mut().for_each(|e| vars.constant_folding_int(e));
                }
                Stmt::ExtensionSupports(exprs, _) => {
                    exprs.iter_mut().for_each(|e| vars.constant_folding_int(e));
                }
                Stmt::GraphDivision(sizes, _edges, edge_lits, _opts) => {
                    sizes.iter_mut().for_each(|e| {
                        e.as_mut().map(|e| vars.constant_folding_int(e));
                    });
                    edge_lits
                        .iter_mut()
                        .for_each(|e| vars.constant_folding_bool(e));
                }
                Stmt::CustomConstraint(exprs, _) => {
                    exprs.iter_mut().for_each(|e| vars.constant_folding_bool(e));
                }
            }
        }
    }

    pub fn optimize(&mut self, use_propagate: bool, verbose: bool) {
        let mut pp_before_optimize = vec![];
        if verbose {
            for stmt in &self.constraints {
                let mut buf = Vec::<u8>::new();
                stmt.pretty_print(&mut buf).unwrap();
                pp_before_optimize.push(String::from_utf8(buf).unwrap());
            }
        }
        if use_propagate {
            loop {
                self.apply_constant_folding();
                let vars = &mut self.vars;
                let mut update_status = UpdateStatus::NotUpdated;
                for stmt in &self.constraints {
                    match stmt {
                        Stmt::Expr(e) => {
                            update_status |= vars.constant_prop_bool(e, true);
                        }
                        _ => (),
                    }
                }
                match update_status {
                    UpdateStatus::NotUpdated => break,
                    UpdateStatus::Updated => (),
                    UpdateStatus::Unsatisfiable => {
                        self.inconsistent = true;
                        return;
                    }
                }
            }
        } else {
            self.apply_constant_folding();
        }

        if verbose {
            let mut pp_after_optimize = vec![];
            for stmt in &self.constraints {
                let mut buf = Vec::<u8>::new();
                stmt.pretty_print(&mut buf).unwrap();
                pp_after_optimize.push(String::from_utf8(buf).unwrap());
            }

            assert_eq!(pp_before_optimize.len(), pp_after_optimize.len());
            for i in 0..pp_before_optimize.len() {
                eprintln!("{} -> {}", pp_before_optimize[i], pp_after_optimize[i]);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Assignment {
    bool_val: BTreeMap<BoolVar, bool>,
    int_val: BTreeMap<IntVar, i32>,
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            bool_val: BTreeMap::new(),
            int_val: BTreeMap::new(),
        }
    }

    pub fn set_bool(&mut self, var: BoolVar, val: bool) {
        self.bool_val.insert(var, val);
    }

    pub fn set_int(&mut self, var: IntVar, val: i32) {
        self.int_val.insert(var, val);
    }

    pub fn get_bool(&self, var: BoolVar) -> Option<bool> {
        self.bool_val.get(&var).copied()
    }

    pub fn get_int(&self, var: IntVar) -> Option<i32> {
        self.int_val.get(&var).copied()
    }

    pub fn remove_bool(&mut self, var: BoolVar) -> Option<bool> {
        self.bool_val.remove(&var)
    }

    pub fn remove_int(&mut self, var: IntVar) -> Option<i32> {
        self.int_val.remove(&var)
    }

    pub fn bool_iter(&self) -> btree_map::Iter<BoolVar, bool> {
        self.bool_val.iter()
    }

    pub fn int_iter(&self) -> btree_map::Iter<IntVar, i32> {
        self.int_val.iter()
    }

    pub fn eval_bool_expr(&self, expr: &BoolExpr) -> bool {
        match expr {
            BoolExpr::Const(b) => *b,
            BoolExpr::Var(v) => *(self.bool_val.get(v).unwrap()),
            &BoolExpr::NVar(_) => panic!(),
            BoolExpr::And(es) => {
                for e in es {
                    if !self.eval_bool_expr(e) {
                        return false;
                    }
                }
                true
            }
            BoolExpr::Or(es) => {
                for e in es {
                    if self.eval_bool_expr(e) {
                        return true;
                    }
                }
                false
            }
            BoolExpr::Not(e) => !self.eval_bool_expr(e),
            BoolExpr::Xor(e1, e2) => self.eval_bool_expr(e1) ^ self.eval_bool_expr(e2),
            BoolExpr::Iff(e1, e2) => self.eval_bool_expr(e1) == self.eval_bool_expr(e2),
            BoolExpr::Imp(e1, e2) => !self.eval_bool_expr(e1) || self.eval_bool_expr(e2),
            BoolExpr::Cmp(op, e1, e2) => {
                let v1 = self.eval_int_expr(e1);
                let v2 = self.eval_int_expr(e2);
                op.compare(v1, v2)
            }
        }
    }

    pub fn eval_int_expr(&self, expr: &IntExpr) -> i32 {
        match expr {
            IntExpr::Const(c) => *c,
            IntExpr::Var(v) => *(self.int_val.get(v).unwrap()),
            &IntExpr::NVar(_) => panic!(),
            IntExpr::Linear(es) => {
                let mut ret = 0i32;
                for (e, c) in es {
                    ret = ret
                        .checked_add(self.eval_int_expr(e).checked_mul(*c).unwrap())
                        .unwrap();
                }
                ret
            }
            IntExpr::If(c, t, f) => self.eval_int_expr(if self.eval_bool_expr(c) { t } else { f }),
            IntExpr::Abs(x) => self.eval_int_expr(x).abs(),
            IntExpr::Mul(x, y) => self.eval_int_expr(x) * self.eval_int_expr(y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_folding1() {
        let mut csp = CSP::new();

        let x = csp.new_bool_var();
        let y = csp.new_bool_var();
        let z = csp.new_bool_var();

        let mut expr = (x.expr() ^ y.expr()) | (y.expr().imp(z.expr()));
        csp.vars[y].set_infeasible(false); // y := true

        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, !x.expr() | z.expr());
    }

    #[test]
    fn test_constant_prop1() {
        let mut csp = CSP::new();

        let w = csp.new_bool_var();
        let x = csp.new_bool_var();
        let y = csp.new_bool_var();
        let z = csp.new_bool_var();

        csp.vars.constant_prop_bool(&(w.expr() & !x.expr()), true);
        assert!(!csp.vars[w].is_feasible(false));
        assert!(!csp.vars[x].is_feasible(true));

        let mut expr =
            (w.expr().iff(x.expr())) | ((x.expr() ^ y.expr()) | (w.expr().imp(z.expr())));

        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, y.expr() | z.expr());
    }
}
