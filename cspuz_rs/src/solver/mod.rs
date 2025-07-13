mod constraints;
mod ndarray;
pub(crate) mod traits;

use std::borrow::Borrow;

pub use cspuz_core::config::{Config, GraphDivisionMode};
use cspuz_core::csp::BoolExpr as CSPBoolExpr;
use cspuz_core::csp::BoolVar as CSPBoolVar;
use cspuz_core::csp::IntExpr as CSPIntExpr;
use cspuz_core::csp::IntVar as CSPIntVar;
use cspuz_core::csp::{Assignment, Stmt};
use cspuz_core::custom_constraints::PropagatorGenerator;
use cspuz_core::domain::Domain;
use cspuz_core::integration::IntegratedSolver;
use cspuz_core::integration::Model as IntegratedModel;
pub use cspuz_core::integration::PerfStats;
pub use cspuz_core::propagators::graph_division::GraphDivisionOptions;

use ndarray::NdArray;

use traits::{BoolArrayLike, IntArrayLike};

pub type BoolVar = NdArray<(), CSPBoolVar>;
pub type BoolExpr = NdArray<(), CSPBoolExpr>;
pub type IntVar = NdArray<(), CSPIntVar>;
pub type IntExpr = NdArray<(), CSPIntExpr>;
pub type BoolVarArray1D = NdArray<(usize,), CSPBoolVar>;
pub type BoolVarArray2D = NdArray<(usize, usize), CSPBoolVar>;
pub type BoolExprArray1D = NdArray<(usize,), CSPBoolExpr>;
pub type BoolExprArray2D = NdArray<(usize, usize), CSPBoolExpr>;
pub type IntVarArray1D = NdArray<(usize,), CSPIntVar>;
pub type IntVarArray2D = NdArray<(usize, usize), CSPIntVar>;
pub type IntExprArray1D = NdArray<(usize,), CSPIntExpr>;
pub type IntExprArray2D = NdArray<(usize, usize), CSPIntExpr>;

pub use constraints::{
    all, any, bool_constant, consecutive_prefix_true, count_true, int_constant, sum, FALSE, TRUE,
};

pub struct Solver<'a> {
    solver: IntegratedSolver<'a>,
    answer_key_bool: Vec<CSPBoolVar>,
    answer_key_int: Vec<CSPIntVar>,
}

