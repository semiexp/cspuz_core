use std::ops::{Add, BitAnd, BitOr, BitXor, Bound, Not, RangeBounds, Sub};

use crate::items::Arrow;
pub use cspuz_core::config::{Config, GraphDivisionMode};
pub use cspuz_core::csp::BoolExpr as CSPBoolExpr;
pub use cspuz_core::csp::BoolVar as CSPBoolVar;
pub use cspuz_core::csp::IntExpr as CSPIntExpr;
pub use cspuz_core::csp::IntVar as CSPIntVar;
use cspuz_core::csp::{Assignment, Stmt};
use cspuz_core::custom_constraints::PropagatorGenerator;
use cspuz_core::domain::Domain;
use cspuz_core::integration::IntegratedSolver;
use cspuz_core::integration::Model as IntegratedModel;
pub use cspuz_core::integration::PerfStats;
pub use cspuz_core::propagators::graph_division::GraphDivisionOptions;

#[derive(Clone)]
pub struct Value<T>(pub(crate) T);

#[derive(Clone)]
pub struct Array0DImpl<T> {
    data: T,
}

#[derive(Clone)]
pub struct Array1DImpl<T> {
    data: Vec<T>,
}

#[derive(Clone)]
pub struct Array2DImpl<T> {
    shape: (usize, usize),
    data: Vec<T>,
}

// ==========
// IntoIter
// ==========

impl<T> IntoIterator for Array0DImpl<T> {
    type Item = T;
    type IntoIter = std::iter::Once<T>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.data)
    }
}

impl<T> IntoIterator for Array1DImpl<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> IntoIterator for Array2DImpl<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> IntoIterator for Value<T>
where
    T: IntoIterator,
{
    type Item = Value<Array0DImpl<T::Item>>;
    type IntoIter = std::iter::Map<T::IntoIter, fn(<T as IntoIterator>::Item) -> Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|x| Value(Array0DImpl { data: x }))
    }
}

impl<T> IntoIterator for &Value<T>
where
    Value<T>: Clone + IntoIterator,
{
    type Item = <Value<T> as IntoIterator>::Item;
    type IntoIter = <Value<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.clone().into_iter()
    }
}

// ==========
// Builders
// ==========

impl<T> Value<Array1DImpl<T>> {
    pub fn new<I>(data: I) -> Value<Array1DImpl<T>>
    where
        I: IntoIterator<Item = Value<Array0DImpl<T>>>,
    {
        Value(Array1DImpl {
            data: data.into_iter().map(|x| x.0.data).collect(),
        })
    }
}

impl<T> Value<Array2DImpl<T>> {
    pub fn new<I>(shape: (usize, usize), data: I) -> Value<Array2DImpl<T>>
    where
        I: IntoIterator<Item = Value<Array0DImpl<T>>>,
    {
        let (height, width) = shape;
        let data: Vec<T> = data.into_iter().map(|x| x.0.data).collect();
        assert_eq!(height * width, data.len());
        Value(Array2DImpl { shape, data })
    }
}

// ==========
// Accessors
// ==========

impl<T: Clone> Value<Array1DImpl<T>> {
    pub fn len(&self) -> usize {
        self.0.data.len()
    }

    pub fn at(&self, idx: usize) -> Value<Array0DImpl<T>> {
        Value(Array0DImpl {
            data: self.0.data[idx].clone(),
        })
    }

    pub fn reverse(&self) -> Value<Array1DImpl<T>> {
        let mut data = self.0.data.clone();
        data.reverse();
        Value(Array1DImpl { data })
    }
}

fn resolve_range<T: RangeBounds<usize>>(len: usize, range: &T) -> (usize, usize) {
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => len,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };
    if start >= end {
        (0, 0)
    } else {
        (start, end)
    }
}

impl<T: Clone> Value<Array1DImpl<T>> {
    pub fn reshape_as_2d(&self, shape: (usize, usize)) -> Value<Array2DImpl<T>> {
        let (height, width) = shape;
        assert_eq!(height * width, self.0.data.len());
        Value(Array2DImpl {
            shape,
            data: self.0.data.clone(),
        })
    }

    pub fn slice<I: RangeBounds<usize>>(&self, idx: I) -> Value<Array1DImpl<T>> {
        let (start, end) = resolve_range(self.len(), &idx);
        Value(Array1DImpl {
            data: self.0.data[start..end].to_vec(),
        })
    }
}

impl<T> Value<Array2DImpl<T>> {
    pub fn shape(&self) -> (usize, usize) {
        self.0.shape
    }

    pub fn four_neighbor_indices(&self, idx: (usize, usize)) -> Vec<(usize, usize)> {
        let (h, w) = self.shape();
        let (y, x) = idx;
        let mut ret = vec![];
        if y > 0 {
            ret.push((y - 1, x));
        }
        if x > 0 {
            ret.push((y, x - 1));
        }
        if y < h - 1 {
            ret.push((y + 1, x));
        }
        if x < w - 1 {
            ret.push((y, x + 1));
        }
        ret
    }
}

impl<T: Clone> Value<Array2DImpl<T>> {
    pub fn at(&self, idx: (usize, usize)) -> Value<Array0DImpl<T>> {
        let (y, x) = idx;
        let (h, w) = self.0.shape;
        assert!(y < h && x < w);
        Value(Array0DImpl {
            data: self.0.data[y * w + x].clone(),
        })
    }

    pub fn at_offset<D, E>(
        &self,
        idx: (usize, usize),
        offset: (i32, i32),
        default: D,
    ) -> Value<Array0DImpl<E>>
    where
        D: Operand<Output = Array0DImpl<E>>,
        Value<Array0DImpl<T>>: Operand<Output = Array0DImpl<E>>,
    {
        let (y, x) = idx;
        let (dy, dx) = offset;
        let y = y as i32 + dy;
        let x = x as i32 + dx;
        if 0 <= y && y < self.shape().0 as i32 && 0 <= x && x < self.shape().1 as i32 {
            Value(self.at((y as usize, x as usize)).as_expr_array())
        } else {
            Value(default.as_expr_array())
        }
    }

