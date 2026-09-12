use std::ffi::{c_void, CString};
use std::ops::Drop;
use std::os::raw::c_char;

use crate::propagators::graph_division::{GraphDivision, GraphDivisionOptions};
use crate::propagators::order_encoding_linear;
use crate::sat::{
    CustomPropagator, GraphDivisionMode, Lit, OrderEncodingLinearMode, SolverManipulator, Var,
};

#[repr(C)]
struct Opaque {
    _private: [u8; 0],
}

extern "C-unwind" {
    fn Glucose_CreateSolver() -> *mut Opaque;
    fn Glucose_DestroySolver(solver: *mut Opaque);
    fn Glucose_NewVar(solver: *mut Opaque) -> i32;
    fn Glucose_NewNamedVar(solver: *mut Opaque, name: *const c_char) -> i32;
    fn Glucose_AddClause(solver: *mut Opaque, lits: *const Lit, n_lits: i32) -> i32;
    fn Glucose_Solve(solver: *mut Opaque) -> i32;
    fn Glucose_NumVar(solver: *mut Opaque) -> i32;
    fn Glucose_GetModelValueVar(solver: *mut Opaque, var: i32) -> i32;
    fn Glucose_SetPolarity(solver: *mut Opaque, var: i32, polarity: i32);
    fn Glucose_AddOrderEncodingLinear(
        solver: *mut Opaque,
        n_terms: i32,
        domain_size: *const i32,
        lits: *const Lit,
        domain: *const i32,
        coefs: *const i32,
        constant: i32,
    ) -> i32;
    fn Glucose_AddActiveVerticesConnected(
        solver: *mut Opaque,
        n_vertices: i32,
        lits: *const Lit,
        n_edges: i32,
        edges: *const i32,
    ) -> i32;
    fn Glucose_AddDirectEncodingExtensionSupports(
        solver: *mut Opaque,
        n_vars: i32,
        domain_size: *const i32,
        lits: *const Lit,
        n_supports: i32,
        supports: *const i32,
    ) -> i32;
    fn Glucose_AddGraphDivision(
        solver: *mut Opaque,
        n_vertices: i32,
        dom_sizes: *const i32,
        domains: *const i32,
        dom_lits: *const Lit,
        n_edges: i32,
        edges: *const i32,
        edge_lits: *const Lit,
    ) -> i32;
    fn Glucose_SolverStats_decisions(solver: *mut Opaque) -> u64;
    fn Glucose_SolverStats_propagations(solver: *mut Opaque) -> u64;
    fn Glucose_SolverStats_conflicts(solver: *mut Opaque) -> u64;
    fn Glucose_Set_random_seed(solver: *mut Opaque, random_seed: f64);
    fn Glucose_Set_rnd_init_act(solver: *mut Opaque, rnd_init_act: i32);
    fn Glucose_Set_dump_analysis_info(solver: *mut Opaque, value: i32);
}

pub struct Solver {
    ptr: *mut Opaque,

    // We have to pass pointers to `dyn CustomPropagator` to the C++ code.
    // Since Box<dyn ...> uses a fat pointer and its representation is not specified,
    // we deliberately use `Box<Box<dyn ...>>` to make it a thin pointer.
    // https://doc.rust-lang.org/reference/type-layout.html#r-layout.pointer.unsized
    #[allow(clippy::vec_box)]
    custom_constraints: Vec<Box<Box<dyn CustomPropagator<GlucoseSolverManipulator>>>>,
}

const NUM_VAR_MAX: i32 = 0x3fffffff;

// The native bridge uses signed 32-bit offsets into flattened arrays.
fn checked_total_len(lengths: impl IntoIterator<Item = usize>) -> usize {
    lengths.into_iter().fold(0usize, |total, len| {
        let total = total.checked_add(len).expect("array length overflow");
        assert!(
            total <= i32::MAX as usize,
            "array exceeds native index range"
        );
        total
    })
}

