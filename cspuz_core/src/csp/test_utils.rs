use super::*;

pub fn eval_bool_expr(assignment: &Assignment, expr: &BoolExpr) -> bool {
    match expr {
        BoolExpr::Const(b) => *b,
        BoolExpr::Var(v) => *(assignment.bool_val.get(v).unwrap()),
        &BoolExpr::NVar(_) => panic!(),
        BoolExpr::And(es) => {
            for e in es {
                if !eval_bool_expr(assignment, e) {
                    return false;
                }
            }
            true
        }
        BoolExpr::Or(es) => {
            for e in es {
                if eval_bool_expr(assignment, e) {
                    return true;
                }
            }
            false
        }
        BoolExpr::Not(e) => !eval_bool_expr(assignment, e),
        BoolExpr::Xor(e1, e2) => eval_bool_expr(assignment, e1) ^ eval_bool_expr(assignment, e2),
        BoolExpr::Iff(e1, e2) => eval_bool_expr(assignment, e1) == eval_bool_expr(assignment, e2),
        BoolExpr::Imp(e1, e2) => !eval_bool_expr(assignment, e1) || eval_bool_expr(assignment, e2),
        BoolExpr::Cmp(op, e1, e2) => {
            let v1 = eval_int_expr(assignment, e1);
            let v2 = eval_int_expr(assignment, e2);
            op.compare(v1, v2)
        }
    }
}

pub fn eval_int_expr(assignment: &Assignment, expr: &IntExpr) -> i32 {
    match expr {
        IntExpr::Const(c) => *c,
        IntExpr::Var(v) => *(assignment.int_val.get(v).unwrap()),
        &IntExpr::NVar(_) => panic!(),
        IntExpr::Linear(es) => {
            let mut ret = 0i32;
            for (e, c) in es {
                ret = ret
                    .checked_add(eval_int_expr(assignment, e).checked_mul(*c).unwrap())
                    .unwrap();
            }
            ret
        }
        IntExpr::If(c, t, f) => eval_int_expr(
            assignment,
            if eval_bool_expr(assignment, c) { t } else { f },
        ),
        IntExpr::Abs(x) => eval_int_expr(assignment, x).abs(),
        IntExpr::Mul(x, y) => eval_int_expr(assignment, x) * eval_int_expr(assignment, y),
    }
}

pub fn clone_stmt(stmt: &Stmt) -> Stmt {
    let cloned = match &stmt {
        Stmt::Expr(e) => Stmt::Expr(e.clone()),
        Stmt::AllDifferent(exprs) => Stmt::AllDifferent(exprs.clone()),
        Stmt::ActiveVerticesConnected(exprs, edges) => {
            Stmt::ActiveVerticesConnected(exprs.clone(), edges.clone())
        }
        Stmt::Circuit(exprs) => Stmt::Circuit(exprs.clone()),
        Stmt::ExtensionSupports(exprs, supports) => {
            Stmt::ExtensionSupports(exprs.clone(), supports.clone())
        }
        Stmt::GraphDivision(sizes, edges, edges_lit, opts) => {
            Stmt::GraphDivision(sizes.clone(), edges.clone(), edges_lit.clone(), *opts)
        }
        Stmt::CustomConstraint(_, _) => {
            panic!("CustomConstraint cannot be cloned");
        }
    };
    cloned
}

mod tests {
    use super::*;

