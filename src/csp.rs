use crate::arithmetic::CheckedInt;
use crate::util::{ConvertMapIndex, UpdateStatus};
use std::collections::{btree_map, BTreeMap};
use std::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};

use super::CmpOp;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Domain {
    low: CheckedInt,
    high: CheckedInt,
}

impl Domain {
    pub fn range(low: i32, high: i32) -> Domain {
        Domain {
            low: CheckedInt::new(low),
            high: CheckedInt::new(high),
        }
    }

    pub(crate) fn range_from_checked(low: CheckedInt, high: CheckedInt) -> Domain {
        Domain { low, high }
    }

    pub(crate) fn enumerate(&self) -> Vec<CheckedInt> {
        (self.low.get()..=self.high.get())
            .map(CheckedInt::new)
            .collect::<Vec<_>>()
    }

    pub(crate) fn lower_bound_checked(&self) -> CheckedInt {
        self.low
    }

    pub(crate) fn upper_bound_checked(&self) -> CheckedInt {
        self.high
    }

    pub(crate) fn as_constant(&self) -> Option<CheckedInt> {
        if self.low == self.high {
            Some(self.low)
        } else {
            None
        }
    }

    pub fn is_infeasible(&self) -> bool {
        self.low > self.high
    }

    pub(crate) fn refine_upper_bound(&mut self, v: CheckedInt) -> UpdateStatus {
        if self.high <= v {
            UpdateStatus::NotUpdated
        } else {
            self.high = v;
            if self.is_infeasible() {
                UpdateStatus::Unsatisfiable
            } else {
                UpdateStatus::Updated
            }
        }
    }

    pub(crate) fn refine_lower_bound(&mut self, v: CheckedInt) -> UpdateStatus {
        if self.low >= v {
            UpdateStatus::NotUpdated
        } else {
            self.low = v;
            if self.is_infeasible() {
                UpdateStatus::Unsatisfiable
            } else {
                UpdateStatus::Updated
            }
        }
    }
}

impl Add<Domain> for Domain {
    type Output = Domain;

    fn add(self, rhs: Domain) -> Domain {
        Domain::range_from_checked(self.low + rhs.low, self.high + rhs.high)
    }
}

impl Mul<CheckedInt> for Domain {
    type Output = Domain;

    fn mul(self, rhs: CheckedInt) -> Domain {
        if rhs == 0 {
            Domain::range(0, 0)
        } else if rhs > 0 {
            Domain::range_from_checked(self.low * rhs, self.high * rhs)
        } else {
            Domain::range_from_checked(self.high * rhs, self.low * rhs)
        }
    }
}

impl BitOr<Domain> for Domain {
    type Output = Domain;

