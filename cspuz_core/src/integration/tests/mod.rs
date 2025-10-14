use crate::arithmetic::CmpOp;
use crate::csp::*;
use crate::domain::Domain;
use crate::integration::*;
use crate::propagators::graph_division::GraphDivisionOptions;

struct IntegrationTester<'a> {
    original_constr: Vec<Stmt>,
    solver: IntegratedSolver<'a>,
}

impl<'a> IntegrationTester<'a> {
    fn new() -> IntegrationTester<'a> {
        IntegrationTester {
            original_constr: vec![],
            solver: IntegratedSolver::new(),
        }
    }

    fn with_config(config: Config) -> IntegrationTester<'a> {
        IntegrationTester {
            original_constr: vec![],
            solver: IntegratedSolver::with_config(config),
        }
    }

    fn new_bool_var(&mut self) -> BoolVar {
        self.solver.new_bool_var()
    }

    fn new_int_var(&mut self, domain: Domain) -> IntVar {
        self.solver.new_int_var(domain.clone())
    }

    fn new_int_var_from_list(&mut self, domain_list: Vec<i32>) -> IntVar {
        self.solver.new_int_var_from_list(domain_list)
    }

    fn add_expr(&mut self, expr: BoolExpr) {
        self.add_constraint(Stmt::Expr(expr));
    }

    fn add_constraint(&mut self, stmt: Stmt) {
        let cloned = crate::csp::test_utils::clone_stmt(&stmt);
        self.original_constr.push(cloned);
        self.solver.add_constraint(stmt);
    }

    fn check(self) -> bool {
        self.check_internal(false)
    }

    fn check_internal(self, no_panic: bool) -> bool {
        let mut expected = crate::csp::test_utils::csp_all_assignments(&self.solver.csp);
        expected.sort();

        let mut actual = self.solver.enumerate_valid_assignments();
        actual.sort();

        if !no_panic {
            assert_eq!(actual, expected);
        }

        actual == expected
    }
}

#[test]
fn test_integration_simple_logic1() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    solver.add_expr(x.expr() | y.expr());
    solver.add_expr(x.expr() | !y.expr());
    solver.add_expr(!x.expr() | !y.expr());

    let model = solver.solve();
    assert!(model.is_some());
    let model = model.unwrap();
    assert_eq!(model.get_bool(x), true);
    assert_eq!(model.get_bool(y), false);
}

#[test]
fn test_integration_simple_logic2() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    solver.add_expr(x.expr() ^ y.expr());
    solver.add_expr(x.expr().iff(y.expr()));

    let model = solver.solve();
    assert!(model.is_none());
}

#[test]
fn test_integration_simple_logic3() {
    let mut solver = IntegratedSolver::new();

    let v = solver.new_bool_var();
    let w = solver.new_bool_var();
    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    let z = solver.new_bool_var();
    solver.add_expr(v.expr() ^ w.expr());
    solver.add_expr(w.expr() ^ x.expr());
    solver.add_expr(x.expr() ^ y.expr());
    solver.add_expr(y.expr() ^ z.expr());
    solver.add_expr(z.expr() | v.expr());

    let model = solver.solve();
    assert!(model.is_some());
    let model = model.unwrap();
    assert_eq!(model.get_bool(v), true);
    assert_eq!(model.get_bool(w), false);
    assert_eq!(model.get_bool(x), true);
    assert_eq!(model.get_bool(y), false);
    assert_eq!(model.get_bool(z), true);
}

#[test]
fn test_integration_simple_logic4() {
    let mut solver = IntegratedSolver::new();

    let v = solver.new_bool_var();
    let w = solver.new_bool_var();
    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    let z = solver.new_bool_var();
    solver.add_expr(v.expr() ^ w.expr());
    solver.add_expr(w.expr() ^ x.expr());
    solver.add_expr(x.expr() ^ y.expr());
    solver.add_expr(y.expr() ^ z.expr());
    solver.add_expr(z.expr() ^ v.expr());

    let model = solver.solve();
    assert!(model.is_none());
}

#[test]
fn test_integration_simple_linear1() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(0, 2));
    let b = solver.new_int_var(Domain::range(0, 2));
    solver.add_expr((a.expr() + b.expr()).ge(IntExpr::Const(3)));
    solver.add_expr(a.expr().gt(b.expr()));

    let model = solver.solve();
    assert!(model.is_some());
    let model = model.unwrap();
    assert_eq!(model.get_int(a), 2);
    assert_eq!(model.get_int(b), 1);
}

#[test]
fn test_integration_simple_linear2() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(1, 4));
    let b = solver.new_int_var(Domain::range(1, 4));
    let c = solver.new_int_var(Domain::range(1, 4));
    solver.add_expr((a.expr() + b.expr() + c.expr()).ge(IntExpr::Const(9)));
    solver.add_expr(a.expr().gt(b.expr()));
    solver.add_expr(b.expr().gt(c.expr()));

    let model = solver.solve();
    assert!(model.is_some());
    let model = model.unwrap();
    assert_eq!(model.get_int(a), 4);
    assert_eq!(model.get_int(b), 3);
    assert_eq!(model.get_int(c), 2);
}

