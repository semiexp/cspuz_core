use crate::arithmetic::CheckedInt;
use crate::normalizer::ConvertedBoolVar;

use super::config::Config;
use super::csp::{
    Assignment, BoolExpr, BoolVar, BoolVarStatus, IntExpr, IntVar, IntVarStatus, Stmt, CSP,
};
use super::encoder::{encode, EncodeMap};
use super::norm_csp::NormCSP;
use super::normalizer::{normalize, NormalizeMap};
use super::sat::{SATModel, SAT};
use crate::domain::Domain;
use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct PerfStats {
    time_normalize: Cell<f64>,
    time_encode: Cell<f64>,
    time_sat_solver: Cell<f64>,
    decisions: Cell<u64>,
    propagations: Cell<u64>,
    conflicts: Cell<u64>,
    iterations: Cell<u64>,
}

impl PerfStats {
    pub fn new() -> PerfStats {
        PerfStats {
            time_normalize: Cell::new(0.0f64),
            time_encode: Cell::new(0.0f64),
            time_sat_solver: Cell::new(0.0f64),
            decisions: Cell::new(0u64),
            propagations: Cell::new(0u64),
            conflicts: Cell::new(0u64),
            iterations: Cell::new(0u64),
        }
    }

    pub fn time_normalize(&self) -> f64 {
        self.time_normalize.get()
    }

    pub fn time_encode(&self) -> f64 {
        self.time_encode.get()
    }

    pub fn time_sat_solver(&self) -> f64 {
        self.time_sat_solver.get()
    }

    pub fn decisions(&self) -> u64 {
        self.decisions.get()
    }

    pub fn propagations(&self) -> u64 {
        self.propagations.get()
    }

    pub fn conflicts(&self) -> u64 {
        self.conflicts.get()
    }

    pub fn iterations(&self) -> u64 {
        self.iterations.get()
    }
}

pub struct IntegratedSolver<'a> {
    csp: CSP,
    normalize_map: NormalizeMap,
    norm: NormCSP,
    encode_map: EncodeMap,
    sat: SAT,
    already_used: bool,
    config: Config,
    perf_stats: Option<&'a PerfStats>,
}

