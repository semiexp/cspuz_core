use super::*;

pub fn is_norm_csp_satisfied(assignment: &Assignment, norm_csp: &NormCSP) -> bool {
    for constraint in &norm_csp.constraints {
        if !assignment.eval_constraint(constraint) {
            return false;
        }
    }

    for constr in &norm_csp.extra_constraints {
        match constr {
            ExtraConstraint::ActiveVerticesConnected(is_active, edges) => {
                let is_active = is_active
                    .iter()
                    .map(|&v| assignment.get_bool(v.var).unwrap() ^ v.negated)
                    .collect::<Vec<_>>();
                if !crate::test_utils::check_graph_active_vertices_connected(&is_active, &edges) {
                    return false;
                }
            }
            &ExtraConstraint::Mul(x, y, m) => {
                let val_x = assignment.get_int(x).unwrap();
                let val_y = assignment.get_int(y).unwrap();
                let val_m = assignment.get_int(m).unwrap();
                if val_x * val_y != val_m {
                    return false;
                }
            }
            ExtraConstraint::ExtensionSupports(vars, supports) => {
                let values = vars
                    .iter()
                    .map(|&v| assignment.get_int(v).unwrap())
                    .collect::<Vec<_>>();
                let mut isok = false;
                for support in supports {
                    let mut flg = true;
                    for i in 0..values.len() {
                        if let Some(n) = support[i] {
                            if values[i] != n {
                                flg = false;
                                break;
                            }
                        }
                    }
                    if flg {
                        isok = true;
                        break;
                    }
                }
                if !isok {
                    return false;
                }
            }
            ExtraConstraint::GraphDivision(_, _, _, _) => todo!(),
            ExtraConstraint::CustomConstraint(_, _) => todo!(),
        }
    }
    true
}

pub fn norm_csp_all_assignments(norm_csp: &NormCSP) -> Vec<Assignment> {
    let mut ret = vec![];

    let bool_domains = vec![vec![false, true]; norm_csp.vars.num_bool_var];
    let mut int_domains = vec![];
    for i in 0..norm_csp.vars.num_int_vars() {
        int_domains.push(norm_csp.vars.int_var[i].enumerate());
    }

    for (vb, vi) in crate::test_utils::product_binary(
        &crate::test_utils::product_multi(&bool_domains),
        &crate::test_utils::product_multi(&int_domains),
    ) {
        let mut assignment = Assignment::new();
        for (v, b) in norm_csp.vars.bool_vars_iter().zip(vb) {
            assignment.set_bool(v, b);
        }
        for (v, i) in norm_csp.vars.int_vars_iter().zip(vi) {
            assignment.set_int(v, i.get());
        }

        if is_norm_csp_satisfied(&assignment, norm_csp) {
            ret.push(assignment);
        }
    }

    ret
}