#[test]
fn test_integration_simple_linear3() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(3, 4));
    let b = solver.new_int_var(Domain::range(1, 2));
    let c = solver.new_int_var(Domain::range(1, 2));
    solver.add_expr(a.expr().ne(b.expr() + c.expr()));
    solver.add_expr(b.expr().gt(c.expr()));

    let model = solver.solve();
    assert!(model.is_some());
    let model = model.unwrap();
    assert_eq!(model.get_int(a), 4);
    assert_eq!(model.get_int(b), 2);
    assert_eq!(model.get_int(c), 1);
}

#[test]
fn test_integration_simple_linear4() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(1, 2));
    let b = solver.new_int_var(Domain::range(1, 2));
    let c = solver.new_int_var(Domain::range(1, 2));
    solver.add_expr(a.expr().ne(b.expr()));
    solver.add_expr(b.expr().ne(c.expr()));
    solver.add_expr(c.expr().ne(a.expr()));

    let model = solver.solve();
    assert!(model.is_none());
}

#[test]
fn test_integration_simple_linear5() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(1, 2));
    let b = solver.new_int_var(Domain::range(1, 2));
    let c = solver.new_int_var(Domain::range(1, 2));
    solver.add_expr(a.expr().ne(b.expr()));
    solver.add_expr(b.expr().ne(c.expr()));
    solver.add_expr(c.expr().ne(a.expr()) | (a.expr() + c.expr()).eq(b.expr()));

    let model = solver.solve();
    assert!(model.is_some());
    let model = model.unwrap();
    assert_eq!(model.get_int(a), 1);
    assert_eq!(model.get_int(b), 2);
    assert_eq!(model.get_int(c), 1);
}

#[test]
fn test_integration_simple_alldifferent() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(1, 2));
    let b = solver.new_int_var(Domain::range(1, 2));
    let c = solver.new_int_var(Domain::range(1, 2));
    solver.add_constraint(Stmt::AllDifferent(vec![a.expr(), b.expr(), c.expr()]));

    let model = solver.solve();
    assert!(model.is_none());
}

#[test]
fn test_integration_unused_bool() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    let z = solver.new_bool_var();
    solver.add_expr(y.expr() | z.expr());

    let model = solver.solve();
    assert!(model.is_some());
    let model = model.unwrap();
    let _ = model.get_bool(x);
    let _ = model.get_bool(y);
    let _ = model.get_bool(z);
}

#[test]
fn test_integration_unused_int() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(0, 2));
    let b = solver.new_int_var(Domain::range(0, 2));
    let c = solver.new_int_var(Domain::range(0, 2));
    solver.add_expr(a.expr().gt(b.expr()));

    let model = solver.solve();
    assert!(model.is_some());
    let model = model.unwrap();
    let _ = model.get_int(a);
    let _ = model.get_int(b);
    let _ = model.get_int(c);
}

#[test]
fn test_integration_unused_then_used_bool() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    let z = solver.new_bool_var();
    solver.add_expr(y.expr() | z.expr());

    {
        let model = solver.solve();
        assert!(model.is_some());
        let model = model.unwrap();
        let _ = model.get_bool(x);
        let _ = model.get_bool(y);
        let _ = model.get_bool(z);
    }

    solver.add_expr(!x.expr());
    {
        let model = solver.solve();
        assert!(model.is_some());
        let model = model.unwrap();
        assert_eq!(model.get_bool(x), false);
    }

    solver.add_expr(x.expr() | !(y.expr() | z.expr()));
    let model = solver.solve();
    assert!(model.is_none());
}

#[test]
fn test_integration_solve_twice() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    solver.add_expr((x.expr() ^ BoolExpr::Const(false)) | (y.expr() ^ BoolExpr::Const(true)));

    {
        let model = solver.solve();
        assert!(model.is_some());
    }

    solver.add_expr(x.expr() ^ y.expr());
    {
        let model = solver.solve();
        assert!(model.is_some());
    }
}

#[test]
fn test_integration_solve_twice_propagation() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    solver.add_expr(x.expr() | y.expr());

    {
        let model = solver.solve();
        assert!(model.is_some());
    }

    solver.add_expr(!x.expr());
    {
        let model = solver.solve();
        assert!(model.is_some());
    }

    solver.add_expr(!y.expr());
    {
        let model = solver.solve();
        assert!(model.is_none());
    }
}

#[test]
fn test_integration_bool_lit_after_decomposition() {
    let mut config = Config::default();
    config.domain_product_threshold = 1;
    let mut solver = IntegratedSolver::with_config(config);

    let x = solver.new_bool_var();

    let a = solver.new_int_var(Domain::range(0, 5));
    let b = solver.new_int_var(Domain::range(0, 5));
    let c = solver.new_int_var(Domain::range(0, 5));
    let d = solver.new_int_var(Domain::range(0, 5));
    let e = solver.new_int_var(Domain::range(0, 5));

    solver.add_expr(
        x.expr()
            .imp((a.expr() + b.expr() + c.expr() + d.expr()).le(e.expr())),
    );
    solver.add_expr(x.expr().imp(a.expr().ge(IntExpr::Const(4))));
    solver.add_expr(x.expr().imp(b.expr().ge(IntExpr::Const(4))));
    solver.add_expr((!x.expr()).imp(a.expr().ge(IntExpr::Const(4))));
    solver.add_expr((!x.expr()).imp(b.expr().ge(IntExpr::Const(4))));
    solver.add_expr((!x.expr()).imp(c.expr().ge(IntExpr::Const(4))));
    solver.add_expr((!x.expr()).imp(d.expr().ge(IntExpr::Const(4))));
    solver.add_expr((!x.expr()).imp(e.expr().ge(IntExpr::Const(4))));

    let res = solver.solve();
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.get_bool(x), false);
}