impl Solver {
    pub fn new() -> Solver {
        let ptr = unsafe { Glucose_CreateSolver() };
        assert!(!ptr.is_null());
        Solver {
            ptr,
            custom_constraints: vec![],
        }
    }

    pub fn new_var(&mut self) -> Var {
        assert!(self.num_var() <= NUM_VAR_MAX);
        let var_id = unsafe { Glucose_NewVar(self.ptr) };
        assert!((0..=NUM_VAR_MAX).contains(&var_id));
        Var(var_id)
    }

    pub fn new_named_var(&mut self, name: &str) -> Var {
        assert!(self.num_var() <= NUM_VAR_MAX);
        let c_string = CString::new(name).unwrap();
        let var_id = unsafe { Glucose_NewNamedVar(self.ptr, c_string.as_ptr()) };
        assert!((0..=NUM_VAR_MAX).contains(&var_id));
        Var(var_id)
    }

    pub fn num_var(&self) -> i32 {
        unsafe { Glucose_NumVar(self.ptr) }
    }

    pub fn all_vars(&self) -> Vec<Var> {
        (0..self.num_var()).map(Var).collect()
    }

    fn validate_lits<'a>(&self, lits: impl IntoIterator<Item = &'a Lit>) {
        let num_var = self.num_var();
        for &lit in lits {
            assert!(lit.0 >= 0 && lit.var().0 < num_var, "invalid SAT literal");
        }
    }

    pub fn set_polarity(&mut self, var: Var, polarity: bool) {
        assert!(0 <= var.0 && var.0 < self.num_var());
        unsafe { Glucose_SetPolarity(self.ptr, var.0, if polarity { 1 } else { 0 }) }
    }

    pub fn add_clause(&mut self, clause: &[Lit]) -> bool {
        assert!(clause.len() <= i32::MAX as usize);
        self.validate_lits(clause);
        let res = unsafe { Glucose_AddClause(self.ptr, clause.as_ptr(), clause.len() as i32) };
        res != 0
    }

    pub fn add_order_encoding_linear(
        &mut self,
        lits: &[Vec<Lit>],
        domain: &[Vec<i32>],
        coefs: &[i32],
        constant: i32,
        mode: OrderEncodingLinearMode,
    ) -> bool {
        assert!(lits.len() <= i32::MAX as usize);
        assert_eq!(lits.len(), domain.len());
        assert_eq!(lits.len(), coefs.len());

        for i in 0..lits.len() {
            assert!(domain[i].len() <= i32::MAX as usize);
            assert_eq!(lits[i].len() + 1, domain[i].len());
        }

        checked_total_len(domain.iter().map(Vec::len));
        self.validate_lits(lits.iter().flatten());
        for values in domain {
            assert!(
                values.windows(2).all(|w| w[0] < w[1]),
                "domain must be strictly increasing"
            );
        }

        match mode {
            OrderEncodingLinearMode::Cpp => {
                assert!(coefs.iter().all(|&coef| coef != 0), "zero coefficient");
                let n_terms = lits.len() as i32;
                let domain_size = domain.iter().map(|x| x.len() as i32).collect::<Vec<_>>();
                let lits_flat = lits.iter().flatten().copied().collect::<Vec<_>>();
                let domain_flat = domain.iter().flatten().copied().collect::<Vec<_>>();
                let res = unsafe {
                    Glucose_AddOrderEncodingLinear(
                        self.ptr,
                        n_terms,
                        domain_size.as_ptr(),
                        lits_flat.as_ptr(),
                        domain_flat.as_ptr(),
                        coefs.as_ptr(),
                        constant,
                    )
                };
                res != 0
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

    pub fn add_active_vertices_connected(
        &mut self,
        lits: &[Lit],
        edges: &[(usize, usize)],
    ) -> bool {
        assert!(lits.len() <= i32::MAX as usize);
        assert!(edges.len() <= (i32::MAX as usize) / 2);
        self.validate_lits(lits);

        let mut edges_flat = vec![];
        for &(u, v) in edges {
            assert!(u < lits.len());
            assert!(v < lits.len());
            edges_flat.push(u as i32);
            edges_flat.push(v as i32);
        }

        let res = unsafe {
            Glucose_AddActiveVerticesConnected(
                self.ptr,
                lits.len() as i32,
                lits.as_ptr(),
                edges.len() as i32,
                edges_flat.as_ptr(),
            )
        };
        res != 0
    }

    pub fn add_direct_encoding_extension_supports(
        &mut self,
        vars: &[Vec<Lit>],
        supports: &[Vec<Option<usize>>],
    ) -> bool {
        assert!(vars.len() <= i32::MAX as usize);
        assert!(supports.len() <= i32::MAX as usize);
        checked_total_len(vars.iter().map(Vec::len));
        assert!(vars
            .len()
            .checked_mul(supports.len())
            .is_some_and(|n| n <= i32::MAX as usize));
        self.validate_lits(vars.iter().flatten());

        let domain_size = vars.iter().map(|v| v.len() as i32).collect::<Vec<_>>();
        let vars_flat = vars.iter().flatten().copied().collect::<Vec<_>>();
        let mut supports_flat = vec![];
        for support in supports {
            assert_eq!(vars.len(), support.len());
            for i in 0..vars.len() {
                if let Some(n) = support[i] {
                    assert!(n < vars[i].len());
                    supports_flat.push(n as i32);
                } else {
                    supports_flat.push(-1);
                }
            }
        }

        let res = unsafe {
            Glucose_AddDirectEncodingExtensionSupports(
                self.ptr,
                vars.len() as i32,
                domain_size.as_ptr(),
                vars_flat.as_ptr(),
                supports.len() as i32,
                supports_flat.as_ptr(),
            )
        };
        res != 0
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
        assert!(domains.len() <= i32::MAX as usize);
        assert!(edges.len() <= (i32::MAX as usize) / 2);
        assert_eq!(domains.len(), dom_lits.len());
        assert_eq!(edges.len(), edge_lits.len());
        checked_total_len(domains.iter().map(Vec::len));
        for (domain, lits) in domains.iter().zip(dom_lits) {
            assert_eq!(lits.len(), domain.len().saturating_sub(1));
            assert!(
                domain.windows(2).all(|w| w[0] < w[1]),
                "domain must be strictly increasing"
            );
        }
        self.validate_lits(dom_lits.iter().flatten());
        self.validate_lits(edge_lits);
        for &(u, v) in edges {
            assert!(
                u < domains.len() && v < domains.len(),
                "invalid graph endpoint"
            );
            assert_ne!(u, v, "graph division does not support self-loops");
        }

        if mode == GraphDivisionMode::Rust {
            let vertex_weights = vec![1; domains.len()];
            let constr =
                GraphDivision::new(domains, dom_lits, &vertex_weights, edges, edge_lits, opts);
            return self.add_custom_constraint(Box::new(constr));
        }

        assert!(!opts.require_extra_constraints());

        let mut dom_sizes = vec![];
        let mut dom_lits_flat = vec![];
        let mut domains_flat = vec![];

        for i in 0..domains.len() {
            dom_sizes.push(domains[i].len() as i32);
            if !domains[i].is_empty() {
                assert_eq!(domains[i].len(), dom_lits[i].len() + 1);
                for &v in &domains[i] {
                    domains_flat.push(v);
                }
                for &l in &dom_lits[i] {
                    dom_lits_flat.push(l);
                }
            }
        }

        let mut edges_flat = vec![];
        for &(u, v) in edges {
            edges_flat.push(u as i32);
            edges_flat.push(v as i32);
        }

        let res = unsafe {
            Glucose_AddGraphDivision(
                self.ptr,
                domains.len() as i32,
                dom_sizes.as_ptr(),
                domains_flat.as_ptr(),
                dom_lits_flat.as_ptr(),
                edges.len() as i32,
                edges_flat.as_ptr(),
                edge_lits.as_ptr(),
            )
        };
        res != 0
    }

    pub fn add_custom_constraint(
        &mut self,
        constraint: Box<dyn CustomPropagator<GlucoseSolverManipulator>>,
    ) -> bool {
        self.custom_constraints.push(Box::new(constraint));
        #[allow(clippy::borrowed_box)]
        let c: &Box<dyn CustomPropagator<GlucoseSolverManipulator>> =
            &self.custom_constraints[self.custom_constraints.len() - 1];
        let c = unsafe { std::mem::transmute::<_, *mut c_void>(c) };
        let res = unsafe { Glucose_AddRustExtraConstraint(self.ptr, c) };
        res != 0
    }

    pub fn set_seed(&mut self, seed: f64) {
        unsafe {
            Glucose_Set_random_seed(self.ptr, seed);
        }
    }

    pub fn set_rnd_init_act(&mut self, rnd_init_act: bool) {
        unsafe {
            Glucose_Set_rnd_init_act(self.ptr, if rnd_init_act { 1 } else { 0 });
        }
    }

    pub fn set_dump_analysis_info(&mut self, dump_analysis_info: bool) {
        unsafe { Glucose_Set_dump_analysis_info(self.ptr, if dump_analysis_info { 1 } else { 0 }) }
    }

    pub fn solve(&mut self) -> Option<Model<'_>> {
        if self.solve_without_model() {
            Some(unsafe { self.model() })
        } else {
            None
        }
    }

    pub fn solve_without_model(&mut self) -> bool {
        let res = unsafe { Glucose_Solve(self.ptr) };
        res != 0
    }

    pub(crate) unsafe fn model(&self) -> Model<'_> {
        Model { solver: self }
    }

    pub fn stats_decisions(&self) -> u64 {
        unsafe { Glucose_SolverStats_decisions(self.ptr) }
    }

    pub fn stats_propagations(&self) -> u64 {
        unsafe { Glucose_SolverStats_propagations(self.ptr) }
    }

    pub fn stats_conflicts(&self) -> u64 {
        unsafe { Glucose_SolverStats_conflicts(self.ptr) }
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe {
            Glucose_DestroySolver(self.ptr);
        }
    }
}