    pub fn select<I, X>(&self, idx: I) -> Value<Array1DImpl<T>>
    where
        I: IntoIterator<Item = X>,
        X: std::borrow::Borrow<(usize, usize)>,
    {
        let mut data = vec![];
        let (h, w) = self.0.shape;
        for p in idx {
            let &(y, x) = p.borrow();
            assert!(y < h && x < w);
            data.push(self.0.data[y * w + x].clone());
        }
        Value(Array1DImpl { data })
    }

    pub fn slice_fixed_y<X: RangeBounds<usize>>(&self, idx: (usize, X)) -> Value<Array1DImpl<T>> {
        let (y, xs) = idx;
        let (_, w) = self.0.shape;
        let (x_start, x_end) = resolve_range(w, &xs);

        let items = (x_start..x_end).map(|x| self.at((y, x)).0.data).collect();
        Value(Array1DImpl { data: items })
    }

    pub fn slice_fixed_x<Y: RangeBounds<usize>>(&self, idx: (Y, usize)) -> Value<Array1DImpl<T>> {
        let (ys, x) = idx;
        let (h, _) = self.0.shape;
        let (y_start, y_end) = resolve_range(h, &ys);

        let items = (y_start..y_end).map(|y| self.at((y, x)).0.data).collect();
        Value(Array1DImpl { data: items })
    }

    pub fn slice<Y: RangeBounds<usize>, X: RangeBounds<usize>>(
        &self,
        idx: (Y, X),
    ) -> Value<Array2DImpl<T>> {
        let (ys, xs) = idx;
        let (h, w) = self.0.shape;
        let (y_start, y_end) = resolve_range(h, &ys); // [y_start, y_end)
        let (x_start, x_end) = resolve_range(w, &xs); // [x_start, x_end)

        let slice_shape = (y_end - y_start, x_end - x_start);
        let mut items = vec![];
        for y in y_start..y_end {
            for x in x_start..x_end {
                items.push(self.0.data[y * w + x].clone());
            }
        }
        Value(Array2DImpl {
            shape: slice_shape,
            data: items,
        })
    }

    pub fn flatten(&self) -> Value<Array1DImpl<T>> {
        Value(Array1DImpl {
            data: self.0.data.clone(),
        })
    }

    pub fn reshape(&self, shape: (usize, usize)) -> Value<Array2DImpl<T>> {
        let (height, width) = shape;
        assert_eq!(height * width, self.0.data.len());
        Value(Array2DImpl {
            shape,
            data: self.0.data.clone(),
        })
    }

    pub fn four_neighbors(&self, idx: (usize, usize)) -> Value<Array1DImpl<T>> {
        self.select(self.four_neighbor_indices(idx))
    }

    pub fn pointing_cells(
        &self,
        cell: (usize, usize),
        arrow: Arrow,
    ) -> Option<Value<Array1DImpl<T>>> {
        let (y, x) = cell;
        match arrow {
            Arrow::Unspecified => None,
            Arrow::Up => Some(self.slice_fixed_x((..y, x))),
            Arrow::Down => Some(self.slice_fixed_x(((y + 1).., x))),
            Arrow::Left => Some(self.slice_fixed_y((y, ..x))),
            Arrow::Right => Some(self.slice_fixed_y((y, (x + 1)..))),
        }
    }
}

// ==========
// Operators for Value<T>
// ==========

pub trait Operand {
    type Output;

    fn as_expr_array(self) -> Self::Output;
    fn as_expr_array_value(self) -> Value<Self::Output>
    where
        Self: Sized,
    {
        Value(self.as_expr_array())
    }
}

impl Operand for bool {
    type Output = Array0DImpl<CSPBoolExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array0DImpl {
            data: CSPBoolExpr::Const(self),
        }
    }
}

impl Operand for &bool {
    type Output = Array0DImpl<CSPBoolExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array0DImpl {
            data: CSPBoolExpr::Const(*self),
        }
    }
}

impl Operand for i32 {
    type Output = Array0DImpl<CSPIntExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array0DImpl {
            data: CSPIntExpr::Const(self),
        }
    }
}

impl Operand for &i32 {
    type Output = Array0DImpl<CSPIntExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array0DImpl {
            data: CSPIntExpr::Const(*self),
        }
    }
}

macro_rules! operand_as_is {
    ($value_type:ty) => {
        impl Operand for Value<$value_type> {
            type Output = $value_type;

            fn as_expr_array(self) -> Self::Output {
                self.0
            }
        }
    };
}

operand_as_is!(Array0DImpl<CSPBoolExpr>);
operand_as_is!(Array1DImpl<CSPBoolExpr>);
operand_as_is!(Array2DImpl<CSPBoolExpr>);
operand_as_is!(Array0DImpl<CSPIntExpr>);
operand_as_is!(Array1DImpl<CSPIntExpr>);
operand_as_is!(Array2DImpl<CSPIntExpr>);

impl Operand for Value<Array0DImpl<CSPBoolVar>> {
    type Output = Array0DImpl<CSPBoolExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array0DImpl {
            data: CSPBoolExpr::Var(self.0.data),
        }
    }
}

impl Operand for Value<Array1DImpl<CSPBoolVar>> {
    type Output = Array1DImpl<CSPBoolExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array1DImpl {
            data: self.0.data.into_iter().map(CSPBoolExpr::Var).collect(),
        }
    }
}

impl Operand for Value<Array2DImpl<CSPBoolVar>> {
    type Output = Array2DImpl<CSPBoolExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array2DImpl {
            data: self.0.data.into_iter().map(CSPBoolExpr::Var).collect(),
            shape: self.0.shape,
        }
    }
}