#[test]
fn test_integration_csp_optimization1() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    solver.add_expr(x.expr() & !y.expr());

    let res = solver.solve();
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.get_bool(x), true);
    assert_eq!(res.get_bool(y), false);
}

#[test]
fn test_integration_csp_optimization2() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    solver.add_expr(x.expr() | x.expr());
    solver.add_expr(!x.expr());

    let res = solver.solve();
    assert!(res.is_none());
}

#[test]
fn test_integration_csp_optimization3() {
    let mut solver = IntegratedSolver::new();

    let mut vars = vec![];
    let mut coefs = vec![];
    for _ in 0..15 {
        let v = solver.new_bool_var();
        vars.push(v);
        coefs.push((
            Box::new(v.expr().ite(IntExpr::Const(1), IntExpr::Const(0))),
            1,
        ));
    }
    solver.add_expr(vars[0].expr());
    solver.add_expr(IntExpr::Linear(coefs).eq(IntExpr::Const(0)));

    let res = solver.solve();
    assert!(res.is_none());
}

#[test]
fn test_integration_irrefutable_logic1() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let y = solver.new_bool_var();
    let z = solver.new_bool_var();
    solver.add_expr(x.expr() | y.expr());
    solver.add_expr(y.expr() | z.expr());
    solver.add_expr(!(x.expr() & z.expr()));

    let res = solver.decide_irrefutable_facts(&[x, y, z], &[]);
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.get_bool(x), None);
    assert_eq!(res.get_bool(y), Some(true));
    assert_eq!(res.get_bool(z), None);
}

#[test]
fn test_integration_irrefutable_complex1() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let a = solver.new_int_var(Domain::range(0, 2));
    let b = solver.new_int_var(Domain::range(0, 2));
    solver.add_expr(x.expr().ite(a.expr(), b.expr()).eq(a.expr()));
    solver.add_expr(a.expr().ne(b.expr()));

    let res = solver.decide_irrefutable_facts(&[x], &[a, b]);
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.get_bool(x), Some(true));
    assert_eq!(res.get_int(a), None);
    assert_eq!(res.get_int(b), None);
}

#[test]
fn test_integration_irrefutable_complex2() {
    let mut solver = IntegratedSolver::new();

    let x = solver.new_bool_var();
    let a = solver.new_int_var(Domain::range(0, 2));
    let b = solver.new_int_var(Domain::range(0, 2));
    let c = solver.new_int_var(Domain::range(0, 2));
    solver.add_expr(
        x.expr()
            .ite(a.expr(), b.expr())
            .lt(c.expr() - IntExpr::Const(1)),
    );
    solver.add_expr(a.expr().ne(b.expr()));

    let res = solver.decide_irrefutable_facts(&[x], &[a, b, c]);
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.get_bool(x), None);
    assert_eq!(res.get_int(a), None);
    assert_eq!(res.get_int(b), None);
    assert_eq!(res.get_int(c), Some(2));
}

#[test]
fn test_integration_irrefutable_many_terms() {
    let mut solver = IntegratedSolver::new();

    let mut ivars = vec![];
    for _ in 0..30 {
        ivars.push(solver.new_int_var(Domain::range(0, 1)));
    }
    solver.add_expr(
        IntExpr::Linear(ivars.iter().map(|v| (Box::new(v.expr()), 1)).collect())
            .ge(IntExpr::Const(10)),
    );

    let x = solver.new_bool_var();
    solver.add_expr(
        IntExpr::Linear(ivars.iter().map(|v| (Box::new(v.expr()), 1)).collect())
            .ge(IntExpr::Const(9))
            .iff(x.expr()),
    );

    let res = solver.decide_irrefutable_facts(&[x], &ivars);
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.get_bool(x), Some(true));
    assert_eq!(res.get_int(ivars[0]), None);
}

#[test]
fn test_integration_irrefutable_alldifferent() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(1, 3));
    let b = solver.new_int_var(Domain::range(1, 3));
    let c = solver.new_int_var(Domain::range(1, 3));
    let d = solver.new_int_var(Domain::range(1, 4));
    solver.add_constraint(Stmt::AllDifferent(vec![
        a.expr(),
        b.expr(),
        c.expr(),
        d.expr(),
    ]));

    let res = solver.decide_irrefutable_facts(&[], &[a, b, c, d]);
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.get_int(a), None);
    assert_eq!(res.get_int(b), None);
    assert_eq!(res.get_int(c), None);
    assert_eq!(res.get_int(d), Some(4));
}

#[test]
fn test_integration_solver_iterator() {
    let mut solver = IntegratedSolver::new();

    let a = solver.new_int_var(Domain::range(1, 3));
    let b = solver.new_int_var(Domain::range(1, 3));
    let x = solver.new_bool_var();

    solver.add_expr((a.expr() + b.expr()).ge(x.expr().ite(IntExpr::Const(3), IntExpr::Const(4))));

    let mut n_ans = 0;
    for _ in solver.answer_iter(&[x], &[a, b]) {
        n_ans += 1;
    }
    assert_eq!(n_ans, 14);
}

