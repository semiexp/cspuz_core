mod repr;

#[cfg(test)]
pub mod test_utils;

use crate::arithmetic::CheckedInt;
use crate::domain::Domain;
use crate::util::{ConvertMapIndex, UpdateStatus};
use std::collections::{btree_map, BTreeMap};
use std::ops::{Index, IndexMut};

pub use repr::{BoolExpr, BoolVar, IntExpr, IntVar, Stmt};

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
        (0..self.bool_var.len()).map(BoolVar::new)
    }

    pub(super) fn int_vars_iter(&self) -> impl Iterator<Item = IntVar> {
        (0..self.int_var.len()).map(IntVar::new)
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
                    if exprs.is_empty() {
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
                    if exprs.is_empty() {
                        *expr = BoolExpr::Const(false);
                    } else if exprs.len() == 1 {
                        *expr = *exprs.remove(0);
                    }
                }
            }
            BoolExpr::Not(e) => {
                self.constant_folding_bool(e);
                if let Some(b) = e.is_const() {
                    *expr = BoolExpr::Const(!b);
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
                if terms.is_empty() {
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

                if let (IntExpr::Const(a), IntExpr::Const(b)) = (x.as_ref(), y.as_ref()) {
                    *expr = IntExpr::Const(a * b);
                } else if matches!(x.as_ref(), IntExpr::Const(0))
                    || matches!(y.as_ref(), IntExpr::Const(0))
                {
                    *expr = IntExpr::Const(0);
                } else if let IntExpr::Const(c) = x.as_ref() {
                    let y_val = std::mem::replace(y.as_mut(), IntExpr::Const(0));
                    *expr = IntExpr::Linear(vec![(Box::new(y_val), *c)]);
                } else if let IntExpr::Const(c) = y.as_ref() {
                    let x_val = std::mem::replace(x.as_mut(), IntExpr::Const(0));
                    *expr = IntExpr::Linear(vec![(Box::new(x_val), *c)]);
                }
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
        assert!(!domain_list.is_empty());
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
                        if let Some(e) = e {
                            vars.constant_folding_int(e);
                        }
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
                    if let Stmt::Expr(e) = stmt {
                        update_status |= vars.constant_prop_bool(e, true);
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
}

#[cfg(test)]
mod tests {
    use crate::propagators::graph_division::GraphDivisionOptions;

    use super::*;

    #[test]
    fn test_csp() {
        let mut csp = CSP::new();

        let n = csp.new_int_var_from_list(vec![
            CheckedInt::new(4),
            CheckedInt::new(2),
            CheckedInt::new(3),
            CheckedInt::new(2),
        ]);
        assert_eq!(csp.vars[n].domain, Domain::enumerative(vec![2, 3, 4]));
    }

    #[test]
    fn test_constant_folding_bool() {
        let mut csp = CSP::new();

        let x = csp.new_bool_var();
        let y = csp.new_bool_var();
        let t = csp.new_bool_var();
        let f = csp.new_bool_var();

        csp.vars[t].set_infeasible(false); // a := true
        csp.vars[f].set_infeasible(true); // b := false

        // and
        let mut expr = x.expr() & y.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr() & y.expr());

        let mut expr = x.expr() & t.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = t.expr() & x.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = x.expr() & f.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(false));

        let mut expr = f.expr() & x.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(false));

        let mut expr = BoolExpr::Const(true) & BoolExpr::Const(true);
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(true));

        let mut expr = BoolExpr::Const(true) & BoolExpr::Const(false);
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(false));

        // or
        let mut expr = x.expr() | y.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr() | y.expr());

        let mut expr = x.expr() | t.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(true));

        let mut expr = t.expr() | x.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(true));

        let mut expr = x.expr() | f.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = f.expr() | x.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = BoolExpr::Const(false) | BoolExpr::Const(false);
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(false));

        let mut expr = BoolExpr::Const(true) | BoolExpr::Const(false);
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(true));

        // not
        let mut expr = !x.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, !x.expr());

        let mut expr = !t.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(false));

        // xor
        let mut expr = x.expr() ^ y.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr() ^ y.expr());

        let mut expr = x.expr() ^ t.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, !x.expr());

        let mut expr = t.expr() ^ x.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, !x.expr());

        let mut expr = x.expr() ^ f.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = f.expr() ^ x.expr();
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = BoolExpr::Const(true) ^ BoolExpr::Const(true);
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(false));

        // iff
        let mut expr = x.expr().iff(y.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr().iff(y.expr()));

        let mut expr = x.expr().iff(t.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = t.expr().iff(x.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = x.expr().iff(f.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, !x.expr());

        let mut expr = f.expr().iff(x.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, !x.expr());

        let mut expr = BoolExpr::Const(true).iff(BoolExpr::Const(true));
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(true));

        // imp
        let mut expr = x.expr().imp(y.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr().imp(y.expr()));

        let mut expr = x.expr().imp(t.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(true));

        let mut expr = t.expr().imp(x.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, x.expr());

        let mut expr = x.expr().imp(f.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, !x.expr());

        let mut expr = f.expr().imp(x.expr());
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(true));

        let mut expr = BoolExpr::Const(true).imp(BoolExpr::Const(false));
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, BoolExpr::Const(false));

        // cmp
        let mut expr = BoolExpr::Cmp(
            crate::arithmetic::CmpOp::Eq,
            Box::new(x.expr().ite(IntExpr::Const(1), IntExpr::Const(0))),
            Box::new((y.expr() | f.expr()).ite(IntExpr::Const(1), IntExpr::Const(0))),
        );
        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(
            expr,
            BoolExpr::Cmp(
                crate::arithmetic::CmpOp::Eq,
                Box::new(x.expr().ite(IntExpr::Const(1), IntExpr::Const(0))),
                Box::new(y.expr().ite(IntExpr::Const(1), IntExpr::Const(0))),
            )
        );
    }

    #[test]
    fn test_constant_folding_int() {
        let mut csp = CSP::new();

        let a = csp.new_int_var(Domain::range(0, 5));
        let n = csp.new_int_var(Domain::range(1, 1));
        let x = csp.new_bool_var();

        // var
        let mut expr = a.expr();
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, a.expr());

        // linear
        let mut expr = IntExpr::Linear(vec![]);
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, IntExpr::Const(0));

        let mut expr = IntExpr::Linear(vec![(Box::new(a.expr()), 1)]);
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, a.expr());

        // if
        let mut expr = x.expr().ite(a.expr(), n.expr());
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, x.expr().ite(a.expr(), IntExpr::Const(1)));

        let mut expr = BoolExpr::Const(true).ite(a.expr(), n.expr());
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, a.expr());

        let mut expr = BoolExpr::Const(false).ite(a.expr(), n.expr());
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, IntExpr::Const(1));

        // abs
        let mut expr = x.expr().ite(a.expr(), n.expr()).abs();
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, x.expr().ite(a.expr(), IntExpr::Const(1)).abs());

        // mul
        let mut expr = x.expr().ite(a.expr(), n.expr()) * a.expr();
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, x.expr().ite(a.expr(), IntExpr::Const(1)) * a.expr());

        let mut expr = IntExpr::Const(4) * IntExpr::Const(2);
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, IntExpr::Const(8));

        let mut expr = IntExpr::Const(4) * a.expr();
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, IntExpr::Linear(vec![(Box::new(a.expr()), 4)]));

        let mut expr = a.expr() * IntExpr::Const(2);
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, IntExpr::Linear(vec![(Box::new(a.expr()), 2)]));

        let mut expr = IntExpr::Const(0) * a.expr();
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, IntExpr::Const(0));

        let mut expr = a.expr() * IntExpr::Const(0);
        csp.vars.constant_folding_int(&mut expr);
        assert_eq!(expr, IntExpr::Const(0));
    }

    #[test]
    fn test_constant_prop_bool() {
        // const
        {
            let mut csp = CSP::new();
            assert_eq!(
                csp.vars.constant_prop_bool(&BoolExpr::Const(true), true),
                UpdateStatus::NotUpdated
            );
        }
        {
            let mut csp = CSP::new();
            assert_eq!(
                csp.vars.constant_prop_bool(&BoolExpr::Const(false), false),
                UpdateStatus::NotUpdated
            );
        }
        {
            let mut csp = CSP::new();
            assert_eq!(
                csp.vars.constant_prop_bool(&BoolExpr::Const(true), false),
                UpdateStatus::Unsatisfiable
            );
        }
        {
            let mut csp = CSP::new();
            assert_eq!(
                csp.vars.constant_prop_bool(&BoolExpr::Const(false), true),
                UpdateStatus::Unsatisfiable
            );
        }

        // var
        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&x.expr(), false),
                UpdateStatus::Updated
            );
            assert!(csp.vars[x].is_feasible(false));
            assert!(!csp.vars[x].is_feasible(true));

            assert_eq!(
                csp.vars.constant_prop_bool(&y.expr(), true),
                UpdateStatus::Updated
            );
            assert!(!csp.vars[y].is_feasible(false));
            assert!(csp.vars[y].is_feasible(true));
        }

        // and
        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&(x.expr() & y.expr()), true),
                UpdateStatus::Updated
            );
            assert!(!csp.vars[x].is_feasible(false));
            assert!(csp.vars[x].is_feasible(true));
            assert!(!csp.vars[y].is_feasible(false));
            assert!(csp.vars[y].is_feasible(true));
        }

        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&(x.expr() & y.expr()), false),
                UpdateStatus::NotUpdated
            );
            assert!(csp.vars[x].is_feasible(false));
            assert!(csp.vars[x].is_feasible(true));
            assert!(csp.vars[y].is_feasible(false));
            assert!(csp.vars[y].is_feasible(true));
        }

        // or
        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&(x.expr() | y.expr()), true),
                UpdateStatus::NotUpdated
            );
            assert!(csp.vars[x].is_feasible(false));
            assert!(csp.vars[x].is_feasible(true));
            assert!(csp.vars[y].is_feasible(false));
            assert!(csp.vars[y].is_feasible(true));
        }

        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&(x.expr() | y.expr()), false),
                UpdateStatus::Updated
            );
            assert!(csp.vars[x].is_feasible(false));
            assert!(!csp.vars[x].is_feasible(true));
            assert!(csp.vars[y].is_feasible(false));
            assert!(!csp.vars[y].is_feasible(true));
        }

        // not
        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&!x.expr(), true),
                UpdateStatus::Updated
            );
            assert!(csp.vars[x].is_feasible(false));
            assert!(!csp.vars[x].is_feasible(true));
        }

        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&!x.expr(), false),
                UpdateStatus::Updated
            );
            assert!(!csp.vars[x].is_feasible(false));
            assert!(csp.vars[x].is_feasible(true));
        }

        // imp
        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&(x.expr().imp(y.expr())), true),
                UpdateStatus::NotUpdated
            );
            assert!(csp.vars[x].is_feasible(false));
            assert!(csp.vars[x].is_feasible(true));
            assert!(csp.vars[y].is_feasible(false));
            assert!(csp.vars[y].is_feasible(true));
        }

        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars
                    .constant_prop_bool(&(x.expr().imp(y.expr())), false),
                UpdateStatus::Updated
            );
            assert!(!csp.vars[x].is_feasible(false));
            assert!(csp.vars[x].is_feasible(true));
            assert!(csp.vars[y].is_feasible(false));
            assert!(!csp.vars[y].is_feasible(true));
        }

        // xor
        for b in [false, true] {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&(x.expr() ^ y.expr()), b),
                UpdateStatus::NotUpdated
            );
            assert!(csp.vars[x].is_feasible(false));
            assert!(csp.vars[x].is_feasible(true));
            assert!(csp.vars[y].is_feasible(false));
            assert!(csp.vars[y].is_feasible(true));
        }

        // iff
        for b in [false, true] {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            assert_eq!(
                csp.vars.constant_prop_bool(&(x.expr().iff(y.expr())), b),
                UpdateStatus::NotUpdated
            );
            assert!(csp.vars[x].is_feasible(false));
            assert!(csp.vars[x].is_feasible(true));
            assert!(csp.vars[y].is_feasible(false));
            assert!(csp.vars[y].is_feasible(true));
        }
    }

    #[test]
    fn test_vars_iter() {
        let mut csp = CSP::new();

        let x = csp.new_bool_var();
        let y = csp.new_bool_var();
        let z = csp.new_bool_var();

        let a = csp.new_int_var(Domain::range(0, 5));
        let b = csp.new_int_var(Domain::range(1, 10));

        assert_eq!(csp.vars.bool_vars_iter().collect::<Vec<_>>(), vec![x, y, z]);
        assert_eq!(csp.vars.int_vars_iter().collect::<Vec<_>>(), vec![a, b]);
    }

    #[test]
    fn test_vars_index() {
        let mut csp = CSP::new();

        let x = csp.new_bool_var();
        let y = csp.new_bool_var();

        let a = csp.new_int_var(Domain::range(0, 5));
        let b = csp.new_int_var(Domain::range(1, 10));

        csp.vars.constant_prop_bool(&y.expr(), false);

        assert_eq!(csp.vars[x].possibility_mask, 3);
        assert_eq!(csp.vars[y].possibility_mask, 1);
        assert_eq!(csp.vars[a].domain, Domain::range(0, 5));
        assert_eq!(csp.vars[b].domain, Domain::range(1, 10));
        csp.vars[b].domain.refine_lower_bound(CheckedInt::new(3));
        assert_eq!(csp.vars[b].domain, Domain::range(3, 10));
    }

    #[test]
    fn test_csp_constant_folding() {
        let mut csp = CSP::new();

        let x = csp.new_bool_var();
        let y = csp.new_bool_var();
        let a = csp.new_int_var(Domain::range(0, 5));

        csp.add_constraint(Stmt::Expr(x.expr() | BoolExpr::Const(false)));
        csp.add_constraint(Stmt::AllDifferent(vec![a.expr(), IntExpr::Linear(vec![])]));
        csp.add_constraint(Stmt::ActiveVerticesConnected(
            vec![x.expr() | BoolExpr::Const(false), y.expr()],
            vec![(0, 1)],
        ));
        csp.add_constraint(Stmt::Circuit(vec![a.expr(), IntExpr::Linear(vec![])]));
        csp.add_constraint(Stmt::ExtensionSupports(
            vec![a.expr(), IntExpr::Linear(vec![])],
            vec![],
        ));
        csp.add_constraint(Stmt::GraphDivision(
            vec![Some(a.expr()), Some(IntExpr::Linear(vec![])), None],
            vec![(0, 1)],
            vec![x.expr() | BoolExpr::Const(false)],
            GraphDivisionOptions::default(),
        ));

        csp.apply_constant_folding();

        assert!(matches!(&csp.constraints[0], Stmt::Expr(e) if e == &x.expr()));
        assert!(
            matches!(&csp.constraints[1], Stmt::AllDifferent(e) if e == &[a.expr(), IntExpr::Const(0)])
        );
        assert!(
            matches!(&csp.constraints[2], Stmt::ActiveVerticesConnected(e, _) if e == &[x.expr(), y.expr()])
        );
        assert!(
            matches!(&csp.constraints[3], Stmt::Circuit(e) if e == &[a.expr(), IntExpr::Const(0)])
        );
        assert!(
            matches!(&csp.constraints[4], Stmt::ExtensionSupports(e, _) if e == &[a.expr(), IntExpr::Const(0)])
        );
        assert!(
            matches!(&csp.constraints[5], Stmt::GraphDivision(s, _, e, _) if s == &[Some(a.expr()), Some(IntExpr::Const(0)), None] && e == &[x.expr()])
        );
    }

    #[test]
    fn test_csp_optimize() {
        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();
            let z = csp.new_bool_var();

            csp.add_constraint(Stmt::Expr(x.expr() | y.expr()));
            csp.add_constraint(Stmt::Expr(!y.expr() | z.expr()));
            csp.add_constraint(Stmt::Expr(!z.expr()));

            csp.optimize(false, true);

            assert!(!csp.is_inconsistent());
            assert!(matches!(&csp.constraints[0], Stmt::Expr(e) if e == &(x.expr() | y.expr())));
            assert!(matches!(&csp.constraints[1], Stmt::Expr(e) if e == &(!y.expr() | z.expr())));
            assert!(matches!(&csp.constraints[2], Stmt::Expr(e) if e == &(!z.expr())));
        }

        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();
            let z = csp.new_bool_var();

            csp.add_constraint(Stmt::Expr(x.expr() | y.expr()));
            csp.add_constraint(Stmt::Expr(!y.expr() | z.expr()));
            csp.add_constraint(Stmt::Expr(!z.expr()));

            csp.optimize(true, true);

            assert!(!csp.is_inconsistent());
            assert!(matches!(&csp.constraints[0], Stmt::Expr(e) if e == &BoolExpr::Const(true)));
            assert!(matches!(&csp.constraints[1], Stmt::Expr(e) if e == &BoolExpr::Const(true)));
            assert!(matches!(&csp.constraints[2], Stmt::Expr(e) if e == &BoolExpr::Const(true)));
            assert!(matches!(
                csp.get_bool_var_status(x),
                BoolVarStatus::Fixed(true)
            ));
            assert!(matches!(
                csp.get_bool_var_status(y),
                BoolVarStatus::Fixed(false)
            ));
            assert!(matches!(
                csp.get_bool_var_status(z),
                BoolVarStatus::Fixed(false)
            ));
        }

        {
            let mut csp = CSP::new();

            let x = csp.new_bool_var();
            let y = csp.new_bool_var();

            csp.add_constraint(Stmt::Expr(x.expr() & y.expr()));
            csp.add_constraint(Stmt::Expr(!x.expr() | !y.expr()));

            csp.optimize(true, true);

            assert!(csp.is_inconsistent());
        }
    }

    #[test]
    fn test_assignment() {
        let mut assignment = Assignment::new();

        let x = BoolVar::new(0);
        let y = BoolVar::new(1);
        let z = BoolVar::new(2);
        let a = IntVar::new(0);
        let b = IntVar::new(1);
        let c = IntVar::new(2);

        assignment.set_bool(x, true);
        assignment.set_bool(y, false);
        assignment.set_int(a, 42);
        assignment.set_int(b, 100);

        assert_eq!(
            assignment.bool_iter().collect::<Vec<_>>(),
            vec![(&x, &true), (&y, &false)]
        );
        assert_eq!(
            assignment.int_iter().collect::<Vec<_>>(),
            vec![(&a, &42), (&b, &100)]
        );

        assert_eq!(assignment.get_bool(x), Some(true));
        assert_eq!(assignment.get_bool(y), Some(false));
        assert_eq!(assignment.get_bool(z), None);

        assert_eq!(assignment.get_int(a), Some(42));
        assert_eq!(assignment.get_int(b), Some(100));
        assert_eq!(assignment.get_int(c), None);

        assert_eq!(assignment.remove_bool(x), Some(true));
        assert_eq!(assignment.remove_bool(x), None);
        assert_eq!(assignment.get_bool(x), None);

        assert_eq!(assignment.remove_int(a), Some(42));
        assert_eq!(assignment.remove_int(a), None);
        assert_eq!(assignment.get_int(a), None);
    }
}