    #[test]
    fn test_eval_expr() {
        let mut assignment = Assignment::new();

        let t = BoolVar::new(0);
        let f = BoolVar::new(1);
        let a = IntVar::new(0);
        let b = IntVar::new(1);

        assignment.set_bool(t, true);
        assignment.set_bool(f, false);
        assignment.set_int(a, 42);
        assignment.set_int(b, 100);

        assert_eq!(eval_bool_expr(&assignment, &BoolExpr::Const(true)), true);

        assert_eq!(eval_bool_expr(&assignment, &t.expr()), true);
        assert_eq!(eval_bool_expr(&assignment, &f.expr()), false);
        assert_eq!(eval_int_expr(&assignment, &a.expr()), 42);
        assert_eq!(eval_int_expr(&assignment, &b.expr()), 100);

        assert_eq!(eval_bool_expr(&assignment, &(t.expr() & t.expr())), true);
        assert_eq!(eval_bool_expr(&assignment, &(t.expr() & f.expr())), false);
        assert_eq!(eval_bool_expr(&assignment, &(f.expr() & t.expr())), false);
        assert_eq!(eval_bool_expr(&assignment, &(f.expr() & f.expr())), false);

        assert_eq!(eval_bool_expr(&assignment, &(t.expr() | t.expr())), true);
        assert_eq!(eval_bool_expr(&assignment, &(t.expr() | f.expr())), true);
        assert_eq!(eval_bool_expr(&assignment, &(f.expr() | t.expr())), true);
        assert_eq!(eval_bool_expr(&assignment, &(f.expr() | f.expr())), false);

        assert_eq!(eval_bool_expr(&assignment, &(!t.expr())), false);
        assert_eq!(eval_bool_expr(&assignment, &(!f.expr())), true);

        assert_eq!(eval_bool_expr(&assignment, &(t.expr() ^ t.expr())), false);
        assert_eq!(eval_bool_expr(&assignment, &(t.expr() ^ f.expr())), true);
        assert_eq!(eval_bool_expr(&assignment, &(f.expr() ^ t.expr())), true);
        assert_eq!(eval_bool_expr(&assignment, &(f.expr() ^ f.expr())), false);

        assert_eq!(eval_bool_expr(&assignment, &(t.expr().iff(t.expr()))), true);
        assert_eq!(
            eval_bool_expr(&assignment, &(t.expr().iff(f.expr()))),
            false
        );
        assert_eq!(
            eval_bool_expr(&assignment, &(f.expr().iff(t.expr()))),
            false
        );
        assert_eq!(eval_bool_expr(&assignment, &(f.expr().iff(f.expr()))), true);

        assert_eq!(eval_bool_expr(&assignment, &(t.expr().imp(t.expr()))), true);
        assert_eq!(
            eval_bool_expr(&assignment, &(t.expr().imp(f.expr()))),
            false
        );
        assert_eq!(eval_bool_expr(&assignment, &(f.expr().imp(t.expr()))), true);
        assert_eq!(eval_bool_expr(&assignment, &(f.expr().imp(f.expr()))), true);

        assert_eq!(
            eval_bool_expr(
                &assignment,
                &BoolExpr::Cmp(
                    crate::arithmetic::CmpOp::Eq,
                    Box::new(a.expr()),
                    Box::new(b.expr())
                )
            ),
            false
        );
        assert_eq!(
            eval_bool_expr(
                &assignment,
                &BoolExpr::Cmp(
                    crate::arithmetic::CmpOp::Ne,
                    Box::new(a.expr()),
                    Box::new(b.expr())
                )
            ),
            true
        );
        assert_eq!(
            eval_bool_expr(
                &assignment,
                &BoolExpr::Cmp(
                    crate::arithmetic::CmpOp::Le,
                    Box::new(a.expr()),
                    Box::new(b.expr())
                )
            ),
            true
        );
        assert_eq!(
            eval_bool_expr(
                &assignment,
                &BoolExpr::Cmp(
                    crate::arithmetic::CmpOp::Lt,
                    Box::new(a.expr()),
                    Box::new(b.expr())
                )
            ),
            true
        );
        assert_eq!(
            eval_bool_expr(
                &assignment,
                &BoolExpr::Cmp(
                    crate::arithmetic::CmpOp::Ge,
                    Box::new(a.expr()),
                    Box::new(b.expr())
                )
            ),
            false
        );
        assert_eq!(
            eval_bool_expr(
                &assignment,
                &BoolExpr::Cmp(
                    crate::arithmetic::CmpOp::Gt,
                    Box::new(a.expr()),
                    Box::new(b.expr())
                )
            ),
            false
        );

        assert_eq!(eval_int_expr(&assignment, &IntExpr::Const(42)), 42);

        assert_eq!(
            eval_int_expr(
                &assignment,
                &IntExpr::Linear(vec![(Box::new(a.expr()), 2), (Box::new(b.expr()), 3)])
            ),
            2 * 42 + 3 * 100
        );

        assert_eq!(
            eval_int_expr(
                &assignment,
                &IntExpr::If(Box::new(t.expr()), Box::new(a.expr()), Box::new(b.expr()))
            ),
            42
        );
        assert_eq!(
            eval_int_expr(
                &assignment,
                &IntExpr::If(Box::new(f.expr()), Box::new(a.expr()), Box::new(b.expr()))
            ),
            100
        );

        assert_eq!(
            eval_int_expr(&assignment, &IntExpr::Abs(Box::new(a.expr()))),
            42
        );
        assert_eq!(
            eval_int_expr(
                &assignment,
                &IntExpr::Abs(Box::new(IntExpr::Linear(vec![(Box::new(a.expr()), -1)])))
            ),
            42
        );

        assert_eq!(
            eval_int_expr(
                &assignment,
                &IntExpr::Mul(Box::new(a.expr()), Box::new(b.expr()))
            ),
            42 * 100
        );
    }
}