#[test]
fn test_integration_perf_stats() {
    let perf_stats = PerfStats::new();
    let mut solver = IntegratedSolver::new();
    solver.set_perf_stats(&perf_stats);

    let a = solver.new_int_var(Domain::range(0, 5));
    let b = solver.new_int_var(Domain::range(0, 5));
    solver.add_expr((a.expr() + b.expr()).ge(IntExpr::Const(4)));
    solver.add_expr((a.expr() - b.expr()).le(IntExpr::Const(2)));

    let mut propagations_prev = 0;
    let mut n_ans = 0;
    for _ in solver.answer_iter(&[], &[a, b]) {
        assert!(propagations_prev < perf_stats.propagations());
        propagations_prev = perf_stats.propagations();
        n_ans += 1;
    }
    assert_eq!(n_ans, 21);
}

#[test]
fn test_integration_seed() {
    let mut propagations = vec![];
    for i in 1..=10 {
        let perf_stats = PerfStats::new();
        let mut solver = IntegratedSolver::new();
        solver.set_perf_stats(&perf_stats);
        solver.sat.set_rnd_init_act(true);
        solver.sat.set_seed(i as f64 / 10.0);

        let a = solver.new_int_var(Domain::range(0, 5));
        let b = solver.new_int_var(Domain::range(0, 5));
        solver.add_expr((a.expr() + b.expr()).ge(IntExpr::Const(4)));
        solver.add_expr((a.expr() - b.expr()).le(IntExpr::Const(2)));

        let mut propagations_prev = 0;
        let mut n_ans = 0;
        for _ in solver.answer_iter(&[], &[a, b]) {
            assert!(propagations_prev < perf_stats.propagations());
            propagations_prev = perf_stats.propagations();
            n_ans += 1;
        }
        assert_eq!(n_ans, 21);
        propagations.push(perf_stats.propagations());
    }
    assert!(propagations.iter().any(|&p| p != propagations[0]));
}

#[test]
fn test_integration_exhaustive_bool1() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let y = tester.new_bool_var();
    let z = tester.new_bool_var();
    let w = tester.new_bool_var();
    tester.add_expr(x.expr().imp(y.expr() ^ z.expr()));
    tester.add_expr(y.expr().imp(z.expr().iff(w.expr())));

    tester.check();
}

#[test]
fn test_integration_exhaustive_linear1() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var(Domain::range(0, 2));
    let b = tester.new_int_var(Domain::range(0, 2));
    let c = tester.new_int_var(Domain::range(0, 2));
    tester.add_expr((a.expr() + b.expr() + c.expr()).ge(IntExpr::Const(3)));

    tester.check();
}

#[test]
fn test_integration_exhaustive_linear2() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var(Domain::range(0, 3));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    let d = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr((a.expr() + b.expr() + c.expr()).ge(IntExpr::Const(5)));
    tester.add_expr((b.expr() + c.expr() + d.expr()).le(IntExpr::Const(5)));

    tester.check();
}

#[test]
fn test_integration_exhaustive_linear3() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var(Domain::range(0, 4));
    let b = tester.new_int_var(Domain::range(0, 4));
    let c = tester.new_int_var(Domain::range(0, 4));
    let d = tester.new_int_var(Domain::range(0, 4));
    tester.add_expr((a.expr() * 2 - b.expr() + c.expr() * 3 + d.expr()).ge(IntExpr::Const(10)));
    tester.add_expr((a.expr() + b.expr() * 4 - c.expr() * 2 - d.expr() * 3).le(IntExpr::Const(2)));

    tester.check();
}

#[cfg(feature = "csp-extra-constraints")]
#[test]
fn test_integration_exhaustive_mul1() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var(Domain::range(-5, 5));
    let b = tester.new_int_var(Domain::range(-4, 4));
    let c = tester.new_int_var(Domain::range(-4, 4));
    let d = tester.new_int_var(Domain::range(-4, 4));
    tester.add_expr((a.expr() * b.expr()).eq(c.expr() * d.expr() + IntExpr::Const(1)));

    tester.check();
}

#[cfg(feature = "csp-extra-constraints")]
#[test]
fn test_integration_exhaustive_mul2() {
    let mut config = Config::default();
    config.force_use_log_encoding = true;
    let mut tester = IntegrationTester::with_config(config);

    let a = tester.new_int_var(Domain::range(1, 100));
    let b = tester.new_int_var(Domain::range(1, 100));
    let c = tester.new_int_var(Domain::range(1, 100));
    tester.add_expr((a.expr() * a.expr() + b.expr() * b.expr()).eq(c.expr() * c.expr()));

    tester.check();
}

#[test]
fn test_integration_exhaustive_complex1() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let y = tester.new_bool_var();
    let z = tester.new_bool_var();
    let a = tester.new_int_var(Domain::range(0, 3));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr(
        x.expr()
            .ite(a.expr(), b.expr() + c.expr())
            .eq(a.expr() - b.expr()),
    );
    tester.add_expr(a.expr().ge(y.expr().ite(b.expr(), c.expr())) ^ z.expr());

    tester.check();
}