impl<'a> IntegratedSolver<'a> {
    pub fn new() -> IntegratedSolver<'a> {
        IntegratedSolver::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> IntegratedSolver<'a> {
        let mut ret = IntegratedSolver {
            csp: CSP::new(),
            normalize_map: NormalizeMap::new(),
            norm: NormCSP::new(),
            encode_map: EncodeMap::new(),
            sat: SAT::new_with_backend(config.backend),
            already_used: false,
            config,
            perf_stats: None,
        };
        ret.sat.set_rnd_init_act(ret.config.glucose_rnd_init_act);
        ret.sat
            .set_dump_analysis_info(ret.config.dump_analysis_info);
        if let Some(seed) = ret.config.glucose_random_seed {
            ret.sat.set_seed(seed);
        }
        ret
    }

    pub fn new_bool_var(&mut self) -> BoolVar {
        self.csp.new_bool_var()
    }

    pub fn new_int_var(&mut self, domain: Domain) -> IntVar {
        self.csp.new_int_var(domain)
    }

    pub fn add_prenormalize_var(&mut self, var: BoolVar) {
        self.csp.add_prenormalize_var(var);
    }

    pub fn new_int_var_from_list(&mut self, domain_list: Vec<i32>) -> IntVar {
        let domain_list = domain_list
            .into_iter()
            .map(CheckedInt::new)
            .collect::<Vec<_>>();
        self.csp.new_int_var_from_list(domain_list)
    }

    pub fn add_constraint(&mut self, stmt: Stmt) {
        self.csp.add_constraint(stmt)
    }

    pub fn add_expr(&mut self, expr: BoolExpr) {
        self.add_constraint(Stmt::Expr(expr))
    }

    pub fn encode(&mut self) -> bool {
        let is_first = !self.already_used;
        self.already_used = true;

        if self.config.use_constant_folding {
            self.csp.optimize(
                is_first && self.config.use_constant_propagation,
                self.config.verbose,
            );
        }

        if self.csp.is_inconsistent() {
            return false;
        }

        let start = std::time::Instant::now();
        normalize(
            &mut self.csp,
            &mut self.norm,
            &mut self.normalize_map,
            &self.config,
        );
        if let Some(perf_stats) = self.perf_stats {
            perf_stats
                .time_normalize
                .set(perf_stats.time_normalize() + start.elapsed().as_secs_f64());
        }

        if is_first && self.config.use_norm_domain_refinement {
            self.norm.refine_domain();
        }
        if self.norm.is_inconsistent() {
            return false;
        }

        let start = std::time::Instant::now();
        encode(
            &mut self.norm,
            &mut self.sat,
            &mut self.encode_map,
            &self.config,
        );
        if let Some(perf_stats) = self.perf_stats {
            perf_stats
                .time_encode
                .set(perf_stats.time_encode() + start.elapsed().as_secs_f64());
        }
        let solver_stats = self.sat.stats();
        if let Some(perf_stats) = self.perf_stats {
            if let Some(decisions) = solver_stats.decisions {
                perf_stats.decisions.set(decisions);
            }
            if let Some(propagations) = solver_stats.propagations {
                perf_stats.propagations.set(propagations);
            }
            if let Some(conflicts) = solver_stats.conflicts {
                perf_stats.conflicts.set(conflicts);
            }
        }
        true
    }

    pub fn solve(&mut self) -> Option<Model<'_>> {
        if !self.encode() {
            return None;
        }
        let start = std::time::Instant::now();
        let solver_result = if self.sat.solve_without_model() {
            Some(unsafe { self.sat.model() })
        } else {
            None
        };
        if let Some(perf_stats) = self.perf_stats {
            perf_stats
                .time_sat_solver
                .set(perf_stats.time_sat_solver() + start.elapsed().as_secs_f64());
        }
        let solver_stats = self.sat.stats();
        if let Some(perf_stats) = self.perf_stats {
            if let Some(decisions) = solver_stats.decisions {
                perf_stats.decisions.set(decisions);
            }
            if let Some(propagations) = solver_stats.propagations {
                perf_stats.propagations.set(propagations);
            }
            if let Some(conflicts) = solver_stats.conflicts {
                perf_stats.conflicts.set(conflicts);
            }
        }

        match solver_result {
            Some(model) => Some(Model {
                csp: &self.csp,
                normalize_map: &self.normalize_map,
                norm_csp: &self.norm,
                encode_map: &self.encode_map,
                model,
            }),
            None => None,
        }
    }

    /// Enumerate all the valid assignments of the CSP problem.
    /// Since this function may modify the problem instance, this consumes `self` to avoid further operations.
    pub fn enumerate_valid_assignments(self) -> Vec<Assignment> {
        let mut bool_vars = vec![];
        for v in self.csp.vars.bool_vars_iter() {
            bool_vars.push(v);
        }
        let mut int_vars = vec![];
        for v in self.csp.vars.int_vars_iter() {
            int_vars.push(v);
        }

        self.answer_iter(&bool_vars, &int_vars).collect()
    }

    pub fn decide_irrefutable_facts(
        mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
    ) -> Option<Assignment> {
        let mut assignment = Assignment::new();
        match self.solve() {
            Some(model) => {
                for &var in bool_vars {
                    assignment.set_bool(var, model.get_bool(var));
                }
                for &var in int_vars {
                    assignment.set_int(var, model.get_int(var));
                }
            }
            None => return None,
        }
        let mut iterations = 1;
        loop {
            let mut refutation = vec![];
            for (&v, &b) in assignment.bool_iter() {
                refutation.push(Box::new(if b { !v.expr() } else { v.expr() }));
            }
            for (&v, &i) in assignment.int_iter() {
                refutation.push(Box::new(v.expr().ne(IntExpr::Const(i))));
            }
            self.add_expr(BoolExpr::Or(refutation));

            if self.config.optimize_polarity {
                // To accelerate convergence, it is better to find assignments in which many variables
                // are assigned with values that are different from current values.
                // Here we set the polarity of the variables so that the negation of the current value is
                // preferred.
                for (&v, &b) in assignment.bool_iter() {
                    let converted = self.normalize_map.get_bool_var_raw(v);
                    if let ConvertedBoolVar::Lit(norm_lit) = converted {
                        let sat_lit = self.encode_map.get_bool_lit(norm_lit);
                        if let Some(sat_lit) = sat_lit {
                            // NOTE: `polarity` is the negation of the preferred value of the variable
                            self.sat
                                .set_polarity(sat_lit.var(), b ^ sat_lit.is_negated());
                        }
                    }
                }

                for (&v, &n) in assignment.int_iter() {
                    if let Some(v) = self.normalize_map.get_int_var(v) {
                        let eq_lit = self.encode_map.int_equal_lit(v, CheckedInt::new(n));
                        if let Some(eq_lit) = eq_lit {
                            self.sat.set_polarity(eq_lit.var(), !eq_lit.is_negated());
                        }
                    }
                }
            }

            iterations += 1;
            match self.solve() {
                Some(model) => {
                    let bool_erased = assignment
                        .bool_iter()
                        .filter_map(|(&v, &b)| {
                            if model.get_bool(v) != b {
                                Some(v)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    let int_erased = assignment
                        .int_iter()
                        .filter_map(|(&v, &i)| if model.get_int(v) != i { Some(v) } else { None })
                        .collect::<Vec<_>>();

                    bool_erased
                        .iter()
                        .for_each(|&v| assert!(assignment.remove_bool(v).is_some()));
                    int_erased
                        .iter()
                        .for_each(|&v| assert!(assignment.remove_int(v).is_some()));
                }
                None => break,
            }
        }

        if let Some(perf_stats) = self.perf_stats {
            perf_stats.iterations.set(iterations);
        }

        Some(assignment)
    }

    pub fn answer_iter(self, bool_vars: &[BoolVar], int_vars: &[IntVar]) -> AnswerIterator<'a> {
        AnswerIterator {
            solver: self,
            key_bool: bool_vars.to_vec(),
            key_int: int_vars.to_vec(),
        }
    }

    pub fn set_perf_stats<'b: 'a>(&mut self, perf_stats: &'b PerfStats) {
        self.perf_stats = Some(perf_stats);
    }

    pub fn perf_stats(&self) -> Option<PerfStats> {
        self.perf_stats.cloned()
    }
}

pub struct AnswerIterator<'a> {
    solver: IntegratedSolver<'a>,
    key_bool: Vec<BoolVar>,
    key_int: Vec<IntVar>,
}

