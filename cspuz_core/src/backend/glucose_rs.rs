use glucose_rs::constraint::{Constraint, ConstraintIdx};
use glucose_rs::constraints::{
    ActiveVerticesConnected, DirectEncodingExtensionSupports, GraphDivision as RsGraphDivision,
    LinearTerm as RsLinearTerm, OptionalOrderEncoding, OrderEncodingLinear as RsOrderEncodingLinear,
};
use glucose_rs::solver::Solver as RawSolver;
use glucose_rs::types::{LBool as RawLBool, Lit as RawLit};

use crate::propagators::graph_division::{GraphDivision, GraphDivisionOptions};
use crate::propagators::order_encoding_linear;
use crate::sat::{
    CustomPropagator, GraphDivisionMode, Lit, OrderEncodingLinearMode, SolverManipulator, Var,
};

const NUM_VAR_MAX: i32 = 0x3fffffff;

fn to_raw_lit(lit: Lit) -> RawLit {
    assert!(lit.0 >= 0);
    RawLit(lit.0 as u32)
}

fn from_raw_lit(lit: RawLit) -> Lit {
    assert!(lit.0 <= i32::MAX as u32);
    Lit(lit.0 as i32)
}

#[derive(Clone, Copy)]
pub struct GlucoseSolverManipulator {
    solver: *mut RawSolver,
    constraint_idx: ConstraintIdx,
}

unsafe impl SolverManipulator for GlucoseSolverManipulator {
    unsafe fn value(&self, lit: Lit) -> Option<bool> {
        match (*self.solver).value_of(to_raw_lit(lit)) {
            RawLBool::True => Some(true),
            RawLBool::False => Some(false),
            RawLBool::Undef => None,
        }
    }

    unsafe fn add_watch(&mut self, lit: Lit) {
        (*self.solver).add_watch(to_raw_lit(lit), self.constraint_idx);
    }

    unsafe fn enqueue(&mut self, lit: Lit) -> bool {
        (*self.solver).constraint_enqueue(to_raw_lit(lit), self.constraint_idx)
    }

    unsafe fn is_current_level(&self, lit: Lit) -> bool {
        self.value(lit) == Some(true) && (*self.solver).current_level() > 0
    }
}

struct CustomConstraintAdapter {
    propagator: Box<dyn CustomPropagator<GlucoseSolverManipulator>>,
}

impl Constraint for CustomConstraintAdapter {
    fn initialize(&mut self, solver: &mut RawSolver, ci: ConstraintIdx) -> bool {
        self.propagator.initialize(&mut GlucoseSolverManipulator {
            solver: solver as *mut RawSolver,
            constraint_idx: ci,
        })
    }

    fn propagate(&mut self, solver: &mut RawSolver, p: RawLit, ci: ConstraintIdx) -> bool {
        self.propagator.propagate(
            &mut GlucoseSolverManipulator {
                solver: solver as *mut RawSolver,
                constraint_idx: ci,
            },
            from_raw_lit(p),
            solver.num_pending(ci),
        )
    }

    fn calc_reason(
        &mut self,
        solver: &mut RawSolver,
        p: Option<RawLit>,
        extra: Option<RawLit>,
        out_reason: &mut Vec<RawLit>,
    ) {
        let reason = self.propagator.calc_reason(
            &mut GlucoseSolverManipulator {
                solver: solver as *mut RawSolver,
                constraint_idx: 0,
            },
            p.map(from_raw_lit),
            extra.map(from_raw_lit),
        );
        out_reason.extend(reason.into_iter().map(to_raw_lit));
    }

    fn undo(&mut self, solver: &mut RawSolver, p: RawLit) {
        self.propagator.undo(
            &mut GlucoseSolverManipulator {
                solver: solver as *mut RawSolver,
                constraint_idx: 0,
            },
            from_raw_lit(p),
        );
    }
}