#[test]
fn test_integration_exhaustive_complex2() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let y = tester.new_bool_var();
    let z = tester.new_bool_var();
    let a = tester.new_int_var(Domain::range(0, 3));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr(
        x.expr()
            .ite(a.expr(), b.expr() + c.expr())
            .eq(a.expr() - b.expr()),
    );
    tester.add_expr(a.expr().ge(y.expr().ite(b.expr(), c.expr())) ^ z.expr());
    tester.add_expr(x.expr());

    tester.check();
}

#[test]
fn test_integration_exhaustive_complex3() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let y = tester.new_bool_var();
    let z = tester.new_bool_var();
    let a = tester.new_int_var(Domain::range(0, 3));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr(x.expr() | (a.expr().ge(IntExpr::Const(2))));
    tester.add_expr(y.expr() | (b.expr().eq(IntExpr::Const(2))) | (c.expr().ne(IntExpr::Const(1))));
    tester.add_expr(
        (z.expr().ite(IntExpr::Const(1), IntExpr::Const(2)) + b.expr()).ge(a.expr() + c.expr()),
    );

    tester.check();
}

#[test]
fn test_integration_exhaustive_complex4() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let y = tester.new_bool_var();
    let z = tester.new_bool_var();
    let a = tester.new_int_var(Domain::range(-3, 3));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr(x.expr() | (a.expr().ge(IntExpr::Const(2))));
    tester.add_expr(y.expr() | (b.expr().eq(IntExpr::Const(2))) | (c.expr().ne(IntExpr::Const(1))));
    tester.add_expr(
        (z.expr().ite(IntExpr::Const(1), IntExpr::Const(2)) + b.expr())
            .ge(a.expr().abs() + c.expr()),
    );

    tester.check();
}

#[test]
fn test_integration_exhaustive_enumerative1() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let y = tester.new_bool_var();
    let z = tester.new_bool_var();
    let a = tester.new_int_var(Domain::range(0, 2));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr(
        x.expr().ite(IntExpr::Const(3), b.expr() + c.expr()).eq(a
            .expr()
            .ne(IntExpr::Const(2))
            .ite(IntExpr::Const(1), IntExpr::Const(3))
            - b.expr()),
    );
    tester.add_expr(
        a.expr()
            .ne(IntExpr::Const(0))
            .ite(IntExpr::Const(2), IntExpr::Const(1))
            .ge(y.expr().ite(b.expr(), c.expr()))
            ^ z.expr(),
    );
    tester.add_expr(x.expr() ^ a.expr().eq(IntExpr::Const(1)));

    tester.check();
}

#[test]
fn test_integration_exhaustive_enumerative2() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let y = tester.new_bool_var();
    let z = tester.new_bool_var();
    let a = tester.new_int_var(Domain::range(0, 2));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr(x.expr().iff(a.expr().eq(IntExpr::Const(0))));
    tester.add_expr(y.expr().iff(b.expr().ne(IntExpr::Const(1))));
    tester.add_expr(z.expr().iff(c.expr().eq(IntExpr::Const(2))));
    tester.check();
}

#[test]
fn test_integration_exhaustive_enumerative3() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var(Domain::range(0, 1));
    let b = tester.new_int_var(Domain::range(0, 1));
    let c = tester.new_int_var(Domain::range(0, 1));
    tester.add_expr(a.expr().eq(b.expr()) | b.expr().eq(c.expr()));
    tester.check();
}

#[test]
fn test_integration_exhaustive_binary1() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let a = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr(
        x.expr()
            .ite(IntExpr::Const(2), IntExpr::Const(3))
            .ge(a.expr()),
    );

    tester.check();
}

#[test]
fn test_integration_exhaustive_binary2() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let y = tester.new_bool_var();
    let z = tester.new_bool_var();
    let a = tester.new_int_var(Domain::range(0, 3));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    tester.add_expr(
        (x.expr().ite(IntExpr::Const(1), IntExpr::Const(2))
            + y.expr().ite(IntExpr::Const(2), IntExpr::Const(1)))
        .ge(a.expr() + b.expr() * 2 - c.expr()),
    );
    tester.add_expr((a.expr() + z.expr().ite(IntExpr::Const(1), IntExpr::Const(0))).le(c.expr()));
    tester.add_expr(x.expr() | z.expr());

    tester.check();
}

#[test]
fn test_integration_exhaustive_binary3() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    tester.add_expr(
        x.expr()
            .ite(IntExpr::Const(1), IntExpr::Const(0))
            .eq(IntExpr::Const(1)),
    );

    tester.check();
}

#[test]
fn test_integration_exhaustive_binary4() {
    let mut config = Config::default();
    config.direct_encoding_for_binary_vars = true;
    let mut tester = IntegrationTester::with_config(config);

    let x = tester.new_bool_var();
    tester.add_expr(
        x.expr()
            .ite(IntExpr::Const(1), IntExpr::Const(0))
            .eq(IntExpr::Const(1))
            | x.expr(),
    );

    tester.check();
}

#[test]
fn test_integration_exhaustive_alldifferent() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var(Domain::range(0, 3));
    let b = tester.new_int_var(Domain::range(0, 3));
    let c = tester.new_int_var(Domain::range(1, 4));
    tester.add_constraint(Stmt::AllDifferent(vec![a.expr(), b.expr(), c.expr()]));
    tester.add_expr((a.expr() + b.expr()).ge(c.expr()));

    tester.check();
}