const LIT_UNDEF: Lit = Lit(-2);

#[derive(Clone, Copy)]
pub struct Model<'a> {
    solver: &'a Solver,
}

impl Model<'_> {
    pub fn assignment(&self, var: Var) -> bool {
        assert!(0 <= var.0 && var.0 < self.solver.num_var());
        unsafe { Glucose_GetModelValueVar(self.solver.ptr, var.0) != 0 }
    }
}

// Interface for implementing custom constraints in Rust

extern "C-unwind" {
    fn Glucose_AddRustExtraConstraint(solver: *mut Opaque, trait_object: *mut c_void) -> i32;
    fn Glucose_CustomPropagatorCopyReason(reason_vec: *mut c_void, n_lits: i32, lits: *const Lit);
    fn Glucose_SolverValue(solver: *mut Opaque, lit: Lit) -> i32;
    fn Glucose_SolverAddWatch(solver: *mut Opaque, lit: Lit, wrapper_object: *mut c_void);
    fn Glucose_SolverEnqueue(solver: *mut Opaque, lit: Lit, wrapper_object: *mut c_void) -> i32;

    fn Glucose_IsCurrentLevel(solver: *mut Opaque, lit: Lit) -> i32;
}

#[derive(Clone, Copy)]
pub struct GlucoseSolverManipulator {
    ptr: *mut Opaque,
    wrapper_object: Option<*mut c_void>,
}