pub struct Solver {
    solver: RawSolver,
    num_var: i32,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            solver: RawSolver::new(),
            num_var: 0,
        }
    }

    pub fn new_var(&mut self) -> Var {
        let var_id = self.solver.new_var();
        assert!(var_id <= NUM_VAR_MAX as u32);
        self.num_var += 1;
        Var(var_id as i32)
    }

    #[cfg(feature = "sat-analyzer")]
    pub fn new_named_var(&mut self, _name: &str) -> Var {
        self.new_var()
    }

    pub fn num_var(&self) -> i32 {
        self.num_var
    }

    pub fn all_vars(&self) -> Vec<Var> {
        (0..self.num_var()).map(Var).collect()
    }

    pub fn set_polarity(&mut self, _var: Var, _polarity: bool) {}

    pub fn add_clause(&mut self, clause: &[Lit]) -> bool {
        let clause = clause.iter().copied().map(to_raw_lit).collect::<Vec<_>>();
        self.solver.add_clause(&clause)
    }

    pub fn add_order_encoding_linear(
        &mut self,
        lits: &[Vec<Lit>],
        domain: &[Vec<i32>],
        coefs: &[i32],
        constant: i32,
        mode: OrderEncodingLinearMode,
    ) -> bool {
        assert_eq!(lits.len(), domain.len());
        assert_eq!(lits.len(), coefs.len());
        match mode {
            OrderEncodingLinearMode::Cpp => {
                let mut terms = vec![];
                for i in 0..lits.len() {
                    terms.push(RsLinearTerm {
                        lits: lits[i].iter().copied().map(to_raw_lit).collect(),
                        domain: domain[i].clone(),
                        coef: coefs[i],
                    });
                }
                self.solver
                    .add_constraint(Box::new(RsOrderEncodingLinear::new(terms, constant)))
            }
            OrderEncodingLinearMode::Rust | OrderEncodingLinearMode::RustOptimized => {
                let mut terms = vec![];
                for i in 0..lits.len() {
                    terms.push(order_encoding_linear::LinearTerm::new(
                        lits[i].clone(),
                        domain[i].clone(),
                        coefs[i],
                    ));
                }
                let optimized = mode == OrderEncodingLinearMode::RustOptimized;
                self.add_custom_constraint(Box::new(
                    order_encoding_linear::OrderEncodingLinear::new(terms, constant, optimized),
                ))
            }
        }
    }

    pub fn add_active_vertices_connected(&mut self, lits: &[Lit], edges: &[(usize, usize)]) -> bool {
        let lits = lits.iter().copied().map(to_raw_lit).collect::<Vec<_>>();
        self.solver
            .add_constraint(Box::new(ActiveVerticesConnected::new(lits, edges)))
    }

    pub fn add_direct_encoding_extension_supports(
        &mut self,
        vars: &[Vec<Lit>],
        supports: &[Vec<Option<usize>>],
    ) -> bool {
        let vars = vars
            .iter()
            .map(|row| row.iter().copied().map(to_raw_lit).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let supports = supports
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| v.map(|x| x as i32).unwrap_or(-1))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        self.solver
            .add_constraint(Box::new(DirectEncodingExtensionSupports::new(vars, supports)))
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
        if mode == GraphDivisionMode::Rust {
            let vertex_weights = vec![1; domains.len()];
            let constr = GraphDivision::new(domains, dom_lits, &vertex_weights, edges, edge_lits, opts);
            return self.add_custom_constraint(Box::new(constr));
        }

        assert!(!opts.require_extra_constraints());
        let vertices = domains
            .iter()
            .zip(dom_lits)
            .map(|(domain, lits)| {
                if domain.is_empty() {
                    OptionalOrderEncoding {
                        lits: vec![],
                        values: vec![],
                    }
                } else {
                    OptionalOrderEncoding {
                        lits: lits.iter().copied().map(to_raw_lit).collect::<Vec<_>>(),
                        values: domain.clone(),
                    }
                }
            })
            .collect::<Vec<_>>();
        let edge_lits = edge_lits.iter().copied().map(to_raw_lit).collect::<Vec<_>>();
        self.solver
            .add_constraint(Box::new(RsGraphDivision::new(vertices, edges, edge_lits)))
    }

    pub fn add_custom_constraint(
        &mut self,
        constraint: Box<dyn CustomPropagator<GlucoseSolverManipulator>>,
    ) -> bool {
        self.solver.add_constraint(Box::new(CustomConstraintAdapter { propagator: constraint }))
    }

    pub fn set_seed(&mut self, _seed: f64) {}

    pub fn set_rnd_init_act(&mut self, _rnd_init_act: bool) {}

    pub fn set_dump_analysis_info(&mut self, _dump_analysis_info: bool) {}

    pub fn solve(&mut self) -> Option<Model<'_>> {
        if self.solve_without_model() {
            Some(unsafe { self.model() })
        } else {
            None
        }
    }

    pub fn solve_without_model(&mut self) -> bool {
        matches!(self.solver.solve(), RawLBool::True)
    }

    pub(crate) unsafe fn model(&self) -> Model<'_> {
        Model { solver: self }
    }

    pub fn stats_decisions(&self) -> u64 {
        0
    }

    pub fn stats_propagations(&self) -> u64 {
        0
    }

    pub fn stats_conflicts(&self) -> u64 {
        0
    }
}

#[derive(Clone, Copy)]
pub struct Model<'a> {
    solver: &'a Solver,
}

impl Model<'_> {
    pub fn assignment(&self, var: Var) -> bool {
        assert!(0 <= var.0 && var.0 < self.solver.num_var());
        match self.solver.solver.model[var.0 as usize] {
            RawLBool::True => true,
            RawLBool::False => false,
            RawLBool::Undef => panic!("unexpected undefined model value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() {
        let mut solver = Solver::new();
        let x = solver.new_var();
        let y = solver.new_var();

        assert!(solver.add_clause(&[Lit::new(x, false), Lit::new(y, false)]));
        assert!(solver.add_clause(&[Lit::new(x, false), Lit::new(y, true)]));
        assert!(solver.add_clause(&[Lit::new(x, true), Lit::new(y, false)]));
        assert!(solver.add_clause(&[Lit::new(x, true), Lit::new(y, true)]));
        assert!(solver.solve().is_none());
    }
}