#[test]
fn test_integration_domain_list1() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var_from_list(vec![0, 1, 2, 3, 5]);
    let b = tester.new_int_var_from_list(vec![0, 1, 3, 4, 5]);
    let c = tester.new_int_var_from_list(vec![0, 2, 3, 4, 5]);
    tester.add_constraint(Stmt::AllDifferent(vec![a.expr(), b.expr(), c.expr()]));
    tester.add_expr((a.expr() + b.expr()).ge(c.expr()));

    tester.check();
}

#[test]
fn test_integration_many_terms() {
    for c in [-3, 0, 3, 12, 13] {
        let mut config = Config::default();
        config.native_linear_encoding_terms = 0;
        config.domain_product_threshold = 2;
        let mut tester = IntegrationTester::with_config(config);

        let mut vars = vec![];
        for _ in 0..12 {
            vars.push(tester.new_int_var(Domain::range(0, 1)));
        }

        let mut expr = IntExpr::Const(0);
        for v in &vars {
            expr = expr + v.expr();
        }

        tester.add_expr(expr.ge(IntExpr::Const(c)));
        tester.check();
    }
}

#[test]
fn test_integration_many_terms_not_equal() {
    for c in [-1, 0, 1, 13, 14] {
        let mut config = Config::default();
        config.native_linear_encoding_terms = 0;
        config.domain_product_threshold = 10;
        let mut tester = IntegrationTester::with_config(config);

        let mut vars = vec![];
        for _ in 0..13 {
            vars.push(tester.new_int_var(Domain::range(0, 1)));
        }

        let mut expr = IntExpr::Const(0);
        for v in &vars {
            expr = expr + v.expr();
        }

        tester.add_expr(expr.ne(IntExpr::Const(c)));
        tester.check();
    }
}

#[cfg(feature = "csp-extra-constraints")]
#[test]
fn test_integration_exhaustive_circuit1() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var(Domain::range(0, 5));
    let b = tester.new_int_var(Domain::range(-1, 3));
    let c = tester.new_int_var(Domain::range(0, 3));
    let d = tester.new_int_var(Domain::range(1, 3));

    tester.add_constraint(Stmt::Circuit(vec![a.expr(), b.expr(), c.expr(), d.expr()]));

    tester.check();
}

#[cfg(feature = "csp-extra-constraints")]
#[test]
fn test_integration_exhaustive_circuit2() {
    let mut tester = IntegrationTester::new();

    let a = tester.new_int_var_from_list(vec![1, 2, 3, 4]);
    let b = tester.new_int_var_from_list(vec![0, 2, 3, 4]);
    let c = tester.new_int_var_from_list(vec![0, 1, 4]);
    let d = tester.new_int_var_from_list(vec![0, 2, 4]);
    let e = tester.new_int_var_from_list(vec![0, 1, 2, 3]);

    tester.add_constraint(Stmt::Circuit(vec![
        a.expr(),
        b.expr(),
        c.expr(),
        d.expr(),
        e.expr(),
    ]));

    tester.check();
}

#[cfg(feature = "csp-extra-constraints")]
#[test]
fn test_integration_exhaustive_extension_supports1() {
    for use_native in [false, true] {
        let mut config = Config::default();
        config.use_native_extension_supports = use_native;
        let mut tester = IntegrationTester::with_config(config);

        let a = tester.new_int_var_from_list(vec![0, 2, 3, 4]);
        let b = tester.new_int_var(Domain::range(0, 4));
        let c = tester.new_int_var(Domain::range(0, 4));
        let d = tester.new_int_var(Domain::range(1, 4));

        tester.add_constraint(Stmt::ExtensionSupports(
            vec![a.expr(), b.expr(), c.expr()],
            vec![
                vec![Some(0), Some(0), Some(1)],
                vec![Some(0), Some(2), Some(1)],
                vec![Some(0), Some(2), Some(3)],
                vec![Some(0), Some(3), Some(4)],
                vec![Some(1), Some(2), Some(4)],
                vec![Some(2), Some(1), Some(1)],
                vec![Some(2), Some(2), Some(2)],
                vec![Some(3), Some(3), Some(2)],
                vec![Some(4), Some(4), Some(0)],
            ],
        ));
        tester.add_constraint(Stmt::ExtensionSupports(
            vec![b.expr(), c.expr(), d.expr()],
            vec![
                vec![Some(0), None, None],
                vec![None, Some(1), None],
                vec![Some(2), None, Some(2)],
                vec![None, Some(3), Some(4)],
            ],
        ));

        tester.check();
    }
}

#[test]
fn test_integration_active_vertices_connected1() {
    let mut tester = IntegrationTester::new();

    let mut vars = vec![];
    for _ in 0..9 {
        vars.push(tester.new_bool_var().expr());
    }

    tester.add_constraint(Stmt::ActiveVerticesConnected(
        vars,
        vec![
            (0, 1),
            (1, 2),
            (3, 4),
            (4, 5),
            (6, 7),
            (7, 8),
            (0, 3),
            (1, 4),
            (2, 5),
            (3, 6),
            (4, 7),
            (5, 8),
        ],
    ));

    tester.check();
}