unsafe impl SolverManipulator for GlucoseSolverManipulator {
    unsafe fn value(&self, lit: Lit) -> Option<bool> {
        let v = Glucose_SolverValue(self.ptr, lit);
        match v {
            0 => Some(true),
            1 => Some(false),
            2 | 3 => None,
            _ => unreachable!(),
        }
    }

    unsafe fn add_watch(&mut self, lit: Lit) {
        assert!(self.wrapper_object.is_some());
        Glucose_SolverAddWatch(self.ptr, lit, self.wrapper_object.unwrap());
    }

    unsafe fn enqueue(&mut self, lit: Lit) -> bool {
        assert!(self.wrapper_object.is_some());
        Glucose_SolverEnqueue(self.ptr, lit, self.wrapper_object.unwrap()) != 0
    }

    unsafe fn is_current_level(&self, lit: Lit) -> bool {
        Glucose_IsCurrentLevel(self.ptr, lit) != 0
    }
}

#[no_mangle]
extern "C-unwind" fn Glucose_CallCustomPropagatorInitialize(
    solver: *mut Opaque,
    wrapper_object: *mut c_void,
    trait_object: *mut c_void,
) -> i32 {
    let trait_object =
        unsafe { &mut *trait_object.cast::<Box<dyn CustomPropagator<GlucoseSolverManipulator>>>() };
    let res = trait_object.initialize(&mut GlucoseSolverManipulator {
        ptr: solver,
        wrapper_object: Some(wrapper_object),
    });
    if res {
        1
    } else {
        0
    }
}