impl<'a> Solver<'a> {
    /// Creates a new `Solver` instance.
    pub fn new() -> Solver<'a> {
        Solver {
            solver: IntegratedSolver::new(),
            answer_key_bool: vec![],
            answer_key_int: vec![],
        }
    }

    /// Creates a new `Solver` instance with a custom configuration.
    pub fn with_config(config: Config) -> Solver<'a> {
        Solver {
            solver: IntegratedSolver::with_config(config),
            answer_key_bool: vec![],
            answer_key_int: vec![],
        }
    }

    /// Creates and returns a new boolean variable.
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = solver.bool_var();
    /// ```
    pub fn bool_var(&mut self) -> BoolVar {
        NdArray::<(), _>::from_raw(self.solver.new_bool_var())
    }

    pub fn add_prenormalize_var(&mut self, var: BoolVar) {
        self.solver.add_prenormalize_var(var.data.0.clone());
    }

    /// Creates and returns a new 1D array of boolean variables of the specified length.
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = solver.bool_var_1d(10);
    /// ```
    pub fn bool_var_1d(&mut self, len: usize) -> BoolVarArray1D {
        NdArray {
            shape: (len,),
            data: (0..len).map(|_| self.solver.new_bool_var()).collect(),
        }
    }

    /// Creates and returns a new 2D array of boolean variables of the specified shape.
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = solver.bool_var_2d((5, 4));
    /// ```
    pub fn bool_var_2d(&mut self, shape: (usize, usize)) -> BoolVarArray2D {
        let (h, w) = shape;
        NdArray {
            shape,
            data: (0..(h * w)).map(|_| self.solver.new_bool_var()).collect(),
        }
    }

    /// Creates and returns a new integer variable with the domain `[low, high]` (inclusive).
    ///
    /// The returned variable can take any integer value between `low` and `high`, inclusive.
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = solver.int_var(0, 10);
    /// ```
    pub fn int_var(&mut self, low: i32, high: i32) -> IntVar {
        NdArray::<(), _>::from_raw(self.solver.new_int_var(Domain::range(low, high)))
    }

    /// Creates and returns a new integer variable with the specified domain.
    ///
    /// The returned variable can take any integer value in the specified `domain`.
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = solver.int_var_from_domain(vec![0, 1, 3, 5]);
    /// ```
    pub fn int_var_from_domain(&mut self, domain: Vec<i32>) -> IntVar {
        NdArray::<(), _>::from_raw(self.solver.new_int_var_from_list(domain))
    }

    /// Creates and returns a new 1D array of integer variables of the specified length with the domain `[low, high]` (inclusive).
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = solver.int_var_1d(10, 0, 5);
    /// ```
    pub fn int_var_1d(&mut self, len: usize, low: i32, high: i32) -> IntVarArray1D {
        NdArray {
            shape: (len,),
            data: (0..len)
                .map(|_| self.solver.new_int_var(Domain::range(low, high)))
                .collect(),
        }
    }

    /// Creates and returns a new 2D array of integer variables of the specified shape with the domain `[low, high]` (inclusive).
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = solver.int_var_2d((5, 4), 0, 5);
    /// ```
    pub fn int_var_2d(&mut self, shape: (usize, usize), low: i32, high: i32) -> IntVarArray2D {
        let (h, w) = shape;
        NdArray {
            shape,
            data: (0..(h * w))
                .map(|_| self.solver.new_int_var(Domain::range(low, high)))
                .collect(),
        }
    }

    /// Creates and returns a new 2D array of integer variables of the specified shape with the specified domain range for each element.
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = solver.int_var_2d_from_ranges((2, 3), &vec![
    ///     vec![(0, 1), (0, 2), (0, 3)],
    ///     vec![(1, 2), (0, 3), (1, 4)],
    /// ]);
    /// ```
    pub fn int_var_2d_from_ranges(
        &mut self,
        shape: (usize, usize),
        range: &[Vec<(i32, i32)>],
    ) -> IntVarArray2D {
        let (h, w) = shape;
        NdArray {
            shape,
            data: (0..(h * w))
                .map(|i| {
                    let (low, high) = range[i / w][i % w];
                    self.solver.new_int_var(Domain::range(low, high))
                })
                .collect(),
        }
    }

    /// Adds a constraint that the specified boolean expression(s) is true.
    ///
    /// You can pass multiple boolean expressions to this method, and the solver will add a constraint that all of them are true.
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = &solver.bool_var();
    /// let y = &solver.bool_var();
    /// solver.add_expr(x | y);
    /// solver.add_expr([x.imp(y), x & y]);  // multiple expressions
    ///
    /// let a = &solver.bool_var_2d((3, 4));
    /// solver.add_expr(a);  // BoolVarArray2D is also supported
    /// ```
    pub fn add_expr<T: BoolArrayLike>(&mut self, exprs: T) {
        exprs
            .to_vec()
            .into_iter()
            .for_each(|e| self.solver.add_expr(e));
    }

    /// Adds a constraint that the specified integer expressions have different values.
    ///
    /// # Examples
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let a = &solver.int_var_1d(5, 0, 3);
    /// solver.all_different(a);
    ///
    /// assert!(solver.solve().is_none());
    /// ```
    pub fn all_different<T: IntArrayLike>(&mut self, exprs: T) {
        let exprs = exprs.to_vec();
        self.solver.add_constraint(Stmt::AllDifferent(exprs));
    }

    pub fn add_active_vertices_connected<T: BoolArrayLike>(
        &mut self,
        exprs: T,
        graph: &[(usize, usize)],
    ) {
        let vertices = exprs.to_vec();
        let n_vertices = vertices.len();
        for &(u, v) in graph {
            assert!(u < n_vertices);
            assert!(v < n_vertices);
        }
        self.solver
            .add_constraint(Stmt::ActiveVerticesConnected(vertices, graph.to_owned()));
    }

    pub fn add_graph_division<T: BoolArrayLike>(
        &mut self,
        sizes: &[Option<IntExpr>],
        edges: &[(usize, usize)],
        edge_values: T,
    ) {
        self.add_graph_division_with_options(
            sizes,
            edges,
            edge_values,
            GraphDivisionOptions::default(),
        );
    }

    pub fn add_graph_division_with_options<T: BoolArrayLike>(
        &mut self,
        sizes: &[Option<IntExpr>],
        edges: &[(usize, usize)],
        edge_values: T,
        opts: GraphDivisionOptions,
    ) {
        let sizes = sizes
            .into_iter()
            .map(|x| x.clone().map(|x| x.data.0.clone()))
            .collect::<Vec<_>>();
        self.solver.add_constraint(Stmt::GraphDivision(
            sizes,
            edges.to_owned(),
            edge_values.to_vec(),
            opts,
        ));
    }

    pub fn add_custom_constraint<T: BoolArrayLike>(
        &mut self,
        constraint: Box<dyn PropagatorGenerator>,
        vars: T,
    ) {
        self.solver
            .add_constraint(Stmt::CustomConstraint(vars.to_vec(), constraint));
    }

    pub fn set_perf_stats<'b: 'a>(&mut self, perf_stats: &'b PerfStats) {
        self.solver.set_perf_stats(perf_stats);
    }

    /// Registers the specified boolean variable(s) as the answer key(s).
    ///
    /// Variables representing the "answer" of the problem instance (not proxy variables) can be
    /// registered as the answer key. Answer keys are used in `irrefutable_facts` and `answer_iter` methods.
    ///
    /// # Example
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = &solver.bool_var();
    /// solver.add_answer_key_bool(x);  // single variable
    ///
    /// let y = &solver.bool_var_1d(10);
    /// solver.add_answer_key_bool(y);  // array of variables
    ///
    /// let z = &solver.bool_var_2d((5, 4));
    /// solver.add_answer_key_bool(z);  // 2D array of variables
    ///
    /// let a = &solver.bool_var();
    /// let b = &solver.bool_var();
    /// solver.add_answer_key_bool([a, b]);  // multiple variables
    /// ```
    pub fn add_answer_key_bool<T>(&mut self, keys: T)
    where
        T: IntoIterator,
        T::Item: std::borrow::Borrow<BoolVar>,
    {
        self.answer_key_bool
            .extend(keys.into_iter().map(|x| x.borrow().data.0.clone()))
    }

    /// Registers the specified integer variable(s) as the answer key(s).
    ///
    /// Variables representing the "answer" of the problem instance (not proxy variables) can be
    /// registered as the answer key. Answer keys are used in `irrefutable_facts` and `answer_iter` methods.
    pub fn add_answer_key_int<T>(&mut self, keys: T)
    where
        T: IntoIterator,
        T::Item: std::borrow::Borrow<IntVar>,
    {
        self.answer_key_int
            .extend(keys.into_iter().map(|x| x.borrow().data.0.clone()))
    }

    pub fn encode(&mut self) -> bool {
        self.solver.encode()
    }

    /// Solves the CSP instance and returns a model (a mapping from variables to values) if it exists.
    ///
    /// If the CSP instance is unsatisfiable, this method returns `None`.
    ///
    /// You can call `solve` multiple times on the same `Solver` instance.
    /// Each call considers all constraints that have been added to the `Solver` so far and returns a model that satisfies them.
    ///
    /// # Example
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = &solver.bool_var();
    /// let y = &solver.bool_var();
    /// let z = &solver.bool_var();
    ///
    /// solver.add_expr(x | y);
    ///
    /// let model = solver.solve();
    /// assert!(model.is_some());
    ///
    /// solver.add_expr(!x | z);
    /// solver.add_expr(!y | z);
    ///
    /// let model = solver.solve();
    /// assert!(model.is_some());
    ///
    /// solver.add_expr(!z);
    ///
    /// let model = solver.solve();
    /// assert!(model.is_none());
    /// ```
    pub fn solve<'b>(&'b mut self) -> Option<Model<'b>> {
        self.solver.solve().map(|model| Model { model })
    }

    /// Returns a partial model containing each answer key variable whose value is the same across all possible models
    /// of the CSP instance. Each such variable is assigned its decided value in the returned model.
    ///
    /// If the CSP instance is unsatisfiable, this method returns `None`.
    ///
    /// This method may introduce additional constraints when computing partial models and therefore consumes the `Solver` instance.
    ///
    /// # Example
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = &solver.bool_var();
    /// let y = &solver.bool_var();
    /// let z = &solver.bool_var();
    /// solver.add_answer_key_bool([x, y, z]);
    ///
    /// solver.add_expr(x | y);
    /// solver.add_expr(x | z);
    /// solver.add_expr(!y | !z);
    ///
    /// let partial_model = solver.irrefutable_facts();
    /// assert!(partial_model.is_some());
    /// let partial_model = partial_model.unwrap();
    ///
    /// // For this instance, there are 3 possible assigmnenets:
    /// // 1. x = true, y = false, z = false
    /// // 2. x = true, y = false, z = true
    /// // 3. x = true, y = true, z = false
    /// // `x` is always true, while the value of `y` and `z` is not decided.
    /// assert_eq!(partial_model.get(x), Some(true));
    /// assert_eq!(partial_model.get(y), None);
    /// assert_eq!(partial_model.get(z), None);
    /// ```
    pub fn irrefutable_facts(self) -> Option<OwnedPartialModel> {
        self.solver
            .decide_irrefutable_facts(&self.answer_key_bool, &self.answer_key_int)
            .map(|assignment| OwnedPartialModel { assignment })
    }

    /// Returns an iterator that yields all possible assignments to the answer key variables.
    ///
    /// The order of assignments is implementation dependent and not guaranteed to be stable.
    ///
    /// This method may introduce additional constraints during search and therefore consumes the `Solver` instance.
    ///
    /// # Example
    /// ```
    /// # use cspuz_rs::solver::Solver;
    /// let mut solver = Solver::new();
    /// let x = &solver.bool_var();
    /// let y = &solver.bool_var();
    /// let z = &solver.bool_var();
    ///
    /// solver.add_answer_key_bool([x, y]);
    ///
    /// solver.add_expr(x | y);
    ///
    /// let iter = solver.answer_iter();
    /// let count = iter.count();
    ///
    /// // For this instance, there are 3 possible assignments to (x, y):
    /// // 1. x = false, y = true
    /// // 2. x = true, y = false
    /// // 3. x = true, y = true
    /// // Note that `z` is not included in the answer key, so the value of `z` is not considered.
    /// assert_eq!(count, 3);
    /// ```
    pub fn answer_iter(self) -> impl Iterator<Item = OwnedPartialModel> + 'a {
        self.solver
            .answer_iter(&self.answer_key_bool, &self.answer_key_int)
            .map(|assignment| OwnedPartialModel { assignment })
    }
}