#[test]
fn test_integration_active_vertices_connected2() {
    for p in 0..7 {
        for q in 0..7 {
            let mut tester = IntegrationTester::new();

            let mut vars = vec![];
            for _ in 0..7 {
                vars.push(tester.new_bool_var().expr());
            }
            vars.push(vars[p].clone());
            vars.push(!vars[q].clone());

            tester.add_constraint(Stmt::ActiveVerticesConnected(
                vars,
                vec![
                    (0, 1),
                    (1, 2),
                    (3, 4),
                    (4, 5),
                    (6, 7),
                    (7, 8),
                    (0, 3),
                    (1, 4),
                    (2, 5),
                    (3, 6),
                    (4, 7),
                    (5, 8),
                ],
            ));

            tester.check();
        }
    }
}

#[test]
fn test_integration_graph_division1() {
    let mut tester = IntegrationTester::new();

    let mut vars = vec![];
    for _ in 0..12 {
        vars.push(tester.new_bool_var().expr());
    }

    let a = tester.new_int_var(Domain::range(2, 3));
    let b = tester.new_int_var(Domain::range(4, 5));

    tester.add_constraint(Stmt::GraphDivision(
        vec![
            Some(a.expr()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(b.expr()),
        ],
        vec![
            (0, 1),
            (1, 2),
            (3, 4),
            (4, 5),
            (6, 7),
            (7, 8),
            (0, 3),
            (1, 4),
            (2, 5),
            (3, 6),
            (4, 7),
            (5, 8),
        ],
        vars,
        GraphDivisionOptions::default(),
    ));

    tester.check();
}

#[test]
fn test_integration_graph_division2() {
    let mut tester = IntegrationTester::new();

    let mut vars = vec![];
    for _ in 0..10 {
        vars.push(tester.new_bool_var().expr());
    }

    tester.add_expr(vars[5].clone());
    tester.add_constraint(Stmt::GraphDivision(
        vec![None; 8],
        vec![
            (0, 1),
            (0, 3),
            (1, 2),
            (1, 4),
            (2, 5),
            (3, 4),
            (3, 6),
            (4, 5),
            (4, 7),
            (6, 7),
        ],
        vars,
        GraphDivisionOptions::default(),
    ));

    tester.check();
}

#[test]
fn test_integration_binary_var1() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let a = x.expr().ite(IntExpr::Const(1), IntExpr::Const(2));

    let y = tester.new_bool_var();
    let b = y.expr().ite(IntExpr::Const(2), IntExpr::Const(1));
    tester.add_expr(a.eq(b));

    tester.check();
}

#[test]
fn test_integration_binary_var2() {
    let mut tester = IntegrationTester::new();

    let x = tester.new_bool_var();
    let a = x.expr().ite(IntExpr::Const(1), IntExpr::Const(2));

    let y = tester.new_bool_var();
    let b = y.expr().ite(IntExpr::Const(2), IntExpr::Const(2));
    tester.add_expr(a.eq(b));

    tester.check();
}

struct Fuzzer {
    random_state: u64,
}

impl Fuzzer {
    fn new() -> Self {
        Fuzzer {
            random_state: 0x123456789abcdef,
        }
    }

    fn next_random(&mut self) -> u64 {
        self.random_state = self.random_state.wrapping_mul(0x123456789);
        self.random_state
    }

    fn next_u32(&mut self, max: u32) -> u32 {
        assert!(0 < max);
        ((self.next_random() >> 16) % (max as u64)) as u32
    }

    fn next_i32(&mut self, low: i32, high: i32) -> i32 {
        assert!(low < high);
        let range = (high - low) as u32;
        self.next_u32(range) as i32 + low
    }

    fn run_single_trial(
        &mut self,
        num_bool_vars: usize,
        num_int_vars: usize,
        num_exprs: usize,
        max_complexity: u32,
    ) {
        let mut tester = IntegrationTester::with_config(Config {
            use_log_encoding: false, // do not use log encoding because negative numbers are not supported
            ..Config::default()
        });

        let mut bool_vars = vec![];
        for _ in 0..num_bool_vars {
            bool_vars.push(tester.new_bool_var());
        }

        let mut int_vars = vec![];
        let mut int_var_descs = vec![];
        for _ in 0..num_int_vars {
            if self.next_u32(2) == 0 {
                let a = self.next_i32(-3, 4);
                let b = self.next_i32(-3, 4);
                int_vars.push(tester.new_int_var(Domain::range(a.min(b), a.max(b))));
                int_var_descs.push(format!("{}..{}", a.min(b), a.max(b)));
            } else {
                let mut domain = vec![];
                for n in -3..=3 {
                    if self.next_u32(2) == 0 {
                        domain.push(n);
                    }
                }
                if domain.is_empty() {
                    domain.push(self.next_i32(-3, 4));
                }
                int_var_descs.push(format!("{:?}", domain));
                int_vars.push(tester.new_int_var_from_list(domain));
            }
        }

        let mut exprs = vec![];
        for _ in 0..num_exprs {
            let complexity = self.next_u32(max_complexity);

            let expr = self.random_bool_expr(&bool_vars, &int_vars, complexity);
            exprs.push(expr.clone());
            tester.add_expr(expr);
        }

        if !tester.check_internal(true) {
            eprintln!("Fuzzer failed!");
            eprintln!("Num bool vars: {}", num_bool_vars);
            eprintln!("Int vars:");
            for desc in &int_var_descs {
                eprintln!("- {}", desc);
            }
            eprintln!("Expressions:");
            for expr in exprs {
                eprint!("- ");
                let mut out_buf = vec![];
                let _ = expr.pretty_print(&mut out_buf);
                eprint!("{}", String::from_utf8(out_buf).unwrap());
                eprintln!();
            }
            panic!();
        }
    }

