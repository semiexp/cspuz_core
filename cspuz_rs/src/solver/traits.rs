use super::ndarray::NdArray;

pub use cspuz_core::csp::BoolExpr as CSPBoolExpr;
pub use cspuz_core::csp::IntExpr as CSPIntExpr;

pub trait HasLength {
    fn len(&self) -> usize;
}

impl<T> HasLength for Vec<T> {
    fn len(&self) -> usize {
        Vec::<T>::len(self)
    }
}

#[derive(Clone)]
pub struct Item<T>(pub(super) T);

impl<T> FromIterator<T> for Item<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let data = iter.next().expect("Item must have exactly one element");
        assert!(iter.next().is_none(), "Item must have exactly one element");
        Item(data)
    }
}

impl<T> IntoIterator for Item<T> {
    type Item = T;
    type IntoIter = std::iter::Once<T>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.0)
    }
}

impl<T> HasLength for Item<T> {
    fn len(&self) -> usize {
        1
    }
}

impl<T> std::ops::Index<usize> for Item<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert_eq!(index, 0, "Item can only be indexed at 0");
        &self.0
    }
}

pub trait ArrayShape<T: Clone> {
    type ContainerType: FromIterator<T>
        + IntoIterator<Item = T>
        + Clone
        + HasLength
        + std::ops::Index<usize, Output = T>;
    type Output;

    fn instantiate(&self, data: Vec<T>) -> Self::Output;
}

impl<T: Clone> ArrayShape<T> for () {
    type ContainerType = Item<T>;
    type Output = T;

    fn instantiate(&self, data: Vec<T>) -> Self::Output {
        assert_eq!(data.len(), 1);
        data.into_iter().next().unwrap()
    }
}

impl<T: Clone> ArrayShape<T> for (usize,) {
    type ContainerType = Vec<T>;
    type Output = Vec<T>;

    fn instantiate(&self, data: Vec<T>) -> Self::Output {
        assert_eq!(data.len(), self.0);
        data
    }
}

impl<T: Clone> ArrayShape<T> for (usize, usize) {
    type ContainerType = Vec<T>;
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

impl<T> BroadcastShape<T> for T
where
    T: std::fmt::Debug + PartialEq + Clone,
{
    type Output = T;

    fn broadcast_with(&self, other: &T) -> Self::Output {
        if self == other {
            self.clone()
        } else {
            panic!("Shapes do not match: {:?} vs {:?}", self, other);
        }
    }
}

impl BroadcastShape<(usize,)> for () {
    type Output = (usize,);

    fn broadcast_with(&self, other: &(usize,)) -> Self::Output {
        *other
    }
}

impl BroadcastShape<(usize, usize)> for () {
    type Output = (usize, usize);

    fn broadcast_with(&self, other: &(usize, usize)) -> Self::Output {
        *other
    }
}

impl BroadcastShape<()> for (usize,) {
    type Output = (usize,);

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
    type Shape: ArrayShape<Self::Value>;
    type Value: Clone;

    fn as_ndarray(&self) -> NdArray<Self::Shape, Self::Value>;
}

impl Operand for bool {
    type Shape = ();
    type Value = CSPBoolExpr;

    fn as_ndarray(&self) -> NdArray<Self::Shape, Self::Value> {
        NdArray::<(), _>::from_raw(CSPBoolExpr::Const(*self))
    }
}

impl Operand for i32 {
    type Shape = ();
    type Value = CSPIntExpr;

    fn as_ndarray(&self) -> NdArray<Self::Shape, Self::Value> {
        NdArray::<(), _>::from_raw(CSPIntExpr::Const(*self))
    }
}

fn propagated_len(a: usize, b: usize) -> usize {
    if a == 1 {
        b
    } else {
        a
    }
}

pub trait PropagateBinary<X, Y, T> {
    type Output;