impl Operand for Value<Array0DImpl<CSPIntVar>> {
    type Output = Array0DImpl<CSPIntExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array0DImpl {
            data: CSPIntExpr::Var(self.0.data),
        }
    }
}

impl Operand for Value<Array1DImpl<CSPIntVar>> {
    type Output = Array1DImpl<CSPIntExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array1DImpl {
            data: self.0.data.into_iter().map(CSPIntExpr::Var).collect(),
        }
    }
}

impl Operand for Value<Array2DImpl<CSPIntVar>> {
    type Output = Array2DImpl<CSPIntExpr>;

    fn as_expr_array(self) -> Self::Output {
        Array2DImpl {
            data: self.0.data.into_iter().map(CSPIntExpr::Var).collect(),
            shape: self.0.shape,
        }
    }
}

impl<T> Operand for &Value<T>
where
    T: Clone,
    Value<T>: Operand,
{
    type Output = <Value<T> as Operand>::Output;

    fn as_expr_array(self) -> Self::Output {
        self.clone().as_expr_array()
    }
}

impl<T> Operand for &&Value<T>
where
    T: Clone,
    Value<T>: Operand,
{
    type Output = <Value<T> as Operand>::Output;

    fn as_expr_array(self) -> Self::Output {
        (*self).as_expr_array()
    }
}

pub trait PropagateBinary<X, Y, T> {
    type Output;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T;
}

impl<X, Y, T> PropagateBinary<X, Y, T> for (Array0DImpl<X>, Array0DImpl<Y>) {
    type Output = Array0DImpl<T>;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        Array0DImpl {
            data: func(self.0.data, self.1.data),
        }
    }
}

impl<X, Y: Clone, T> PropagateBinary<X, Y, T> for (Array1DImpl<X>, Array0DImpl<Y>) {
    type Output = Array1DImpl<T>;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        let rhs = self.1.data;
        Array1DImpl {
            data: self
                .0
                .data
                .into_iter()
                .map(|lhs| func(lhs, rhs.clone()))
                .collect(),
        }
    }
}

impl<X: Clone, Y, T> PropagateBinary<X, Y, T> for (Array0DImpl<X>, Array1DImpl<Y>) {
    type Output = Array1DImpl<T>;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        let lhs = self.0.data;
        Array1DImpl {
            data: self
                .1
                .data
                .into_iter()
                .map(|rhs| func(lhs.clone(), rhs))
                .collect(),
        }
    }
}

impl<X, Y, T> PropagateBinary<X, Y, T> for (Array1DImpl<X>, Array1DImpl<Y>) {
    type Output = Array1DImpl<T>;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        Array1DImpl {
            data: self
                .0
                .data
                .into_iter()
                .zip(self.1.data.into_iter())
                .map(|(lhs, rhs)| func(lhs, rhs))
                .collect(),
        }
    }
}

impl<X, Y: Clone, T> PropagateBinary<X, Y, T> for (Array2DImpl<X>, Array0DImpl<Y>) {
    type Output = Array2DImpl<T>;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        let rhs = self.1.data;
        Array2DImpl {
            shape: self.0.shape,
            data: self
                .0
                .data
                .into_iter()
                .map(|lhs| func(lhs, rhs.clone()))
                .collect(),
        }
    }
}

impl<X: Clone, Y, T> PropagateBinary<X, Y, T> for (Array0DImpl<X>, Array2DImpl<Y>) {
    type Output = Array2DImpl<T>;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        let lhs = self.0.data;
        Array2DImpl {
            shape: self.1.shape,
            data: self
                .1
                .data
                .into_iter()
                .map(|rhs| func(lhs.clone(), rhs))
                .collect(),
        }
    }
}

impl<X, Y, T> PropagateBinary<X, Y, T> for (Array2DImpl<X>, Array2DImpl<Y>) {
    type Output = Array2DImpl<T>;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        assert_eq!(self.0.shape, self.1.shape);
        Array2DImpl {
            shape: self.0.shape,
            data: self
                .0
                .data
                .into_iter()
                .zip(self.1.data.into_iter())
                .map(|(lhs, rhs)| func(lhs, rhs))
                .collect(),
        }
    }
}

pub trait PropagateBinaryGeneric<X, Y, T> {
    type Output;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T;
}

impl<A, B, X, Y, T> PropagateBinaryGeneric<X, Y, T> for (A, B)
where
    A: Operand,
    B: Operand,
    (<A as Operand>::Output, <B as Operand>::Output): PropagateBinary<X, Y, T>,
{
    type Output =
        <(<A as Operand>::Output, <B as Operand>::Output) as PropagateBinary<X, Y, T>>::Output;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y) -> T,
    {
        let (a, b) = self;
        (a.as_expr_array(), b.as_expr_array()).generate(func)
    }
}

pub trait PropagateTernary<X, Y, Z, T> {
    type Output;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y, Z) -> T;
}

impl<A, B, C, X, Y, Z, T> PropagateTernary<X, Y, Z, T> for (A, B, C)
where
    (A, B): PropagateBinary<X, Y, (X, Y)>,
    (<(A, B) as PropagateBinary<X, Y, (X, Y)>>::Output, C): PropagateBinary<(X, Y), Z, T>,
{
    type Output = <(<(A, B) as PropagateBinary<X, Y, (X, Y)>>::Output, C) as PropagateBinary<
        (X, Y),
        Z,
        T,
    >>::Output;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y, Z) -> T,
    {
        let (a, b, c) = self;
        let ab = (a, b).generate(|x, y| (x, y));
        (ab, c).generate(|(x, y), z| func(x, y, z))
    }
}

pub trait PropagateTernaryGeneric<X, Y, Z, T> {
    type Output;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y, Z) -> T;
}