    fn random_bool_expr(
        &mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
        complexity: u32,
    ) -> BoolExpr {
        if complexity == 0 {
            let idx = self.next_i32(-1, bool_vars.len() as i32);
            if idx < 0 {
                return BoolExpr::Const(self.next_u32(2) == 0);
            } else {
                return bool_vars[idx as usize].expr();
            }
        }

        let mode = self.next_u32(7);
        match mode {
            0 => BoolExpr::Not(Box::new(self.random_bool_expr(
                bool_vars,
                int_vars,
                complexity - 1,
            ))),
            1 | 2 | 3 | 4 | 5 => {
                let left_complexity = self.next_u32(complexity);
                let right_complexity = complexity - left_complexity - 1;

                let lhs = Box::new(self.random_bool_expr(bool_vars, int_vars, left_complexity));
                let rhs = Box::new(self.random_bool_expr(bool_vars, int_vars, right_complexity));

                match mode {
                    1 => BoolExpr::And(vec![lhs, rhs]),
                    2 => BoolExpr::Or(vec![lhs, rhs]),
                    3 => BoolExpr::Xor(lhs, rhs),
                    4 => BoolExpr::Iff(lhs, rhs),
                    5 => BoolExpr::Imp(lhs, rhs),
                    _ => unreachable!(),
                }
            }
            6 => {
                let left_complexity = self.next_u32(complexity);
                let right_complexity = complexity - left_complexity - 1;

                let op = match self.next_u32(6) {
                    0 => CmpOp::Eq,
                    1 => CmpOp::Ne,
                    2 => CmpOp::Le,
                    3 => CmpOp::Ge,
                    4 => CmpOp::Lt,
                    5 => CmpOp::Gt,
                    _ => unreachable!(),
                };

                let lhs = Box::new(self.random_int_expr(bool_vars, int_vars, left_complexity));
                let rhs = Box::new(self.random_int_expr(bool_vars, int_vars, right_complexity));

                BoolExpr::Cmp(op, lhs, rhs)
            }
            _ => unreachable!(),
        }
    }

    fn random_int_expr(
        &mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
        complexity: u32,
    ) -> IntExpr {
        if complexity == 0 {
            let idx = self.next_i32(-1, int_vars.len() as i32);
            if idx < 0 {
                return IntExpr::Const(self.next_i32(-3, 4));
            } else {
                return int_vars[idx as usize].expr();
            }
        }

        let mode = self.next_u32(4);
        match mode {
            0 => {
                let cond_complexity = self.next_u32(complexity);
                let t_complexity = self.next_u32(complexity - cond_complexity);
                let f_complexity = complexity - cond_complexity - t_complexity - 1;

                let cond = Box::new(self.random_bool_expr(bool_vars, int_vars, cond_complexity));
                let t_expr = Box::new(self.random_int_expr(bool_vars, int_vars, t_complexity));
                let f_expr = Box::new(self.random_int_expr(bool_vars, int_vars, f_complexity));

                IntExpr::If(cond, t_expr, f_expr)
            }
            1 => IntExpr::Abs(Box::new(self.random_int_expr(
                bool_vars,
                int_vars,
                complexity - 1,
            ))),
            2 => {
                let scale = self.next_i32(-3, 4);
                IntExpr::Linear(vec![(
                    Box::new(self.random_int_expr(bool_vars, int_vars, complexity - 1)),
                    scale,
                )])
            }
            3 => {
                let t1_complexity = self.next_u32(complexity);
                let t2_complexity = complexity - t1_complexity - 1;

                let t1 = Box::new(self.random_int_expr(bool_vars, int_vars, t1_complexity));
                let t2 = Box::new(self.random_int_expr(bool_vars, int_vars, t2_complexity));

                let scale1 = self.next_i32(-3, 4);
                let scale2 = self.next_i32(-3, 4);

                IntExpr::Linear(vec![(t1, scale1), (t2, scale2)])
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_integration_fuzz_quick() {
    let mut fuzzer = Fuzzer::new();
    for _ in 0..1000 {
        let num_bool_vars = fuzzer.next_i32(3, 6) as usize;
        let num_int_vars = fuzzer.next_i32(1, 4) as usize;
        let num_exprs = fuzzer.next_i32(2, 11) as usize;
        let max_complexity = 7;

        fuzzer.run_single_trial(num_bool_vars, num_int_vars, num_exprs, max_complexity);
    }
}

#[test]
#[ignore] // This test can take a long time to run
fn test_integration_fuzz_long() {
    let mut fuzzer = Fuzzer::new();
    for _ in 0..100000 {
        let num_bool_vars = fuzzer.next_i32(3, 7) as usize;
        let num_int_vars = fuzzer.next_i32(1, 5) as usize;
        let num_exprs = fuzzer.next_i32(2, 12) as usize;
        let max_complexity = 10;

        fuzzer.run_single_trial(num_bool_vars, num_int_vars, num_exprs, max_complexity);
    }
}
