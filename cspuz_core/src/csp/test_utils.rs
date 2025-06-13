use super::Stmt;

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