pub struct Model<'a> {
    model: IntegratedModel<'a>,
}

impl<'a> Model<'a> {
    pub fn get<T>(&self, var: &T) -> <T as FromModel>::Output
    where
        T: FromModel,
    {
        var.from_model(self)
    }
}

pub trait FromModel {
    type Output;

    fn from_model(&self, model: &Model) -> Self::Output;
}

impl<S> FromModel for NdArray<S, CSPBoolVar>
where
    S: traits::ArrayShape<bool> + traits::ArrayShape<CSPBoolVar>,
{
    type Output = <S as traits::ArrayShape<bool>>::Output;

    fn from_model(&self, model: &Model) -> Self::Output {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|var| model.model.get_bool(var))
            .collect();
        self.shape.instantiate(data)
    }
}

impl<S> FromModel for NdArray<S, CSPIntVar>
where
    S: traits::ArrayShape<i32> + traits::ArrayShape<CSPIntVar>,
{
    type Output = <S as traits::ArrayShape<i32>>::Output;

    fn from_model(&self, model: &Model) -> Self::Output {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|var| model.model.get_int(var))
            .collect();
        self.shape.instantiate(data)
    }
}

pub struct OwnedPartialModel {
    assignment: Assignment,
}

impl OwnedPartialModel {
    pub fn get<T>(&self, var: &T) -> <T as FromOwnedPartialModel>::Output
    where
        T: FromOwnedPartialModel,
    {
        var.from_irrefutable_facts(self)
    }