impl<A, B, C, X, Y, Z, T> PropagateTernaryGeneric<X, Y, Z, T> for (A, B, C)
where
    A: Operand,
    B: Operand,
    C: Operand,
    (
        <A as Operand>::Output,
        <B as Operand>::Output,
        <C as Operand>::Output,
    ): PropagateTernary<X, Y, Z, T>,
{
    type Output = <(
        <A as Operand>::Output,
        <B as Operand>::Output,
        <C as Operand>::Output,
    ) as PropagateTernary<X, Y, Z, T>>::Output;

    fn generate<F>(self, func: F) -> Self::Output
    where
        F: Fn(X, Y, Z) -> T,
    {
        let (a, b, c) = self;
        (a.as_expr_array(), b.as_expr_array(), c.as_expr_array()).generate(func)
    }
}

macro_rules! binary_op {
    ($trait_name:ident, $trait_func:ident, $input_type:ty, $output_type:ty, $gen:expr) => {
        impl<X, Y> $trait_name<Y> for Value<X>
        where
            (Value<X>, Y): PropagateBinaryGeneric<$input_type, $input_type, $output_type>,
        {
            type Output =
                Value<
                    <(Value<X>, Y) as PropagateBinaryGeneric<
                        $input_type,
                        $input_type,
                        $output_type,
                    >>::Output,
                >;

            fn $trait_func(self, rhs: Y) -> Self::Output {
                Value((self, rhs).generate($gen))
            }
        }

        impl<'a, X, Y> $trait_name<Y> for &'a Value<X>
        where
            (&'a Value<X>, Y): PropagateBinaryGeneric<$input_type, $input_type, $output_type>,
        {
            type Output = Value<
                <(&'a Value<X>, Y) as PropagateBinaryGeneric<
                    $input_type,
                    $input_type,
                    $output_type,
                >>::Output,
            >;

            fn $trait_func(self, rhs: Y) -> Self::Output {
                Value((self, rhs).generate($gen))
            }
        }
    };
}

binary_op!(BitAnd, bitand, CSPBoolExpr, CSPBoolExpr, |x, y| x & y);
binary_op!(BitOr, bitor, CSPBoolExpr, CSPBoolExpr, |x, y| x | y);
binary_op!(BitXor, bitxor, CSPBoolExpr, CSPBoolExpr, |x, y| x ^ y);
binary_op!(Add, add, CSPIntExpr, CSPIntExpr, |x, y| x + y);
binary_op!(Sub, sub, CSPIntExpr, CSPIntExpr, |x, y| x - y);

impl<X> Not for Value<X>
where
    (Value<X>, bool): PropagateBinaryGeneric<CSPBoolExpr, CSPBoolExpr, CSPBoolExpr>,
{
    type Output = Value<
        <(Value<X>, bool) as PropagateBinaryGeneric<CSPBoolExpr, CSPBoolExpr, CSPBoolExpr>>::Output,
    >;

    fn not(self) -> Self::Output {
        Value((self, false).generate(|x, _| !x))
    }
}

impl<'a, X> Not for &'a Value<X>
where
    (&'a Value<X>, bool): PropagateBinaryGeneric<CSPBoolExpr, CSPBoolExpr, CSPBoolExpr>,
{
    type Output =
        Value<
            <(&'a Value<X>, bool) as PropagateBinaryGeneric<
                CSPBoolExpr,
                CSPBoolExpr,
                CSPBoolExpr,
            >>::Output,
        >;

    fn not(self) -> Self::Output {
        Value((self, false).generate(|x, _| !x))
    }
}

macro_rules! comparator {
    ($func_name:ident) => {
        impl<X> Value<X> {
            pub fn $func_name<'a, Y>(&'a self, rhs: Y) -> Value<<(&'a Self, Y) as PropagateBinaryGeneric<CSPIntExpr, CSPIntExpr, CSPBoolExpr>>::Output> where
            (&'a Self, Y): PropagateBinaryGeneric<CSPIntExpr, CSPIntExpr, CSPBoolExpr>
            {
                Value((self, rhs).generate(|x, y| x.$func_name(y)))
            }
        }
    }
}

comparator!(eq);
comparator!(ne);
comparator!(ge);
comparator!(gt);
comparator!(le);
comparator!(lt);

impl<X> Value<X> {
    pub fn imp<'a, Y>(
        &'a self,
        rhs: Y,
    ) -> Value<
        <(&'a Self, Y) as PropagateBinaryGeneric<CSPBoolExpr, CSPBoolExpr, CSPBoolExpr>>::Output,
    >
    where
        (&'a Self, Y): PropagateBinaryGeneric<CSPBoolExpr, CSPBoolExpr, CSPBoolExpr>,
    {
        Value((self, rhs).generate(|x, y| x.imp(y)))
    }

    pub fn iff<'a, Y>(
        &'a self,
        rhs: Y,
    ) -> Value<
        <(&'a Self, Y) as PropagateBinaryGeneric<CSPBoolExpr, CSPBoolExpr, CSPBoolExpr>>::Output,
    >
    where
        (&'a Self, Y): PropagateBinaryGeneric<CSPBoolExpr, CSPBoolExpr, CSPBoolExpr>,
    {
        Value((self, rhs).generate(|x, y| x.iff(y)))
    }

    pub fn ite<'a, Y, Z>(
        &'a self,
        if_true: Y,
        if_false: Z,
    ) -> Value<
        <(&'a Self, Y, Z) as PropagateTernaryGeneric<
            CSPBoolExpr,
            CSPIntExpr,
            CSPIntExpr,
            CSPIntExpr,
        >>::Output,
    >
    where
        (&'a Self, Y, Z): PropagateTernaryGeneric<CSPBoolExpr, CSPIntExpr, CSPIntExpr, CSPIntExpr>,
    {
        Value((self, if_true, if_false).generate(|x, y, z| x.ite(y, z)))
    }
}

pub fn count_true<T>(values: T) -> Value<Array0DImpl<CSPIntExpr>>
where
    T: IntoIterator,
    T::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
{
    let terms = values
        .into_iter()
        .map(|x| {
            (
                Box::new(
                    x.as_expr_array()
                        .data
                        .ite(CSPIntExpr::Const(1), CSPIntExpr::Const(0)),
                ),
                1,
            )
        })
        .collect();
    Value(Array0DImpl {
        data: CSPIntExpr::Linear(terms),
    })
}

pub fn any<T>(values: T) -> Value<Array0DImpl<CSPBoolExpr>>
where
    T: IntoIterator,
    T::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
{
    let terms = values
        .into_iter()
        .map(|x| Box::new(x.as_expr_array().data))
        .collect();
    Value(Array0DImpl {
        data: CSPBoolExpr::Or(terms),
    })
}

pub fn all<T>(values: T) -> Value<Array0DImpl<CSPBoolExpr>>
where
    T: IntoIterator,
    T::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
{
    let terms = values
        .into_iter()
        .map(|x| Box::new(x.as_expr_array().data))
        .collect();
    Value(Array0DImpl {
        data: CSPBoolExpr::And(terms),
    })
}

pub fn sum<T>(values: T) -> Value<Array0DImpl<CSPIntExpr>>
where
    T: IntoIterator,
    T::Item: Operand<Output = Array0DImpl<CSPIntExpr>>,
{
    let terms = values
        .into_iter()
        .map(|x| (Box::new(x.as_expr_array().data), 1))
        .collect();
    Value(Array0DImpl {
        data: CSPIntExpr::Linear(terms),
    })
}

pub fn consecutive_prefix_true<T>(values: T) -> Value<Array0DImpl<CSPIntExpr>>
where
    T: IntoIterator,
    T::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
{
    let terms: Vec<Value<Array0DImpl<CSPBoolExpr>>> = values
        .into_iter()
        .map(|x| Value(x.as_expr_array()))
        .collect();
    let mut ret = int_constant(0);
    for t in terms.iter().rev() {
        ret = t.ite(ret + 1, 0);
    }
    ret
}

impl<T> Value<Array2DImpl<T>>
where
    T: Clone,
    Value<Array2DImpl<T>>: Operand<Output = Array2DImpl<CSPBoolExpr>>,
{
    pub fn conv2d_and(&self, filter: (usize, usize)) -> Value<Array2DImpl<CSPBoolExpr>> {
        self.conv2d_impl(filter, CSPBoolExpr::And)
    }

    pub fn conv2d_or(&self, filter: (usize, usize)) -> Value<Array2DImpl<CSPBoolExpr>> {
        self.conv2d_impl(filter, CSPBoolExpr::Or)
    }

    fn conv2d_impl<F>(&self, filter: (usize, usize), op: F) -> Value<Array2DImpl<CSPBoolExpr>>
    where
        F: Fn(Vec<Box<CSPBoolExpr>>) -> CSPBoolExpr,
    {
        let orig = self.as_expr_array();
        let (h, w) = orig.shape;
        let (fh, fw) = filter;
        assert!(h >= fh);
        assert!(w >= fw);

        let mut data = vec![];
        for y in 0..=(h - fh) {
            for x in 0..=(w - fw) {
                let mut part = vec![];
                for dy in 0..fh {
                    for dx in 0..fw {
                        part.push(Box::new(orig.data[(y + dy) * w + (x + dx)].clone()));
                    }
                }
                data.push(op(part));
            }
        }

        Value(Array2DImpl {
            shape: (h - fh + 1, w - fw + 1),
            data,
        })
    }
}

impl<T> Value<T>
where
    Value<T>: IntoIterator + Clone,
    <Value<T> as IntoIterator>::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
{
    pub fn count_true(&self) -> Value<Array0DImpl<CSPIntExpr>> {
        count_true(self.clone())
    }

    pub fn any(&self) -> Value<Array0DImpl<CSPBoolExpr>> {
        any(self.clone())
    }

    pub fn all(&self) -> Value<Array0DImpl<CSPBoolExpr>> {
        all(self.clone())
    }

    pub fn consecutive_prefix_true(&self) -> Value<Array0DImpl<CSPIntExpr>> {
        consecutive_prefix_true(self.clone())
    }
}

impl<T> Value<T>
where
    Value<T>: IntoIterator + Clone,
    <Value<T> as IntoIterator>::Item: Operand<Output = Array0DImpl<CSPIntExpr>>,
{
    pub fn sum(&self) -> Value<Array0DImpl<CSPIntExpr>> {
        sum(self.clone())
    }
}

impl<T> Value<T>
where
    Value<T>: Clone + Operand,
{
    pub fn expr(&self) -> Value<<Self as Operand>::Output> {
        self.clone().as_expr_array_value()
    }
}

pub const TRUE: Value<Array0DImpl<CSPBoolExpr>> = Value(Array0DImpl {
    data: CSPBoolExpr::Const(true),
});
pub const FALSE: Value<Array0DImpl<CSPBoolExpr>> = Value(Array0DImpl {
    data: CSPBoolExpr::Const(false),
});

pub fn int_constant(n: i32) -> IntExpr {
    Value(Array0DImpl {
        data: CSPIntExpr::Const(n),
    })
}

// ==========
// Solver
// ==========

pub type BoolVar = Value<Array0DImpl<CSPBoolVar>>;
pub type BoolVarArray1D = Value<Array1DImpl<CSPBoolVar>>;
pub type BoolVarArray2D = Value<Array2DImpl<CSPBoolVar>>;
pub type BoolExpr = Value<Array0DImpl<CSPBoolExpr>>;
pub type IntVar = Value<Array0DImpl<CSPIntVar>>;
pub type IntVarArray1D = Value<Array1DImpl<CSPIntVar>>;
pub type IntVarArray2D = Value<Array2DImpl<CSPIntVar>>;
pub type IntExpr = Value<Array0DImpl<CSPIntExpr>>;

pub trait DerefVar {
    type Var;

    fn deref_var(self) -> Self::Var;
}

macro_rules! impl_deref_var {
    ($type_name:ty) => {
        impl DerefVar for Value<Array0DImpl<$type_name>> {
            type Var = Value<Array0DImpl<$type_name>>;

            fn deref_var(self) -> Self::Var {
                self
            }
        }

        impl DerefVar for &Value<Array0DImpl<$type_name>> {
            type Var = Value<Array0DImpl<$type_name>>;

            fn deref_var(self) -> Self::Var {
                self.clone()
            }
        }

        impl DerefVar for &&Value<Array0DImpl<$type_name>> {
            type Var = Value<Array0DImpl<$type_name>>;

            fn deref_var(self) -> Self::Var {
                self.clone().clone()
            }
        }
    };
}

impl_deref_var!(CSPBoolVar);
impl_deref_var!(CSPIntVar);

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
        Value(Array0DImpl {
            data: self.solver.new_bool_var(),
        })
    }

    pub fn add_prenormalize_var(&mut self, var: BoolVar) {
        self.solver.add_prenormalize_var(var.0.data);
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
        Value(Array1DImpl {
            data: (0..len).map(|_| self.solver.new_bool_var()).collect(),
        })
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
        Value(Array2DImpl {
            shape,
            data: (0..(h * w)).map(|_| self.solver.new_bool_var()).collect(),
        })
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
        Value(Array0DImpl {
            data: self.solver.new_int_var(Domain::range(low, high)),
        })
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
        Value(Array0DImpl {
            data: self.solver.new_int_var_from_list(domain),
        })
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
        Value(Array1DImpl {
            data: (0..len)
                .map(|_| self.solver.new_int_var(Domain::range(low, high)))
                .collect(),
        })
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
        Value(Array2DImpl {
            shape,
            data: (0..(h * w))
                .map(|_| self.solver.new_int_var(Domain::range(low, high)))
                .collect(),
        })
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
        Value(Array2DImpl {
            shape,
            data: (0..(h * w))
                .map(|i| {
                    let (low, high) = range[i / w][i % w];
                    self.solver.new_int_var(Domain::range(low, high))
                })
                .collect(),
        })
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
    pub fn add_expr<T>(&mut self, exprs: T)
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
    {
        exprs
            .into_iter()
            .for_each(|e| self.solver.add_expr(e.as_expr_array().data));
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
    pub fn all_different<T>(&mut self, exprs: T)
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: Operand<Output = Array0DImpl<CSPIntExpr>>,
    {
        let exprs = exprs
            .into_iter()
            .map(|e| e.as_expr_array().data)
            .collect::<Vec<_>>();
        self.solver.add_constraint(Stmt::AllDifferent(exprs));
    }

    pub fn add_active_vertices_connected<T>(&mut self, exprs: T, graph: &[(usize, usize)])
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
    {
        let vertices: Vec<CSPBoolExpr> =
            exprs.into_iter().map(|x| x.as_expr_array().data).collect();
        let n_vertices = vertices.len();
        for &(u, v) in graph {
            assert!(u < n_vertices);
            assert!(v < n_vertices);
        }
        self.solver
            .add_constraint(Stmt::ActiveVerticesConnected(vertices, graph.to_owned()));
    }

    pub fn add_graph_division<T>(
        &mut self,
        sizes: &[Option<Value<Array0DImpl<CSPIntExpr>>>],
        edges: &[(usize, usize)],
        edge_values: T,
    ) where
        T: IntoIterator,
        <T as IntoIterator>::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
    {
        self.add_graph_division_with_options(
            sizes,
            edges,
            edge_values,
            GraphDivisionOptions::default(),
        );
    }

    pub fn add_graph_division_with_options<T>(
        &mut self,
        sizes: &[Option<Value<Array0DImpl<CSPIntExpr>>>],
        edges: &[(usize, usize)],
        edge_values: T,
        opts: GraphDivisionOptions,
    ) where
        T: IntoIterator,
        <T as IntoIterator>::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
    {
        let sizes = sizes
            .into_iter()
            .map(|x| x.clone().map(|x| x.0.data))
            .collect::<Vec<_>>();
        let edge_values: Vec<CSPBoolExpr> = edge_values
            .into_iter()
            .map(|x| x.as_expr_array().data)
            .collect();
        self.solver.add_constraint(Stmt::GraphDivision(
            sizes,
            edges.to_owned(),
            edge_values,
            opts,
        ));
    }

    pub fn add_custom_constraint<T>(&mut self, constraint: Box<dyn PropagatorGenerator>, vars: T)
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: Operand<Output = Array0DImpl<CSPBoolExpr>>,
    {
        let vars = vars.into_iter().map(|x| x.as_expr_array().data).collect();
        self.solver
            .add_constraint(Stmt::CustomConstraint(vars, constraint));
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
        <T as IntoIterator>::Item: DerefVar<Var = Value<Array0DImpl<CSPBoolVar>>>,
    {
        self.answer_key_bool
            .extend(keys.into_iter().map(|x| x.deref_var().0.data))
    }

    /// Registers the specified integer variable(s) as the answer key(s).
    ///
    /// Variables representing the "answer" of the problem instance (not proxy variables) can be
    /// registered as the answer key. Answer keys are used in `irrefutable_facts` and `answer_iter` methods.
    pub fn add_answer_key_int<T>(&mut self, keys: T)
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: DerefVar<Var = Value<Array0DImpl<CSPIntVar>>>,
    {
        self.answer_key_int
            .extend(keys.into_iter().map(|x| x.deref_var().0.data))
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

pub trait MapForArray<A, B> {
    type Output;

    fn map<F>(&self, func: F) -> Self::Output
    where
        F: Fn(&A) -> B;
}

impl<A, B> MapForArray<A, B> for Array0DImpl<A> {
    type Output = B;

    fn map<F>(&self, func: F) -> B
    where
        F: Fn(&A) -> B,
    {
        func(&self.data)
    }
}

impl<A, B> MapForArray<A, B> for Array1DImpl<A> {
    type Output = Vec<B>;

    fn map<F>(&self, func: F) -> Vec<B>
    where
        F: Fn(&A) -> B,
    {
        self.data.iter().map(func).collect()
    }
}

impl<A, B> MapForArray<A, B> for Array2DImpl<A> {
    type Output = Vec<Vec<B>>;

    fn map<F>(&self, func: F) -> Vec<Vec<B>>
    where
        F: Fn(&A) -> B,
    {
        let func = &func;
        let (h, w) = self.shape;
        (0..h)
            .into_iter()
            .map(|i| self.data[(i * w)..((i + 1) * w)].iter().map(func).collect())
            .collect()
    }
}

pub trait FromModel {
    type Output;

    fn from_model(&self, model: &Model) -> Self::Output;
}

impl FromModel for Value<Array0DImpl<CSPBoolVar>> {
    type Output = <Array0DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::Output;

    fn from_model(&self, model: &Model) -> Self::Output {
        <Array0DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::map(&self.0, |v| {
            model.model.get_bool(*v)
        })
    }
}

impl FromModel for Value<Array1DImpl<CSPBoolVar>> {
    type Output = <Array1DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::Output;

    fn from_model(&self, model: &Model) -> Self::Output {
        <Array1DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::map(&self.0, |v| {
            model.model.get_bool(*v)
        })
    }
}

impl FromModel for Value<Array2DImpl<CSPBoolVar>> {
    type Output = <Array2DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::Output;

    fn from_model(&self, model: &Model) -> Self::Output {
        <Array2DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::map(&self.0, |v| {
            model.model.get_bool(*v)
        })
    }
}

impl FromModel for Value<Array0DImpl<CSPIntVar>> {
    type Output = <Array0DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::Output;

    fn from_model(&self, model: &Model) -> Self::Output {
        <Array0DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::map(&self.0, |v| {
            model.model.get_int(*v)
        })
    }
}

impl FromModel for Value<Array1DImpl<CSPIntVar>> {
    type Output = <Array1DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::Output;

    fn from_model(&self, model: &Model) -> Self::Output {
        <Array1DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::map(&self.0, |v| {
            model.model.get_int(*v)
        })
    }
}

impl FromModel for Value<Array2DImpl<CSPIntVar>> {
    type Output = <Array2DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::Output;

    fn from_model(&self, model: &Model) -> Self::Output {
        <Array2DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::map(&self.0, |v| {
            model.model.get_int(*v)
        })
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

pub trait FromOwnedPartialModel {
    type Output;
    type OutputUnwrap;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output;
    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap;
}

impl FromOwnedPartialModel for Value<Array0DImpl<CSPBoolVar>> {
    type Output = <Array0DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, Option<bool>>>::Output;
    type OutputUnwrap = <Array0DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::Output;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output {
        <Array0DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, Option<bool>>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_bool(*v)
        })
    }

    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap {
        <Array0DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_bool(*v).unwrap()
        })
    }
}

impl FromOwnedPartialModel for Value<Array1DImpl<CSPBoolVar>> {
    type Output = <Array1DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, Option<bool>>>::Output;
    type OutputUnwrap = <Array1DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::Output;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output {
        <Array1DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, Option<bool>>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_bool(*v)
        })
    }

    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap {
        <Array1DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_bool(*v).unwrap()
        })
    }
}

impl FromOwnedPartialModel for Value<Array2DImpl<CSPBoolVar>> {
    type Output = <Array2DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, Option<bool>>>::Output;
    type OutputUnwrap = <Array2DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::Output;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output {
        <Array2DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, Option<bool>>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_bool(*v)
        })
    }

    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap {
        <Array2DImpl<CSPBoolVar> as MapForArray<CSPBoolVar, bool>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_bool(*v).unwrap()
        })
    }
}

impl FromOwnedPartialModel for Value<Array0DImpl<CSPIntVar>> {
    type Output = <Array0DImpl<CSPIntVar> as MapForArray<CSPIntVar, Option<i32>>>::Output;
    type OutputUnwrap = <Array0DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::Output;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output {
        <Array0DImpl<CSPIntVar> as MapForArray<CSPIntVar, Option<i32>>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_int(*v)
        })
    }

    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap {
        <Array0DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_int(*v).unwrap()
        })
    }
}

