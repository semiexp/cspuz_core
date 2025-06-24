use super::ndarray::NdArray;

pub use cspuz_core::csp::BoolExpr as CSPBoolExpr;
pub use cspuz_core::csp::IntExpr as CSPIntExpr;

pub trait ArrayShape<T> {
    type Output;

    fn instantiate(&self, data: Vec<T>) -> Self::Output;
}

impl<T> ArrayShape<T> for () {
    type Output = T;

    fn instantiate(&self, data: Vec<T>) -> Self::Output {
        assert_eq!(data.len(), 1);
        data.into_iter().next().unwrap()
    }
}

impl<T> ArrayShape<T> for (usize, ) {
    type Output = Vec<T>;

    fn instantiate(&self, data: Vec<T>) -> Self::Output {
        assert_eq!(data.len(), self.0);
        data
    }
}

impl<T> ArrayShape<T> for (usize, usize) {
    type Output = Vec<Vec<T>>;

    fn instantiate(&self, data: Vec<T>) -> Self::Output {
        assert_eq!(data.len(), self.0 * self.1);
        let mut out = vec![];
        for _ in 0..self.0 {
            out.push(vec![]);
        }

        for (i, value) in data.into_iter().enumerate() {
            out[i / self.1].push(value);
        }
        out
    }
}

pub trait BroadcastShape<T> {
    type Output;

    fn broadcast_with(&self, other: &T) -> Self::Output;
}

impl<T> BroadcastShape<T> for T where T: std::fmt::Debug + PartialEq + Clone {
    type Output = T;

    fn broadcast_with(&self, other: &T) -> Self::Output {
        if self == other {
            self.clone()
        } else {
            panic!("Shapes do not match: {:?} vs {:?}", self, other);
        }
    }
}

impl BroadcastShape<(usize, )> for () {
    type Output = (usize, );

    fn broadcast_with(&self, other: &(usize, )) -> Self::Output {
        *other
    }
}

impl BroadcastShape<(usize, usize)> for () {
    type Output = (usize, usize);

    fn broadcast_with(&self, other: &(usize, usize)) -> Self::Output {
        *other
    }
}

impl BroadcastShape<()> for (usize, ) {
    type Output = (usize, );

    fn broadcast_with(&self, _other: &()) -> Self::Output {
        *self
    }
}

impl BroadcastShape<()> for (usize, usize) {
    type Output = (usize, usize);

    fn broadcast_with(&self, _other: &()) -> Self::Output {
        *self
    }
}

pub trait Operand {
    type Shape;
    type Value;

    fn as_ndarray(&self) -> NdArray<Self::Shape, Self::Value>;
}

pub trait PropagateBinary<X, Y, T> {
    type Output;

    fn propagate_binary<F>(&self, func: F) -> Self::Output where
        F: Fn(X, Y) -> T;
}

impl<A, B, X, Y, T> PropagateBinary<X, Y, T> for (A, B) where 
    A: Operand<Value = X>,
    B: Operand<Value = Y>,
    A::Shape: BroadcastShape<B::Shape>,
    X: Clone,
    Y: Clone,
{
    type Output = NdArray<<A::Shape as BroadcastShape<B::Shape>>::Output, T>;

    fn propagate_binary<F>(&self, func: F) -> Self::Output where
        F: Fn(X, Y) -> T,
    {
        let (lhs, rhs) = self;
        let lhs = lhs.as_ndarray();
        let rhs = rhs.as_ndarray();

        let out_shape = lhs.shape.broadcast_with(&rhs.shape);

        assert!(lhs.data.len() == rhs.data.len() || lhs.data.len() == 1 || rhs.data.len() == 1);

        let mut data = vec![];
        for i in 0..(lhs.data.len().max(rhs.data.len())) {
            let lhs_value = &lhs.data[if lhs.data.len() == 1 { 0 } else { i }];
            let rhs_value = &rhs.data[if rhs.data.len() == 1 { 0 } else { i }];
            data.push(func(lhs_value.clone(), rhs_value.clone()));
        }

        NdArray { shape: out_shape, data }
    }
}

pub trait PropagateTernary<X, Y, Z, T> {
    type Output;

    fn propagate_ternary<F>(&self, func: F) -> Self::Output where
        F: Fn(X, Y, Z) -> T;
}

impl<A, B, C, X, Y, Z, T> PropagateTernary<X, Y, Z, T> for (A, B, C) where
    A: Operand<Value = X>,
    B: Operand<Value = Y>,
    C: Operand<Value = Z>,
    A::Shape: BroadcastShape<B::Shape>,
    <A::Shape as BroadcastShape<B::Shape>>::Output: BroadcastShape<C::Shape>,
    X: Clone,
    Y: Clone,
    Z: Clone,
{
    type Output = NdArray<
        <<A::Shape as BroadcastShape<B::Shape>>::Output as BroadcastShape<C::Shape>>::Output,
        T>;

    fn propagate_ternary<F>(&self, func: F) -> Self::Output where
        F: Fn(X, Y, Z) -> T,
    {
        let (a, b, c) = self;
        let a = a.as_ndarray();
        let b = b.as_ndarray();
        let c = c.as_ndarray();

        let out_shape = a.shape.broadcast_with(&b.shape).broadcast_with(&c.shape);
        let r_len = a.data.len().max(b.data.len()).max(c.data.len());

        assert!(a.data.len() == r_len || a.data.len() == 1);
        assert!(b.data.len() == r_len || b.data.len() == 1);
        assert!(c.data.len() == r_len || c.data.len() == 1);

        let mut data = vec![];
        for i in 0..r_len {
            let a_value = &a.data[if a.data.len() == 1 { 0 } else { i }];
            let b_value = &b.data[if b.data.len() == 1 { 0 } else { i }];
            let c_value = &c.data[if c.data.len() == 1 { 0 } else { i }];
            data.push(func(a_value.clone(), b_value.clone(), c_value.clone()));
        }

        NdArray { shape: out_shape, data }
    }
}

pub trait BoolArrayLike {
    fn to_vec(self) -> Vec<CSPBoolExpr>;
}

impl<T> BoolArrayLike for T
where 
    T: IntoIterator,
    T::Item: Operand<Value = CSPBoolExpr>,
{
    fn to_vec(self) -> Vec<CSPBoolExpr> {
        self.into_iter().map(|x| x.as_ndarray().data.remove(0)).collect()
    }
}

pub trait IntArrayLike {
    fn to_vec(self) -> Vec<CSPIntExpr>;
}

impl<T> IntArrayLike for T
where 
    T: IntoIterator,
    T::Item: Operand<Value = CSPIntExpr>,
{
    fn to_vec(self) -> Vec<CSPIntExpr> {
        self.into_iter().map(|x| x.as_ndarray().data.remove(0)).collect()
    }
}