#[no_mangle]
extern "C-unwind" fn Glucose_CallCustomPropagatorPropagate(
    solver: *mut Opaque,
    wrapper_object: *mut c_void,
    trait_object: *mut c_void,
    p: Lit,
    num_pending_propagations: i32,
) -> i32 {
    let trait_object =
        unsafe { &mut *trait_object.cast::<Box<dyn CustomPropagator<GlucoseSolverManipulator>>>() };
    let res = trait_object.propagate(
        &mut GlucoseSolverManipulator {
            ptr: solver,
            wrapper_object: Some(wrapper_object),
        },
        p,
        num_pending_propagations,
    );
    if res {
        1
    } else {
        0
    }
}

#[no_mangle]
extern "C-unwind" fn Glucose_CallCustomPropagatorCalcReason(
    solver: *mut Opaque,
    trait_object: *mut c_void,
    p: Lit,
    extra: Lit,
    out_reason: *mut c_void,
) {
    let trait_object =
        unsafe { &mut *trait_object.cast::<Box<dyn CustomPropagator<GlucoseSolverManipulator>>>() };
    let res = trait_object.calc_reason(
        &mut GlucoseSolverManipulator {
            ptr: solver,
            wrapper_object: None,
        },
        if p != LIT_UNDEF { Some(p) } else { None },
        if extra != LIT_UNDEF {
            Some(extra)
        } else {
            None
        },
    );
    assert!(res.len() <= i32::MAX as usize);
    let num_var = unsafe { Glucose_NumVar(solver) };
    let mut has_current_level = false;
    for &lit in &res {
        assert!(
            lit.0 >= 0 && lit.var().0 < num_var,
            "invalid reason literal"
        );
        assert_eq!(unsafe { Glucose_SolverValue(solver, lit) }, 0);
        if unsafe { Glucose_IsCurrentLevel(solver, lit) } != 0 {
            has_current_level = true;
        }
    }
    assert!(res.is_empty() || has_current_level);

    unsafe {
        Glucose_CustomPropagatorCopyReason(out_reason, res.len() as i32, res.as_ptr());
    }
}

