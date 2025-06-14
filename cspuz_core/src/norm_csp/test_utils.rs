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

pub fn norm_csp_all_assignments_for_vars(
    norm_csp: &NormCSP,
    bool_vars: &[BoolVar],
    int_vars: &[IntVar],
) -> Vec<Assignment> {
    let mut ret = vec![];

    let bool_domains = vec![vec![false, true]; bool_vars.len()];
    let mut int_domains = vec![];
    for v in int_vars {
        int_domains.push(norm_csp.vars.int_var(*v).enumerate());
    }

    for (vb, vi) in crate::test_utils::product_binary(
        &crate::test_utils::product_multi(&bool_domains),
        &crate::test_utils::product_multi(&int_domains),
    ) {
        let mut assignment = Assignment::new();
        for (v, b) in bool_vars.iter().zip(vb) {
            assignment.set_bool(*v, b);
        }
        for (v, i) in int_vars.iter().zip(vi) {
            assignment.set_int(*v, i.get());
        }

        if is_norm_csp_satisfied(&assignment, norm_csp) {
            ret.push(assignment);
        }
    }

    ret
}

pub fn norm_csp_all_assignments(norm_csp: &NormCSP) -> Vec<Assignment> {
    let bool_vars = norm_csp.vars.bool_vars_iter().collect::<Vec<_>>();
    let int_vars = norm_csp.vars.int_vars_iter().collect::<Vec<_>>();

    norm_csp_all_assignments_for_vars(norm_csp, &bool_vars, &int_vars)
}