    fn propagate_binary<F>(&self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T;
}

impl<A, B, X, Y, T> PropagateBinary<X, Y, T> for (A, B)
where
    A: Operand<Value = X>,
    B: Operand<Value = Y>,
    A::Shape: BroadcastShape<B::Shape>,
    X: Clone,
    Y: Clone,
    <A::Shape as BroadcastShape<B::Shape>>::Output: ArrayShape<T>,
    T: Clone,
{
    type Output = NdArray<<A::Shape as BroadcastShape<B::Shape>>::Output, T>;

    fn propagate_binary<F>(&self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        let (lhs, rhs) = self;
        let lhs = lhs.as_ndarray();
        let rhs = rhs.as_ndarray();

        let out_shape = lhs.shape.broadcast_with(&rhs.shape);
        let r_len = propagated_len(lhs.data.len(), rhs.data.len());

        assert!(lhs.data.len() == rhs.data.len() || lhs.data.len() == 1 || rhs.data.len() == 1);

        NdArray {
            shape: out_shape,
            data: (0..r_len)
                .map(|i| {
                    let lhs_value = &lhs.data[if lhs.data.len() == 1 { 0 } else { i }];
                    let rhs_value = &rhs.data[if rhs.data.len() == 1 { 0 } else { i }];
                    func(lhs_value.clone(), rhs_value.clone())
                })
                .collect(),
        }
    }
}

pub trait PropagateTernary<X, Y, Z, T> {
    type Output;