#[no_mangle]
extern "C-unwind" fn Glucose_CallCustomPropagatorUndo(
    solver: *mut Opaque,
    trait_object: *mut c_void,
    p: Lit,
) {
    let trait_object =
        unsafe { &mut *trait_object.cast::<Box<dyn CustomPropagator<GlucoseSolverManipulator>>>() };
    trait_object.undo(
        &mut GlucoseSolverManipulator {
            ptr: solver,
            wrapper_object: None,
        },
        p,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_invalid_literals() {
        for lit in [Lit(-2), Lit(-1), Lit(2), Lit(i32::MAX)] {
            for operation in 0..6 {
                let mut solver = Solver::new();
                solver.new_var();
                let result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match operation {
                        0 => solver.add_clause(&[lit]),
                        1 => solver.add_active_vertices_connected(&[lit], &[]),
                        2 => solver.add_order_encoding_linear(
                            &[vec![lit]],
                            &[vec![0, 1]],
                            &[1],
                            0,
                            OrderEncodingLinearMode::Cpp,
                        ),
                        3 => solver
                            .add_direct_encoding_extension_supports(&[vec![lit]], &[vec![Some(0)]]),
                        4 => solver.add_graph_division(
                            &[vec![1, 2]],
                            &[vec![lit]],
                            &[],
                            &[],
                            GraphDivisionMode::Cpp,
                            &GraphDivisionOptions::default(),
                        ),
                        _ => solver.add_graph_division(
                            &[vec![], vec![]],
                            &[vec![], vec![]],
                            &[(0, 1)],
                            &[lit],
                            GraphDivisionMode::Cpp,
                            &GraphDivisionOptions::default(),
                        ),
                    }));
                assert!(result.is_err(), "operation {operation} accepted {lit:?}");
                assert!(solver.solve().is_some());
            }
        }
    }

    #[test]
    fn test_reject_invalid_graph_structure() {
        for mode in [GraphDivisionMode::Cpp, GraphDivisionMode::Rust] {
            for edge in [(0, 1), (0, usize::MAX), (0, 0)] {
                let mut solver = Solver::new();
                let lit = solver.new_var().as_lit(false);
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    solver.add_graph_division(
                        &[vec![]],
                        &[vec![]],
                        &[edge],
                        &[lit],
                        mode,
                        &GraphDivisionOptions::default(),
                    );
                }));
                assert!(result.is_err());
            }
        }
    }

    #[test]
    #[should_panic(expected = "array exceeds native index range")]
    fn test_reject_flattened_length_overflow() {
        checked_total_len([i32::MAX as usize, 1]);
    }

    #[test]
    #[should_panic(expected = "zero coefficient")]
    fn test_reject_zero_coefficient() {
        Solver::new().add_order_encoding_linear(
            &[vec![]],
            &[vec![1]],
            &[0],
            0,
            OrderEncodingLinearMode::Cpp,
        );
    }

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

    #[test]
    fn test_solver2() {
        let mut solver = Solver::new();
        let x = solver.new_var();
        let y = solver.new_var();

        assert!(solver.add_clause(&[Lit::new(x, false), Lit::new(y, false)]));
        assert!(solver.add_clause(&[Lit::new(x, false), Lit::new(y, true)]));
        assert!(solver.add_clause(&[Lit::new(x, true), Lit::new(y, true)]));
        {
            match solver.solve() {
                Some(model) => {
                    assert!(model.assignment(x));
                    assert!(!model.assignment(y));
                }
                None => panic!(),
            }
        }
    }

    struct Xor {
        vars: Vec<Var>,
        values: Vec<Option<bool>>,
        parity: bool,
        n_undecided: usize,
    }

    impl Xor {
        fn new(vars: Vec<Var>, parity: bool) -> Xor {
            // This implementation is just for testing.
            // For practical use, we need to add deduplication of literals.
            let size = vars.len();
            Xor {
                vars,
                values: vec![None; size],
                parity,
                n_undecided: size,
            }
        }

        fn var_index(&self, var: Var) -> Option<usize> {
            for i in 0..self.vars.len() {
                if self.vars[i] == var {
                    return Some(i);
                }
            }
            None
        }
    }

    unsafe impl<T: SolverManipulator> CustomPropagator<T> for Xor {
        fn initialize(&mut self, solver: &mut T) -> bool {
            for &var in &self.vars {
                unsafe {
                    solver.add_watch(var.as_lit(false));
                    solver.add_watch(var.as_lit(true));
                }
            }

            for var in self.vars.clone() {
                // TODO: this `clone` is just for silencing the borrow checker
                if let Some(v) = unsafe { solver.value(var.as_lit(false)) } {
                    let lit_sign = var.as_lit(!v);
                    if !self.propagate(solver, lit_sign, 0) {
                        return false;
                    }
                }
            }
            true
        }

        fn propagate(&mut self, solver: &mut T, p: Lit, _num_pending_propagations: i32) -> bool {
            let s = !p.is_negated();
            let v = p.var();

            let idx = self.var_index(v).unwrap();
            assert!(self.values[idx].is_none());
            self.values[idx] = Some(s);
            self.parity ^= s;
            assert_ne!(self.n_undecided, 0);
            self.n_undecided -= 1;

            if self.n_undecided == 0 {
                if self.parity {
                    return false;
                }
            } else if self.n_undecided == 1 {
                for i in 0..self.vars.len() {
                    if self.values[i].is_none() {
                        if !unsafe { solver.enqueue(self.vars[i].as_lit(!self.parity)) } {
                            return false;
                        }
                    }
                }
            }

            true
        }

        fn calc_reason(&mut self, _: &mut T, p: Option<Lit>, extra: Option<Lit>) -> Vec<Lit> {
            let mut ret = vec![];
            for i in 0..self.vars.len() {
                if let Some(v) = self.values[i] {
                    ret.push(self.vars[i].as_lit(!v));
                } else if p.is_none() && extra.is_some() {
                    ret.push(self.vars[i].as_lit(self.parity));
                }
            }
            assert_eq!(if p.is_some() { 1 } else { 0 } + ret.len(), self.vars.len());
            ret
        }

        fn undo(&mut self, _: &mut T, p: Lit) {
            let v = p.var();

            let idx = self.var_index(v).unwrap();
            assert!(self.values[idx] == Some(!p.is_negated()));
            self.parity ^= self.values[idx].unwrap();
            self.values[idx] = None;
            self.n_undecided += 1;
        }
    }

    #[test]
    fn test_custom_constraint_xor1() {
        let mut solver = Solver::new();
        let x = solver.new_var();
        let y = solver.new_var();

        assert!(solver.add_clause(&[Lit::new(x, false), Lit::new(y, true)]));
        assert!(solver.add_custom_constraint(Box::new(Xor::new(vec![x, y], true))));
        let model = solver.solve().unwrap();
        assert!(model.assignment(x));
        assert!(!model.assignment(y));
    }

    #[test]
    fn test_custom_constraint_xor2() {
        let mut solver = Solver::new();
        let mut vars = vec![];
        for _ in 0..10 {
            vars.push(solver.new_var());
        }

        assert!(solver.add_custom_constraint(Box::new(Xor::new(
            vec![vars[0], vars[1], vars[2], vars[3], vars[4]],
            false
        ))));
        assert!(solver.add_custom_constraint(Box::new(Xor::new(
            vec![vars[3], vars[4], vars[5], vars[6], vars[7]],
            true
        ))));
        assert!(solver.add_custom_constraint(Box::new(Xor::new(
            vec![vars[7], vars[8], vars[9], vars[0], vars[1]],
            true
        ))));

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

        assert_eq!(n_assignments, 128);
    }

    #[test]
    fn test_custom_constraint_xor3() {
        let mut solver = Solver::new();
        let mut vars = vec![];
        for _ in 0..10 {
            vars.push(solver.new_var());
        }

        assert!(solver.add_custom_constraint(Box::new(Xor::new(
            vec![vars[0], vars[1], vars[2], vars[3], vars[4]],
            false
        ))));
        assert!(solver.add_custom_constraint(Box::new(Xor::new(
            vec![vars[3], vars[4], vars[5], vars[6], vars[7]],
            true
        ))));
        assert!(solver.add_custom_constraint(Box::new(Xor::new(
            vec![vars[7], vars[8], vars[9], vars[0], vars[1]],
            true
        ))));
        assert!(solver.add_custom_constraint(Box::new(Xor::new(
            vec![vars[2], vars[5], vars[6], vars[8], vars[9]],
            true
        ))));
        assert!(solver.solve().is_none());
    }
}
