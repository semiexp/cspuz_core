use super::ndarray::NdArray;
use super::traits::{BoolArrayLike, IntArrayLike};

use cspuz_core::csp::BoolExpr as CSPBoolExpr;
use cspuz_core::csp::IntExpr as CSPIntExpr;

pub fn any<T: BoolArrayLike>(values: T) -> NdArray<(), CSPBoolExpr> {
    let terms = values.to_vec().into_iter().map(Box::new).collect();
    NdArray::<(), _>::from_raw(CSPBoolExpr::Or(terms))
}

pub fn all<T: BoolArrayLike>(values: T) -> NdArray<(), CSPBoolExpr> {
    let terms = values.to_vec().into_iter().map(Box::new).collect();
    NdArray::<(), _>::from_raw(CSPBoolExpr::And(terms))
}

pub fn sum<T: IntArrayLike>(values: T) -> NdArray<(), CSPIntExpr> {
    let terms = values
        .to_vec()
        .into_iter()
        .map(|x| (Box::new(x), 1))
        .collect();
    NdArray::<(), _>::from_raw(CSPIntExpr::Linear(terms))
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
    NdArray::<(), _>::from_raw(CSPIntExpr::Linear(terms))
}

pub fn consecutive_prefix_true<T: BoolArrayLike>(values: T) -> NdArray<(), CSPIntExpr> {
    let terms = values.to_vec();

    let mut ret = CSPIntExpr::Const(0);
    for t in terms.into_iter().rev() {
        ret = t.ite(ret + CSPIntExpr::Const(1), CSPIntExpr::Const(0));
    }

    NdArray::<(), _>::from_raw(ret)
}

pub fn bool_constant(b: bool) -> NdArray<(), CSPBoolExpr> {
    NdArray::<(), _>::from_raw(CSPBoolExpr::Const(b))
}

pub fn int_constant(n: i32) -> NdArray<(), CSPIntExpr> {
    NdArray::<(), _>::from_raw(CSPIntExpr::Const(n))
}

pub const TRUE: NdArray<(), CSPBoolExpr> = NdArray::<(), _>::from_raw(CSPBoolExpr::Const(true));
pub const FALSE: NdArray<(), CSPBoolExpr> = NdArray::<(), _>::from_raw(CSPBoolExpr::Const(false));