impl FromOwnedPartialModel for Value<Array1DImpl<CSPIntVar>> {
    type Output = <Array1DImpl<CSPIntVar> as MapForArray<CSPIntVar, Option<i32>>>::Output;
    type OutputUnwrap = <Array1DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::Output;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output {
        <Array1DImpl<CSPIntVar> as MapForArray<CSPIntVar, Option<i32>>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_int(*v)
        })
    }

    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap {
        <Array1DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_int(*v).unwrap()
        })
    }
}

impl FromOwnedPartialModel for Value<Array2DImpl<CSPIntVar>> {
    type Output = <Array2DImpl<CSPIntVar> as MapForArray<CSPIntVar, Option<i32>>>::Output;
    type OutputUnwrap = <Array2DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::Output;

    fn from_irrefutable_facts(&self, irrefutable_facts: &OwnedPartialModel) -> Self::Output {
        <Array2DImpl<CSPIntVar> as MapForArray<CSPIntVar, Option<i32>>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_int(*v)
        })
    }

    fn from_irrefutable_facts_unwrap(
        &self,
        irrefutable_facts: &OwnedPartialModel,
    ) -> Self::OutputUnwrap {
        <Array2DImpl<CSPIntVar> as MapForArray<CSPIntVar, i32>>::map(&self.0, |v| {
            irrefutable_facts.assignment.get_int(*v).unwrap()
        })
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

pub mod ops {

    use super::*;

    pub trait GenericComparable<Rhs> {
        type Output;

        fn eq(&self, rhs: &Rhs) -> Self::Output;
        fn ne(&self, rhs: &Rhs) -> Self::Output;
        fn le(&self, rhs: &Rhs) -> Self::Output;
        fn lt(&self, rhs: &Rhs) -> Self::Output;
        fn ge(&self, rhs: &Rhs) -> Self::Output;
        fn gt(&self, rhs: &Rhs) -> Self::Output;
    }

    impl<X> GenericComparable<X> for X
    where
        X: PartialOrd,
    {
        type Output = bool;

        fn eq(&self, rhs: &X) -> Self::Output {
            self == rhs
        }
        fn ne(&self, rhs: &X) -> Self::Output {
            self == rhs
        }
        fn le(&self, rhs: &X) -> Self::Output {
            self < rhs
        }
        fn lt(&self, rhs: &X) -> Self::Output {
            self <= rhs
        }
        fn ge(&self, rhs: &X) -> Self::Output {
            self >= rhs
        }
        fn gt(&self, rhs: &X) -> Self::Output {
            self > rhs
        }
    }

    impl<X, Y, Z> GenericComparable<Y> for Value<X>
    where
        Y: Clone,
        for<'a, 'b> (&'a Value<X>, &'b Y):
            PropagateBinaryGeneric<CSPIntExpr, CSPIntExpr, CSPBoolExpr, Output = Z>,
    {
        type Output = Value<Z>;

        fn eq(&self, rhs: &Y) -> Self::Output {
            Value::eq(self, rhs)
        }
        fn ne(&self, rhs: &Y) -> Self::Output {
            Value::ne(self, rhs)
        }
        fn le(&self, rhs: &Y) -> Self::Output {
            Value::le(self, rhs)
        }
        fn lt(&self, rhs: &Y) -> Self::Output {
            Value::lt(self, rhs)
        }
        fn ge(&self, rhs: &Y) -> Self::Output {
            Value::ge(self, rhs)
        }
        fn gt(&self, rhs: &Y) -> Self::Output {
            Value::gt(self, rhs)
        }
    }

    impl<X, Y, Z> GenericComparable<Y> for &Value<X>
    where
        Y: Clone,
        for<'a, 'b> (&'a Value<X>, &'b Y):
            PropagateBinaryGeneric<CSPIntExpr, CSPIntExpr, CSPBoolExpr, Output = Z>,
    {
        type Output = Value<Z>;

        fn eq(&self, rhs: &Y) -> Self::Output {
            Value::eq(self, rhs)
        }
        fn ne(&self, rhs: &Y) -> Self::Output {
            Value::ne(self, rhs)
        }
        fn le(&self, rhs: &Y) -> Self::Output {
            Value::le(self, rhs)
        }
        fn lt(&self, rhs: &Y) -> Self::Output {
            Value::lt(self, rhs)
        }
        fn ge(&self, rhs: &Y) -> Self::Output {
            Value::ge(self, rhs)
        }
        fn gt(&self, rhs: &Y) -> Self::Output {
            Value::gt(self, rhs)
        }
    }

    pub fn eq<X, Y>(lhs: &X, rhs: &Y) -> <X as GenericComparable<Y>>::Output
    where
        X: GenericComparable<Y>,
    {
        lhs.eq(rhs)
    }

    pub fn ne<X, Y>(lhs: &X, rhs: &Y) -> <X as GenericComparable<Y>>::Output
    where
        X: GenericComparable<Y>,
    {
        lhs.ne(rhs)
    }

    pub fn le<X, Y>(lhs: &X, rhs: &Y) -> <X as GenericComparable<Y>>::Output
    where
        X: GenericComparable<Y>,
    {
        lhs.le(rhs)
    }

    pub fn lt<X, Y>(lhs: &X, rhs: &Y) -> <X as GenericComparable<Y>>::Output
    where
        X: GenericComparable<Y>,
    {
        lhs.lt(rhs)
    }

    pub fn ge<X, Y>(lhs: &X, rhs: &Y) -> <X as GenericComparable<Y>>::Output
    where
        X: GenericComparable<Y>,
    {
        lhs.ge(rhs)
    }

    pub fn gt<X, Y>(lhs: &X, rhs: &Y) -> <X as GenericComparable<Y>>::Output
    where
        X: GenericComparable<Y>,
    {
        lhs.gt(rhs)
    }

    pub trait GenericCallable2<X, Y, Z> {
        fn call(&self, a1: X, a2: Y) -> Z;
    }

    impl<X, Y, Z> GenericCallable2<X, Y, Z> for fn(X, Y) -> Z {
        fn call(&self, a1: X, a2: Y) -> Z {
            self(a1, a2)
        }
    }

    impl<T> GenericCallable2<usize, usize, Value<Array0DImpl<T>>> for Value<Array2DImpl<T>>
    where
        T: Clone,
    {
        fn call(&self, y: usize, x: usize) -> Value<Array0DImpl<T>> {
            self.at((y, x))
        }
    }

    impl<T> GenericCallable2<usize, usize, Value<Array0DImpl<T>>> for &Value<Array2DImpl<T>>
    where
        T: Clone,
    {
        fn call(&self, y: usize, x: usize) -> Value<Array0DImpl<T>> {
            self.at((y, x))
        }
    }

    pub fn call<F, X, Y, Z>(f: F, x: X, y: Y) -> Z
    where
        F: GenericCallable2<X, Y, Z>,
    {
        f.call(x, y)
    }
}

pub use cspuz_rs_macro::_expr_impl;

#[macro_export]
macro_rules! expr {
    ($x:expr) => {
        $crate::solver::_expr_impl!($x, $crate)
    };
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
        solver.add_answer_key_bool(&[b0d]);
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

    #[test]
    fn test_expr_macro() {
        let mut solver = Solver::new();
        let x = &solver.int_var(0, 5);
        let y = &solver.int_var(0, 5);
        solver.add_answer_key_int(x);
        solver.add_answer_key_int(y);
        solver.add_expr(expr!(x > y));

        let n_ans = solver.answer_iter().count();
        assert_eq!(n_ans, 15);
    }
}