    fn propagate_ternary<F>(&self, func: F) -> Self::Output
    where
        F: Fn(X, Y, Z) -> T;
}

impl<A, B, C, X, Y, Z, T> PropagateTernary<X, Y, Z, T> for (A, B, C)
where
    A: Operand<Value = X>,
    B: Operand<Value = Y>,
    C: Operand<Value = Z>,
    A::Shape: BroadcastShape<B::Shape>,
    <A::Shape as BroadcastShape<B::Shape>>::Output: BroadcastShape<C::Shape>,
    X: Clone,
    Y: Clone,
    Z: Clone,
    <A::Shape as BroadcastShape<B::Shape>>::Output: ArrayShape<T>,
    T: Clone,
    <<A::Shape as BroadcastShape<B::Shape>>::Output as BroadcastShape<C::Shape>>::Output:
        ArrayShape<T>,
{
    type Output = NdArray<
        <<A::Shape as BroadcastShape<B::Shape>>::Output as BroadcastShape<C::Shape>>::Output,
        T,
    >;

    fn propagate_ternary<F>(&self, func: F) -> Self::Output
    where
        F: Fn(X, Y, Z) -> T,
    {
        let (a, b, c) = self;
        let a = a.as_ndarray();
        let b = b.as_ndarray();
        let c = c.as_ndarray();

        let out_shape = a.shape.broadcast_with(&b.shape).broadcast_with(&c.shape);
        let r_len = propagated_len(propagated_len(a.data.len(), b.data.len()), c.data.len());

        assert!(a.data.len() == r_len || a.data.len() == 1);
        assert!(b.data.len() == r_len || b.data.len() == 1);
        assert!(c.data.len() == r_len || c.data.len() == 1);

        NdArray {
            shape: out_shape,
            data: (0..r_len)
                .map(|i| {
                    let a_value = &a.data[if a.data.len() == 1 { 0 } else { i }];
                    let b_value = &b.data[if b.data.len() == 1 { 0 } else { i }];
                    let c_value = &c.data[if c.data.len() == 1 { 0 } else { i }];
                    func(a_value.clone(), b_value.clone(), c_value.clone())
                })
                .collect(),
        }
    }
}

pub trait BoolArrayLike {
    fn to_vec(self) -> Vec<CSPBoolExpr>;
}

impl<T> BoolArrayLike for T
where
    T: IntoIterator,
    T::Item: Operand<Shape = (), Value = CSPBoolExpr>,
{
    fn to_vec(self) -> Vec<CSPBoolExpr> {
        self.into_iter().map(|x| x.as_ndarray().data.0).collect()
    }
}

pub trait IntArrayLike {
    fn to_vec(self) -> Vec<CSPIntExpr>;
}

impl<T> IntArrayLike for T
where
    T: IntoIterator,
    T::Item: Operand<Shape = (), Value = CSPIntExpr>,
{
    fn to_vec(self) -> Vec<CSPIntExpr> {
        self.into_iter().map(|x| x.as_ndarray().data.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_shape_unit() {
        let shape = ();
        let data = vec![42];
        let result = shape.instantiate(data);
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_array_shape_unit_wrong_size() {
        let shape = ();
        let data = vec![42, 43];
        shape.instantiate(data);
    }

    #[test]
    fn test_array_shape_1d() {
        let shape = (3,);
        let data = vec![1, 2, 3];
        let result = shape.instantiate(data);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_array_shape_1d_wrong_size() {
        let shape = (3,);
        let data = vec![1, 2];
        shape.instantiate(data);
    }

    #[test]
    fn test_array_shape_2d() {
        let shape = (2, 3);
        let data = vec![1, 2, 3, 4, 5, 6];
        let result = shape.instantiate(data);
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_array_shape_2d_single_row() {
        let shape = (1, 3);
        let data = vec![1, 2, 3];
        let result = shape.instantiate(data);
        assert_eq!(result, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_array_shape_2d_single_column() {
        let shape = (3, 1);
        let data = vec![1, 2, 3];
        let result = shape.instantiate(data);
        assert_eq!(result, vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    #[should_panic]
    fn test_array_shape_2d_wrong_size() {
        let shape = (2, 3);
        let data = vec![1, 2, 3, 4, 5];
        shape.instantiate(data);
    }

    #[test]
    fn test_broadcast_shape_same_type() {
        let shape1 = (5,);
        let shape2 = (5,);
        let result = shape1.broadcast_with(&shape2);
        assert_eq!(result, (5,));
    }

    #[test]
    #[should_panic(expected = "Shapes do not match")]
    fn test_broadcast_shape_different_same_type() {
        let shape1 = (5,);
        let shape2 = (3,);
        shape1.broadcast_with(&shape2);
    }

    #[test]
    fn test_broadcast_shape_unit_with_1d() {
        let shape1 = ();
        let shape2 = (5,);
        let result = shape1.broadcast_with(&shape2);
        assert_eq!(result, (5,));
    }

    #[test]
    fn test_broadcast_shape_unit_with_1d_zero() {
        let shape1 = ();
        let shape2 = (0,);
        let result = shape1.broadcast_with(&shape2);
        assert_eq!(result, (0,));
    }

    #[test]
    fn test_broadcast_shape_1d_with_unit() {
        let shape1 = (5,);
        let shape2 = ();
        let result = shape1.broadcast_with(&shape2);
        assert_eq!(result, (5,));
    }

    #[test]
    fn test_broadcast_shape_1d_zero_with_unit() {
        let shape1 = (0,);
        let shape2 = ();
        let result = shape1.broadcast_with(&shape2);
        assert_eq!(result, (0,));
    }

    #[test]
    fn test_broadcast_shape_unit_with_2d() {
        let shape1 = ();
        let shape2 = (3, 4);
        let result = shape1.broadcast_with(&shape2);
        assert_eq!(result, (3, 4));
    }

    #[test]
    fn test_broadcast_shape_2d_with_unit() {
        let shape1 = (3, 4);
        let shape2 = ();
        let result = shape1.broadcast_with(&shape2);
        assert_eq!(result, (3, 4));
    }

    #[test]
    fn test_broadcast_shape_2d_same() {
        let shape1 = (3, 4);
        let shape2 = (3, 4);
        let result = shape1.broadcast_with(&shape2);
        assert_eq!(result, (3, 4));
    }

    #[test]
    #[should_panic(expected = "Shapes do not match")]
    fn test_broadcast_shape_2d_different() {
        let shape1 = (3, 4);
        let shape2 = (2, 4);
        shape1.broadcast_with(&shape2);
    }

    #[test]
    fn test_propagated_len() {
        assert_eq!(propagated_len(1, 5), 5);
        assert_eq!(propagated_len(5, 1), 5);
        assert_eq!(propagated_len(3, 3), 3);
        assert_eq!(propagated_len(7, 2), 7);
        assert_eq!(propagated_len(1, 0), 0);
        assert_eq!(propagated_len(0, 1), 0);
    }
}
