use std::ops::Not;

#[cfg(feature = "backend-cadical")]
use crate::backend::cadical;
#[cfg(feature = "backend-external")]
use crate::backend::external;
use crate::backend::glucose;

use crate::custom_constraints::PropagatorGenerator;
use crate::propagators::graph_division::GraphDivisionOptions;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Var(pub(crate) i32);

impl Var {
    pub fn as_lit(self, negated: bool) -> Lit {
        Lit(self.0 * 2 + if negated { 1 } else { 0 })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lit(pub(crate) i32);

impl Lit {
    pub fn new(var: Var, negated: bool) -> Lit {
        Lit(var.0 * 2 + if negated { 1 } else { 0 })
    }

    pub fn var(self) -> Var {
        Var(self.0 / 2)
    }

    pub fn is_negated(self) -> bool {
        self.0 % 2 == 1
    }
}

impl Not for Lit {
    type Output = Lit;

    fn not(self) -> Self::Output {
        Lit(self.0 ^ 1)
    }
}

pub struct SATSolverStats {
    pub decisions: Option<u64>,
    pub propagations: Option<u64>,
    pub conflicts: Option<u64>,
}

/// Adapter to SAT solver.
/// To support other SAT solver without changing previous stages, we introduce an adapter instead of
/// using `glucose::Solver` directly from the encoder.
pub enum SAT {
    Glucose(glucose::Solver),
    #[cfg(feature = "backend-external")]
    External(external::Solver),
    #[cfg(feature = "backend-cadical")]
    CaDiCaL(cadical::Solver),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Glucose,
    External,
    CaDiCaL,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OrderEncodingLinearMode {
    Cpp,
    Rust,
    RustOptimized,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GraphDivisionMode {
    Cpp,
    Rust,
}

pub unsafe trait SolverManipulator {
    /// # Safety
    /// `lit` must be a valid literal associated with the solver.
    unsafe fn value(&self, lit: Lit) -> Option<bool>;

    /// # Safety
    /// `lit` must be a valid literal associated with the solver.
    unsafe fn add_watch(&mut self, lit: Lit);

    /// # Safety
    /// `lit` must be a valid literal associated with the solver.
    unsafe fn enqueue(&mut self, lit: Lit) -> bool;

    /// # Safety
    /// `lit` must be a valid literal associated with the solver.
    unsafe fn is_current_level(&self, lit: Lit) -> bool;
}

/// A trait for custom propagators.
///
/// # Safety
/// See the "Safety" section of `calc_reason`.
///
/// Note that, inappropriate implementation of this trait will be "safe" (i.e. not causing undefined
/// behavior) as long as it follows the conditions in the "Safety" section, but it can be
/// logically unsound.
/// TODO: is this really "safe"?
pub unsafe trait CustomPropagator<T: SolverManipulator> {
    /// Initializes the propagator.
    ///
    /// This function is called when the propagator is added to the solver.
    /// Then it should register the propagator to the watch list of any literals it uses by calling
    /// `add_watch`.
    /// It should return `true` if the propagator did not find any conflict, or `false` otherwise.
    fn initialize(&mut self, solver: &mut T) -> bool;

    /// Notifies the propagator of a new assignment.
    ///
    /// This function is called when a literal `p` is assigned to `true`.
    /// The propagator should check if the assignment so far is consistent with the constraints
    /// it represents.
    /// It should return `true` if the propagator did not find any conflict, or `false` otherwise.
    ///
    /// Also, this function should check for any immediate propagation. That is, if the propagator
    /// can deduce the value of any other literal, it should enqueue the literal using `enqueue`.
    ///
    /// `num_pending_propagations` is the number of pending propagations of this propagator.
    /// If this number is greater than 0, the propagator may skip checks for inconsistencies.
    /// This is useful for propagators that are expensive to check.
    fn propagate(&mut self, solver: &mut T, p: Lit, num_pending_propagations: i32) -> bool;

    /// Calculates the reason for a conflict.
    ///
    /// When this function is called, it is guaranteed that the propagator has already found
    /// a conflict or a propagation with the current decisions.
    ///
    /// - If both `p` and `extra` is `None`, there is a conflict in the current decision.
    ///   This function should return a vector of literals that causes the conflict.
    /// - If `p` is `None`, but `extra` is `None`, `!extra` could be propagated
    ///   from the current decision, but `extra` has already been added, causing a conflict.
    ///   This function should return a vector of literals that causes the conflict.
    ///   Note that this is NOT the reason why `!extra` is propagated. Thus, the returned vector
    ///   typically contains `extra` itself.
    /// - If `p` is not `None` and `extra` is `None`, `p` can be propagated from the current
    ///   decision, and `p` has already been `enqueue`'d.
    ///
    /// This function should return a vector of literals from which `p` can be propagated.
    ///
    /// # Safety
    /// The returned vector must contain only literals that are assigned to `true` at the
    /// moment of the propagation.
    /// It should be noted that, during `calc_reason`, `solver.value(lit)` may return `Some(...)`
    /// for `lit` whose assignment has been undone.
    /// Also, at least one literal in the returned vector must be from the current decision level.
    /// Violating this rule causes undefined behavior.
    fn calc_reason(&mut self, solver: &mut T, p: Option<Lit>, extra: Option<Lit>) -> Vec<Lit>;

    /// Notifies the propagator of a backtrack.
    ///
    /// This function is called when the solver undos the decision of a literal `p`.
    fn undo(&mut self, solver: &mut T, p: Lit);
}

impl SAT {
    pub fn new() -> SAT {
        SAT::new_glucose()
    }

    pub fn new_glucose() -> SAT {
        SAT::Glucose(glucose::Solver::new())
    }

    #[cfg(feature = "backend-external")]
    pub fn new_external() -> SAT {
        SAT::External(external::Solver::new())
    }

    #[cfg(feature = "backend-cadical")]
    pub fn new_cadical() -> SAT {
        SAT::CaDiCaL(cadical::Solver::new())
    }

    pub fn new_with_backend(backend: Backend) -> SAT {
        match backend {
            Backend::Glucose => SAT::new_glucose(),
            #[cfg(feature = "backend-external")]
            Backend::External => SAT::new_external(),
            #[cfg(not(feature = "backend-external"))]
            Backend::External => panic!("external backend is not enabled"),
            #[cfg(feature = "backend-cadical")]
            Backend::CaDiCaL => SAT::new_cadical(),
            #[cfg(not(feature = "backend-cadical"))]
            Backend::CaDiCaL => panic!("CaDiCaL backend is not enabled"),
        }
    }

    pub fn get_backend(&self) -> Backend {
        match self {
            SAT::Glucose(_) => Backend::Glucose,
            #[cfg(feature = "backend-external")]
            SAT::External(_) => Backend::External,
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(_) => Backend::CaDiCaL,
        }
    }

    pub fn num_var(&self) -> usize {
        match self {
            SAT::Glucose(solver) => solver.num_var() as usize,
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => solver.num_var() as usize,
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => solver.num_var() as usize,
        }
    }

    pub fn all_vars(&self) -> Vec<Var> {
        match self {
            SAT::Glucose(solver) => solver.all_vars(),
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => solver.all_vars(),
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => solver.all_vars(),
        }
    }

    #[cfg(feature = "sat-analyzer")]
    pub fn new_var(&mut self, name: &str) -> Var {
        match self {
            SAT::Glucose(solver) => solver.new_named_var(name),
            SAT::External(_) => panic!("new_var is not supported in external backend"),
            SAT::CaDiCaL(_) => panic!("new_var is not supported in cadical backend"),
        }
    }

    #[cfg(not(feature = "sat-analyzer"))]
    pub fn new_var(&mut self) -> Var {
        match self {
            SAT::Glucose(solver) => solver.new_var(),
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => solver.new_var(),
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => solver.new_var(),
        }
    }

    #[cfg(feature = "sat-analyzer")]
    pub fn new_vars(&mut self, count: usize, name: &str) -> Vec<Var> {
        let mut vars = vec![];
        for i in 0..count {
            vars.push(self.new_var(&format!("{}.{}", name, i)));
        }
        vars
    }

    #[cfg(not(feature = "sat-analyzer"))]
    pub fn new_vars(&mut self, count: usize) -> Vec<Var> {
        let mut vars = vec![];
        for _ in 0..count {
            vars.push(self.new_var());
        }
        vars
    }

    #[cfg(feature = "sat-analyzer")]
    pub fn new_vars_as_lits(&mut self, count: usize, name: &str) -> Vec<Lit> {
        let vars = self.new_vars(count, name);
        vars.iter().map(|v| v.as_lit(false)).collect()
    }

    #[cfg(not(feature = "sat-analyzer"))]
    pub fn new_vars_as_lits(&mut self, count: usize) -> Vec<Lit> {
        let vars = self.new_vars(count);
        vars.iter().map(|v| v.as_lit(false)).collect()
    }

    pub fn add_clause(&mut self, clause: &[Lit]) {
        match self {
            SAT::Glucose(solver) => {
                solver.add_clause(clause);
            }
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => {
                solver.add_clause(clause);
            }
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => {
                solver.add_clause(clause);
            }
        }
    }

    pub fn add_order_encoding_linear(
        &mut self,
        lits: Vec<Vec<Lit>>,
        domain: Vec<Vec<i32>>,
        coefs: Vec<i32>,
        constant: i32,
        mode: OrderEncodingLinearMode,
    ) -> bool {
        match self {
            SAT::Glucose(solver) => {
                solver.add_order_encoding_linear(&lits, &domain, &coefs, constant, mode)
            }
            #[cfg(feature = "backend-external")]
            SAT::External(_) => {
                panic!("add_order_encoding_linear is not supported in external backend")
            }
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(_) => todo!(),
        }
    }

    pub fn add_active_vertices_connected(
        &mut self,
        lits: Vec<Lit>,
        edges: Vec<(usize, usize)>,
    ) -> bool {
        match self {
            SAT::Glucose(solver) => solver.add_active_vertices_connected(&lits, &edges),
            #[cfg(feature = "backend-external")]
            SAT::External(_) => {
                panic!("add_active_vertices_connected is not supported in external backend")
            }
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => {
                solver.add_active_vertices_connected(&lits, &edges);
                true
            }
        }
    }

    #[cfg(not(feature = "csp-extra-constraints"))]
    pub fn add_direct_encoding_extension_supports(
        &mut self,
        _: &[Vec<Lit>],
        _: &[Vec<Option<usize>>],
    ) -> bool {
        panic!("feature not enabled");
    }

    #[cfg(feature = "csp-extra-constraints")]
    pub fn add_direct_encoding_extension_supports(
        &mut self,
        vars: &[Vec<Lit>],
        supports: &[Vec<Option<usize>>],
    ) -> bool {
        match self {
            SAT::Glucose(solver) => solver.add_direct_encoding_extension_supports(vars, supports),
            #[cfg(feature = "backend-external")]
            SAT::External(_) => panic!(
                "add_direct_encoding_extension_supports is not supported in external backend"
            ),
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(_) => todo!(),
        }
    }

    pub fn add_graph_division(
        &mut self,
        domains: &[Vec<i32>],
        dom_lits: &[Vec<Lit>],
        edges: &[(usize, usize)],
        edge_lits: &[Lit],
        mode: GraphDivisionMode,
        opts: &GraphDivisionOptions,
    ) -> bool {
        match self {
            SAT::Glucose(solver) => {
                solver.add_graph_division(domains, dom_lits, edges, edge_lits, mode, opts)
            }
            #[cfg(feature = "backend-external")]
            SAT::External(_) => panic!("add_graph_division is not supported in external backend"),
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(_) => todo!(),
        }
    }

    pub fn add_custom_constraint(
        &mut self,
        inputs: Vec<Lit>,
        constr: Box<dyn PropagatorGenerator>,
    ) -> bool {
        #[allow(unreachable_patterns)]
        match self {
            SAT::Glucose(solver) => {
                let propagator = constr.generate(inputs);
                solver.add_custom_constraint(propagator)
            }
            _ => todo!("add_custom_constraint is supported only in Glucose backend"),
        }
    }

    pub fn set_seed(&mut self, seed: f64) {
        match self {
            SAT::Glucose(solver) => solver.set_seed(seed),
            #[cfg(feature = "backend-external")]
            SAT::External(_) => (), // TODO: add warning
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(_) => (), // TODO
        }
    }

    pub fn set_rnd_init_act(&mut self, rnd_init_act: bool) {
        match self {
            SAT::Glucose(solver) => solver.set_rnd_init_act(rnd_init_act),
            #[cfg(feature = "backend-external")]
            SAT::External(_) => (), // TODO: add warning
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(_) => (), // TODO
        }
    }

    pub fn set_dump_analysis_info(&mut self, dump_analysis_info: bool) {
        match self {
            SAT::Glucose(solver) => solver.set_dump_analysis_info(dump_analysis_info),
            #[cfg(feature = "backend-external")]
            SAT::External(_) => (), // TODO: add warning
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(_) => (), // TODO: add warning
        }
    }

    pub fn solve(&mut self) -> Option<SATModel<'_>> {
        match self {
            SAT::Glucose(solver) => solver.solve().map(SATModel::Glucose),
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => solver.solve().map(SATModel::External),
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => solver.solve().map(SATModel::CaDiCaL),
        }
    }

    pub fn solve_without_model(&mut self) -> bool {
        match self {
            SAT::Glucose(solver) => solver.solve_without_model(),
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => solver.solve_without_model(),
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => solver.solve_without_model(),
        }
    }

    pub(crate) unsafe fn model(&self) -> SATModel<'_> {
        match self {
            SAT::Glucose(solver) => SATModel::Glucose(solver.model()),
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => SATModel::External(solver.model()),
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => SATModel::CaDiCaL(solver.model()),
        }
    }

    pub fn stats(&self) -> SATSolverStats {
        match self {
            SAT::Glucose(solver) => SATSolverStats {
                decisions: Some(solver.stats_decisions()),
                propagations: Some(solver.stats_propagations()),
                conflicts: Some(solver.stats_conflicts()),
            },
            #[cfg(feature = "backend-external")]
            SAT::External(_) => SATSolverStats {
                decisions: None,
                propagations: None,
                conflicts: None,
            },
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(_) => SATSolverStats {
                decisions: None,
                propagations: None,
                conflicts: None,
            }, // TODO
        }
    }
}

pub enum SATModel<'a> {
    Glucose(glucose::Model<'a>),
    #[cfg(feature = "backend-external")]
    External(external::Model<'a>),
    #[cfg(feature = "backend-cadical")]
    CaDiCaL(cadical::Model<'a>),
}

impl SATModel<'_> {
    pub fn assignment(&self, var: Var) -> bool {
        match self {
            SATModel::Glucose(model) => model.assignment(var),
            #[cfg(feature = "backend-external")]
            SATModel::External(model) => model.assignment(var),
            #[cfg(feature = "backend-cadical")]
            SATModel::CaDiCaL(model) => model.assignment(var),
        }
    }

    pub fn assignment_lit(&self, lit: Lit) -> bool {
        self.assignment(lit.var()) ^ lit.is_negated()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RandomPropagator {
        vars: Vec<Var>,
        num_decisions: usize,
        counter: u32,
        reason_inconsistency: Vec<Lit>,
        reason_propagate: std::collections::BTreeMap<Lit, Vec<Lit>>,
    }

    impl RandomPropagator {
        fn random_value(&mut self) -> f64 {
            self.counter += 1;
            self.counter.wrapping_mul(0x87654321) as f64 / 0xFFFFFFFFu32 as f64
        }

        fn fake_reason<T: SolverManipulator>(&mut self, solver: &mut T) -> Vec<Lit> {
            let mut cur_level = vec![];
            let mut not_cur_level = vec![];

            for &var in &self.vars {
                for neg in [false, true] {
                    let lit = var.as_lit(neg);
                    if unsafe { solver.value(lit) } != Some(true) {
                        continue;
                    }
                    if unsafe { solver.is_current_level(lit) } {
                        cur_level.push(lit);
                    } else {
                        not_cur_level.push(lit);
                    }
                }
            }

            let mut ret = vec![];
            ret.push(cur_level[(self.random_value() * cur_level.len() as f64) as usize]);

            for &lit in &not_cur_level {
                if self.random_value() < 0.5 {
                    ret.push(lit);
                }
            }
            ret
        }
    }

    unsafe impl<T: SolverManipulator> CustomPropagator<T> for RandomPropagator {
        fn initialize(&mut self, solver: &mut T) -> bool {
            for &var in &self.vars {
                unsafe {
                    solver.add_watch(var.as_lit(false));
                    solver.add_watch(var.as_lit(true));
                }
            }
            true
        }

        fn propagate(&mut self, solver: &mut T, _p: Lit, _num_pending_propagations: i32) -> bool {
            self.num_decisions += 1;

            if self.num_decisions == 0 {
                return true;
            }

            for var in self.vars.clone() {
                let lit = var.as_lit(false);
                if unsafe { solver.value(lit) }.is_some() {
                    continue;
                }

                if self.random_value() < 0.01 {
                    let reason = self.fake_reason(solver);
                    self.reason_propagate.insert(lit, reason.clone());
                    assert!(unsafe { solver.enqueue(lit) });
                }
            }

            if self.random_value() < 0.01 {
                let reason = self.fake_reason(solver);
                self.reason_inconsistency = reason;
                false
            } else {
                true
            }
        }

        fn calc_reason(&mut self, _solver: &mut T, p: Option<Lit>, extra: Option<Lit>) -> Vec<Lit> {
            assert!(extra.is_none());
            if let Some(p) = p {
                self.reason_propagate.get(&p).unwrap().clone()
            } else {
                self.reason_inconsistency.clone()
            }
        }

        fn undo(&mut self, _solver: &mut T, _p: Lit) {
            self.num_decisions -= 1;
        }
    }

    #[test]
    fn test_random_propagator() {
        let mut solver = crate::backend::glucose::Solver::new();
        let mut vars = vec![];
        let n_vars = 14;

        for _ in 0..n_vars {
            vars.push(solver.new_var());
        }

        assert!(solver.add_custom_constraint(Box::new(RandomPropagator {
            vars: vars.clone(),
            num_decisions: 0,
            counter: 0,
            reason_inconsistency: vec![],
            reason_propagate: std::collections::BTreeMap::new(),
        })));

        let mut n_assignments = 0;
        loop {
            match solver.solve() {
                Some(model) => {
                    n_assignments += 1;
                    let mut new_clause = vec![];
                    for &v in &vars {
                        new_clause.push(v.as_lit(model.assignment(v)));
                    }
                    solver.add_clause(&new_clause);
                }
                None => break,
            }
        }

        assert!(n_assignments < (1 << n_vars));
    }
}