impl Iterator for AnswerIterator<'_> {
    type Item = Assignment;

    fn next(&mut self) -> Option<Assignment> {
        let model = self.solver.solve();
        if let Some(model) = &model {
            let mut ret = Assignment::new();
            let mut refutation = vec![];
            for &var in &self.key_bool {
                let b = model.get_bool(var);
                ret.set_bool(var, b);
                refutation.push(Box::new(if b { !var.expr() } else { var.expr() }));
            }
            for &var in &self.key_int {
                let n = model.get_int(var);
                ret.set_int(var, n);
                refutation.push(Box::new(var.expr().ne(IntExpr::Const(n))));
            }
            self.solver.add_expr(BoolExpr::Or(refutation));

            Some(ret)
        } else {
            None
        }
    }
}

pub struct Model<'a> {
    csp: &'a CSP,
    normalize_map: &'a NormalizeMap,
    norm_csp: &'a NormCSP,
    encode_map: &'a EncodeMap,
    model: SATModel<'a>,
}

impl Model<'_> {
    pub fn get_bool(&self, var: BoolVar) -> bool {
        match self.normalize_map.get_bool_var_raw(var) {
            ConvertedBoolVar::Lit(norm_lit) => {
                self.encode_map
                    .get_bool_lit(norm_lit)
                    .map(|sat_lit| self.model.assignment(sat_lit.var()) ^ sat_lit.is_negated())
                    .unwrap_or(false) // unused variable optimization
            }
            ConvertedBoolVar::Removed => {
                let var_data = self.csp.get_bool_var_status(var);
                match var_data {
                    BoolVarStatus::Infeasible => panic!(),
                    BoolVarStatus::Fixed(v) => v,
                    BoolVarStatus::Unfixed => panic!(),
                }
            }
            ConvertedBoolVar::NotConverted => {
                let var_data = self.csp.get_bool_var_status(var);
                match var_data {
                    BoolVarStatus::Infeasible => panic!(),
                    BoolVarStatus::Fixed(_) => panic!(),
                    BoolVarStatus::Unfixed => false, // unused variable optimization
                }
            }
        }
    }

    pub fn get_int(&self, var: IntVar) -> i32 {
        self.get_int_checked(var).get()
    }

    fn get_int_checked(&self, var: IntVar) -> CheckedInt {
        match self.normalize_map.get_int_var(var) {
            Some(norm_var) => {
                self.encode_map
                    .get_int_value_checked(&self.model, norm_var)
                    .unwrap_or(self.norm_csp.vars.int_var(norm_var).lower_bound_checked())
                // unused variable optimization
            }
            None => {
                let var_data = self.csp.get_int_var_status(var);
                match var_data {
                    IntVarStatus::Infeasible => panic!(),
                    IntVarStatus::Fixed(v) => v,
                    IntVarStatus::Unfixed(v) => v,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
