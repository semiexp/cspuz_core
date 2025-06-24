use super::ndarray::NdArray;
use super::traits::{BoolArrayLike, IntArrayLike};

use cspuz_core::csp::BoolExpr as CSPBoolExpr;
use cspuz_core::csp::IntExpr as CSPIntExpr;

pub fn any<T: BoolArrayLike>(values: T) -> NdArray<(), CSPBoolExpr> {
    let terms = values.to_vec().into_iter().map(Box::new).collect();
    NdArray {
        shape: (),
        data: vec![CSPBoolExpr::Or(terms)],
    }
}

pub fn all<T: BoolArrayLike>(values: T) -> NdArray<(), CSPBoolExpr> {
    let terms = values.to_vec().into_iter().map(Box::new).collect();
    NdArray {
        shape: (),
        data: vec![CSPBoolExpr::And(terms)],
    }
}

pub fn sum<T: IntArrayLike>(values: T) -> NdArray<(), CSPIntExpr> {
    let terms = values
        .to_vec()
        .into_iter()
        .map(|x| (Box::new(x), 1))
        .collect();
    NdArray {
        shape: (),
        data: vec![CSPIntExpr::Linear(terms)],
    }
}

pub fn count_true<T: BoolArrayLike>(values: T) -> NdArray<(), CSPIntExpr> {
    let terms = values
        .to_vec()
        .into_iter()
        .map(|x| {
            (
                Box::new(x.ite(CSPIntExpr::Const(1), CSPIntExpr::Const(0))),
                1,
            )
        })
        .collect();
    NdArray {
        shape: (),
        data: vec![CSPIntExpr::Linear(terms)],
    }
}