    pub fn get_unwrap<T>(&self, var: &T) -> <T as FromOwnedPartialModel>::OutputUnwrap
    where
        T: FromOwnedPartialModel,
    {
        var.from_irrefutable_facts_unwrap(self)
    }
}

pub trait FromOwnedPartialModel {
    type Output;
    type OutputUnwrap;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output;
    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap;
}

impl<S> FromOwnedPartialModel for NdArray<S, CSPBoolVar>
where
    S: traits::ArrayShape<bool> + traits::ArrayShape<Option<bool>> + traits::ArrayShape<CSPBoolVar>,
{
    type Output = <S as traits::ArrayShape<Option<bool>>>::Output;
    type OutputUnwrap = <S as traits::ArrayShape<bool>>::Output;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|var| irrefutable_facts.assignment.get_bool(var))
            .collect();
        self.shape.instantiate(data)
    }

    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|var| irrefutable_facts.assignment.get_bool(var).unwrap())
            .collect();
        self.shape.instantiate(data)
    }
}

impl<S> FromOwnedPartialModel for NdArray<S, CSPIntVar>
where
    S: traits::ArrayShape<i32> + traits::ArrayShape<Option<i32>> + traits::ArrayShape<CSPIntVar>,
{
    type Output = <S as traits::ArrayShape<Option<i32>>>::Output;
    type OutputUnwrap = <S as traits::ArrayShape<i32>>::Output;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|var| irrefutable_facts.assignment.get_int(var))
            .collect();
        self.shape.instantiate(data)
    }

    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|var| irrefutable_facts.assignment.get_int(var).unwrap())
            .collect();
        self.shape.instantiate(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_operators_bool() {
        let mut solver = Solver::new();
        let b0d = &solver.bool_var();
        let b1d = &solver.bool_var_1d(7);
        let b2d = &solver.bool_var_2d((3, 5));

        let _ = b0d ^ ((b0d | b0d) & b0d);
        let _ = b1d ^ ((b0d | b1d) & b0d);
        let _ = b1d ^ ((b1d | b0d) & b0d);
        let _ = b1d | b1d;
        let _ = b2d ^ ((b0d | b2d) & b0d);
        let _ = b2d ^ ((b2d | b0d) & b0d);
        let _ = b2d | b2d;

        let _ = !b0d;
        let _ = !(b0d ^ b0d);
        let _ = !b1d;
        let _ = !(b1d ^ b1d);
        let _ = !b2d;
        let _ = !(b2d ^ b2d);
    }

    #[test]
    fn test_ite() {
        let mut solver = Solver::new();

        let b0d = &solver.bool_var();
        let b1d = &solver.bool_var_1d(7);
        let b2d = &solver.bool_var_2d((3, 5));
        let i0d = &solver.int_var(0, 2);
        let i1d = &solver.int_var_1d(7, 0, 2);
        let i2d = &solver.int_var_2d((3, 5), 0, 2);

        let _ = b0d.ite(i0d, i0d);
        let _ = b0d.ite(i0d, i1d);
        let _ = b0d.ite(i0d, i2d);
        let _ = b0d.ite(i1d, i0d);
        let _ = b0d.ite(i1d, i1d);
        let _ = b0d.ite(i2d, i0d);
        let _ = b0d.ite(i2d, i2d);
        let _ = b1d.ite(i0d, i0d);
        let _ = b1d.ite(i0d, i1d);
        let _ = b1d.ite(i1d, i1d);
        let _ = b1d.ite(i1d, i1d);
        let _ = b2d.ite(i0d, i0d);
        let _ = b2d.ite(i0d, i2d);
        let _ = b2d.ite(i2d, i2d);
        let _ = b2d.ite(i2d, i2d);
    }

    #[test]
    fn test_count_true() {
        let mut solver = Solver::new();
        let b0d = &solver.bool_var();
        let b1d = &solver.bool_var_1d(5);
        let b2d = &solver.bool_var_2d((3, 7));

        let _ = count_true(b0d);
        let _ = count_true([b0d, b0d]);
        let _ = count_true(&[b0d, b0d]);
        let _ = count_true(vec![b0d, b0d]);
        let _ = count_true(&vec![b0d, b0d]);
        let _ = count_true(b1d);
        let _ = count_true(b2d);
        let _ = b0d.count_true();
        let _ = b1d.count_true();
        let _ = b2d.count_true();
    }

    #[test]
    fn test_solver_interface() {
        let mut solver = Solver::new();
        let b0d = &solver.bool_var();
        let b1d = &solver.bool_var_1d(5);
        let b2d = &solver.bool_var_2d((3, 7));

        solver.add_expr(b0d);
        solver.add_expr([b0d, b0d]);
        solver.add_expr(&[b0d, b0d]);
        solver.add_expr(vec![b0d, b0d]);
        solver.add_expr(&vec![b0d, b0d]);
        solver.add_expr([b0d | b0d, b0d & b0d]);
        solver.add_expr(b1d);
        solver.add_expr(b1d | b1d);
        solver.add_expr(b2d);
        solver.add_expr(b2d | b2d);

        solver.add_answer_key_bool(b0d);
        solver.add_answer_key_bool([b0d]);
    }

    #[test]
    fn test_solver_iterator() {
        let mut solver = Solver::new();
        let array = &solver.bool_var_1d(5);
        solver.add_answer_key_bool(array);
        solver.add_expr(array.at(0) | array.at(1));

        let mut n_ans = 0;
        for _ in solver.answer_iter() {
            n_ans += 1;
        }
        assert_eq!(n_ans, 24);
    }
}