    fn bitor(self, rhs: Domain) -> Domain {
        Domain::range_from_checked(self.low.min(rhs.low), self.high.max(rhs.high))
    }
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
        IntVarData {
            domain: domain.clone(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct BoolVar(usize);

impl BoolVar {
    pub fn expr(self) -> BoolExpr {
        BoolExpr::Var(self)
    }
}

impl ConvertMapIndex for BoolVar {
    fn to_index(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct IntVar(usize);

impl IntVar {
    pub fn expr(self) -> IntExpr {
        IntExpr::Var(self)
    }
}

impl ConvertMapIndex for IntVar {
    fn to_index(&self) -> usize {
        self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Stmt {
    Expr(BoolExpr),
    AllDifferent(Vec<IntExpr>),
    ActiveVerticesConnected(Vec<BoolExpr>, Vec<(usize, usize)>),
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum BoolExpr {
    Const(bool),
    Var(BoolVar),
    NVar(super::norm_csp::BoolVar),
    And(Vec<Box<BoolExpr>>),
    Or(Vec<Box<BoolExpr>>),
    Not(Box<BoolExpr>),
    Xor(Box<BoolExpr>, Box<BoolExpr>),
    Iff(Box<BoolExpr>, Box<BoolExpr>),
    Imp(Box<BoolExpr>, Box<BoolExpr>),
    Cmp(CmpOp, Box<IntExpr>, Box<IntExpr>),
}

impl BoolExpr {
    pub fn imp(self, rhs: BoolExpr) -> BoolExpr {
        BoolExpr::Imp(Box::new(self), Box::new(rhs))
    }

    pub fn iff(self, rhs: BoolExpr) -> BoolExpr {
        BoolExpr::Iff(Box::new(self), Box::new(rhs))
    }

    pub fn ite(self, t: IntExpr, f: IntExpr) -> IntExpr {
        IntExpr::If(Box::new(self), Box::new(t), Box::new(f))
    }

    fn is_const(&self) -> Option<bool> {
        match self {
            &BoolExpr::Const(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_var(&self) -> Option<BoolVar> {
        match self {
            &BoolExpr::Var(v) => Some(v),
            _ => None,
        }
    }
}

impl BitAnd<BoolExpr> for BoolExpr {
    type Output = BoolExpr;

    fn bitand(self, rhs: BoolExpr) -> Self::Output {
        BoolExpr::And(vec![Box::new(self), Box::new(rhs)])
    }
}

impl BitOr<BoolExpr> for BoolExpr {
    type Output = BoolExpr;

    fn bitor(self, rhs: BoolExpr) -> Self::Output {
        BoolExpr::Or(vec![Box::new(self), Box::new(rhs)])
    }
}

impl BitXor<BoolExpr> for BoolExpr {
    type Output = BoolExpr;

    fn bitxor(self, rhs: BoolExpr) -> Self::Output {
        BoolExpr::Xor(Box::new(self), Box::new(rhs))
    }
}

impl Not for BoolExpr {
    type Output = BoolExpr;

    fn not(self) -> Self::Output {
        BoolExpr::Not(Box::new(self))
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum IntExpr {
    Const(i32),
    Var(IntVar),
    NVar(super::norm_csp::IntVar),
    Linear(Vec<(Box<IntExpr>, i32)>),
    If(Box<BoolExpr>, Box<IntExpr>, Box<IntExpr>),
}

impl IntExpr {
    pub fn eq(self, rhs: IntExpr) -> BoolExpr {
        BoolExpr::Cmp(CmpOp::Eq, Box::new(self), Box::new(rhs))
    }

    pub fn ne(self, rhs: IntExpr) -> BoolExpr {
        BoolExpr::Cmp(CmpOp::Ne, Box::new(self), Box::new(rhs))
    }

    pub fn le(self, rhs: IntExpr) -> BoolExpr {
        BoolExpr::Cmp(CmpOp::Le, Box::new(self), Box::new(rhs))
    }

    pub fn lt(self, rhs: IntExpr) -> BoolExpr {
        BoolExpr::Cmp(CmpOp::Lt, Box::new(self), Box::new(rhs))
    }

    pub fn ge(self, rhs: IntExpr) -> BoolExpr {
        BoolExpr::Cmp(CmpOp::Ge, Box::new(self), Box::new(rhs))
    }

    pub fn gt(self, rhs: IntExpr) -> BoolExpr {
        BoolExpr::Cmp(CmpOp::Gt, Box::new(self), Box::new(rhs))
    }
}

impl Add<IntExpr> for IntExpr {
    type Output = IntExpr;

    fn add(self, rhs: IntExpr) -> IntExpr {
        IntExpr::Linear(vec![(Box::new(self), 1), (Box::new(rhs), 1)])
    }
}

impl Sub<IntExpr> for IntExpr {
    type Output = IntExpr;

    fn sub(self, rhs: IntExpr) -> IntExpr {
        IntExpr::Linear(vec![(Box::new(self), 1), (Box::new(rhs), -1)])
    }
}

impl Mul<i32> for IntExpr {
    type Output = IntExpr;

    fn mul(self, rhs: i32) -> IntExpr {
        IntExpr::Linear(vec![(Box::new(self), rhs)])
    }
}

pub(super) struct CSPVars {
    // TODO: remove `pub(super)`
    pub(super) bool_var: Vec<BoolVarData>,
    pub(super) int_var: Vec<IntVarData>,
}

impl CSPVars {
    pub(super) fn bool_vars_iter(&self) -> impl Iterator<Item = BoolVar> {
        (0..self.bool_var.len()).map(|x| BoolVar(x))
    }

    pub(super) fn int_vars_iter(&self) -> impl Iterator<Item = IntVar> {
        (0..self.int_var.len()).map(|x| IntVar(x))
    }

    pub(super) fn int_var(&self, var: IntVar) -> &IntVarData {
        &self.int_var[var.0]
    }

    fn constant_folding_bool(&self, expr: &mut BoolExpr) {
        match expr {
            BoolExpr::Const(_) => (),
            BoolExpr::Var(v) => {
                let value = &self.bool_var[v.0];
                if !value.is_feasible(true) && value.is_feasible(false) {
                    *expr = BoolExpr::Const(false)
                } else if value.is_feasible(true) && !value.is_feasible(false) {
                    *expr = BoolExpr::Const(true)
                } else if value.is_unsatisfiable() {
                    panic!(); // this should be handled when the inconsistency first occurred.
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
            &BoolExpr::Var(v) => self.bool_var[v.0].set_infeasible(!expected),
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

pub enum BoolVarStatus {
    Infeasible,
    Fixed(bool),
    Unfixed,
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
        }
    }

    pub fn new_bool_var(&mut self) -> BoolVar {
        let id = self.vars.bool_var.len();
        self.vars.bool_var.push(BoolVarData::new());
        BoolVar(id)
    }

    pub fn new_int_var(&mut self, domain: Domain) -> IntVar {
        let id = self.vars.int_var.len();
        self.vars.int_var.push(IntVarData::new(domain));
        IntVar(id)
    }

    pub fn add_constraint(&mut self, stmt: Stmt) {
        self.constraints.push(stmt);
    }

    pub fn is_inconsistent(&self) -> bool {
        self.inconsistent
    }

    pub fn get_bool_var_status(&self, var: BoolVar) -> BoolVarStatus {
        let data = &self.vars.bool_var[var.0];
        match (data.is_feasible(false), data.is_feasible(true)) {
            (false, false) => BoolVarStatus::Infeasible,
            (false, true) => BoolVarStatus::Fixed(true),
            (true, false) => BoolVarStatus::Fixed(false),
            (true, true) => BoolVarStatus::Unfixed,
        }
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
            }
        }
    }

    pub fn optimize(&mut self, use_propagate: bool) {
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
                match *op {
                    CmpOp::Eq => v1 == v2,
                    CmpOp::Ne => v1 != v2,
                    CmpOp::Le => v1 <= v2,
                    CmpOp::Lt => v1 < v2,
                    CmpOp::Ge => v1 >= v2,
                    CmpOp::Gt => v1 > v2,
                }
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
        csp.vars.bool_var[y.0].set_infeasible(false); // y := true

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
        assert!(!csp.vars.bool_var[w.0].is_feasible(false));
        assert!(!csp.vars.bool_var[x.0].is_feasible(true));

        let mut expr =
            (w.expr().iff(x.expr())) | ((x.expr() ^ y.expr()) | (w.expr().imp(z.expr())));

        csp.vars.constant_folding_bool(&mut expr);
        assert_eq!(expr, y.expr() | z.expr());
    }
}
