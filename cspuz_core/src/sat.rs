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
    unsafe fn value(&self, lit: Lit) -> Option<bool>;
    unsafe fn add_watch(&mut self, lit: Lit);
    unsafe fn enqueue(&mut self, lit: Lit) -> bool;
    unsafe fn is_current_level(&self, lit: Lit) -> bool;
}

pub unsafe trait CustomPropagator<T: SolverManipulator> {
    fn initialize(&mut self, solver: &mut T) -> bool;
    fn propagate(&mut self, solver: &mut T, p: Lit, num_pending_propagations: i32) -> bool;
    fn calc_reason(&mut self, solver: &mut T, p: Option<Lit>, extra: Option<Lit>) -> Vec<Lit>;
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
            SAT::Glucose(solver) => {
                let ret = solver.all_vars();
                unsafe { std::mem::transmute::<_, Vec<Var>>(ret) }
            }
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => {
                let ret = solver.all_vars();
                unsafe { std::mem::transmute::<_, Vec<Var>>(ret) }
            }
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => {
                let ret = solver.all_vars();
                unsafe { std::mem::transmute::<_, Vec<Var>>(ret) }
            }
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
            SAT::Glucose(solver) => solver.add_direct_encoding_extension_supports(&vars, supports),
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

    pub fn solve<'a>(&'a mut self) -> Option<SATModel<'a>> {
        match self {
            SAT::Glucose(solver) => solver.solve().map(|model| SATModel::Glucose(model)),
            #[cfg(feature = "backend-external")]
            SAT::External(solver) => solver.solve().map(|model| SATModel::External(model)),
            #[cfg(feature = "backend-cadical")]
            SAT::CaDiCaL(solver) => solver.solve().map(|model| SATModel::CaDiCaL(model)),
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

    pub(crate) unsafe fn model<'a>(&'a self) -> SATModel<'a> {
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

impl<'a> SATModel<'a> {
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
