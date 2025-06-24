use std::ops::{Bound, RangeBounds};
use crate::items::Arrow;
pub use super::traits::{Operand, PropagateBinary, PropagateTernary};

use cspuz_core::csp::BoolExpr as CSPBoolExpr;
use cspuz_core::csp::BoolVar as CSPBoolVar;
use cspuz_core::csp::IntExpr as CSPIntExpr;
use cspuz_core::csp::IntVar as CSPIntVar;

// TODO: we may want to avoid Vec<T> for 0-dimensional arrays
#[derive(Debug, Clone)]
pub struct NdArray<S, T> {
    pub(super) shape: S,
    pub(super) data: Vec<T>,
}

impl<S, T> IntoIterator for NdArray<S, T> {
    type Item = NdArray<(), T>;
    type IntoIter = std::iter::Map<std::vec::IntoIter<T>, fn(T) -> Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().map(|v| NdArray {
            shape: (),
            data: vec![v],
        })
    }
}

impl<S, T: Clone> IntoIterator for &NdArray<S, T> {
    type Item = NdArray<(), T>;
    type IntoIter = std::iter::Map<std::vec::IntoIter<T>, fn(T) -> Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.clone().into_iter().map(|v| NdArray {
            shape: (),
            data: vec![v],
        })
    }
}

impl<S: Clone> Operand for NdArray<S, CSPBoolExpr> {
    type Shape = S;
    type Value = CSPBoolExpr;

    fn as_ndarray(&self) -> NdArray<Self::Shape, Self::Value> {
        self.clone()
    }
}

impl<S: Clone> Operand for NdArray<S, CSPBoolVar> {
    type Shape = S;
    type Value = CSPBoolExpr;

    fn as_ndarray(&self) -> NdArray<Self::Shape, Self::Value> {
        NdArray {
            shape: self.shape.clone(),
            data: self.data.iter().map(|v| v.expr()).collect(),
        }
    }
}

impl<S: Clone> Operand for NdArray<S, CSPIntExpr> {
    type Shape = S;
    type Value = CSPIntExpr;

    fn as_ndarray(&self) -> NdArray<Self::Shape, Self::Value> {
        self.clone()
    }
}

impl<S: Clone> Operand for NdArray<S, CSPIntVar> {
    type Shape = S;
    type Value = CSPIntExpr;

    fn as_ndarray(&self) -> NdArray<Self::Shape, Self::Value> {
        NdArray {
            shape: self.shape.clone(),
            data: self.data.iter().map(|v| v.expr()).collect(),
        }
    }
}

// ==========
// Accessors
// ==========

impl<T: Clone> NdArray<(usize, ), T> {
    pub fn len(&self) -> usize {
        self.shape.0
    }

    pub fn at(&self, idx: usize) -> NdArray<(), T> {
        NdArray {
            shape: (),
            data: vec![self.data[idx].clone()],
        }
    }

    pub fn reverse(&self) -> NdArray<(usize, ), T> {
        let mut data = self.data.clone();
        data.reverse();
        NdArray {
            shape: self.shape,
            data,
        }
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

impl<T: Clone> NdArray<(usize, ), T> {
    pub fn reshape_as_2d(&self, shape: (usize, usize)) -> NdArray<(usize, usize), T> {
        let (height, width) = shape;
        assert_eq!(height * width, self.data.len());
        NdArray {
            shape,
            data: self.data.clone(),
        }
    }

    pub fn slice<I: RangeBounds<usize>>(&self, idx: I) -> NdArray<(usize, ), T> {
        let (start, end) = resolve_range(self.len(), &idx);
        NdArray {
            shape: (end - start,),
            data: self.data[start..end].to_vec(),
        }
    }
}

impl<T> NdArray<(usize, usize), T> {
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    fn at_raw(&self, idx: (usize, usize)) -> &T {
        let (y, x) = idx;
        let (h, w) = self.shape;
        assert!(y < h && x < w);
        &self.data[y * w + x]
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

impl<T: Clone> NdArray<(usize, usize), T> {
    pub fn at(&self, idx: (usize, usize)) -> NdArray<(), T> {
        NdArray {
            shape: (),
            data: vec![self.at_raw(idx).clone()],
        }
    }

    pub fn at_offset<D, E>(
        &self,
        idx: (usize, usize),
        offset: (i32, i32),
        default: D,
    ) -> NdArray<(), E>
    where
        D: Operand<Shape = (), Value = E>,
        NdArray<(), T>: Operand<Shape = (), Value = E>,
    {
        let (y, x) = idx;
        let (dy, dx) = offset;
        let y = y as i32 + dy;
        let x = x as i32 + dx;
        if 0 <= y && y < self.shape().0 as i32 && 0 <= x && x < self.shape().1 as i32 {
            self.at((y as usize, x as usize)).as_ndarray()
        } else {
            default.as_ndarray()
        }
    }

    pub fn select<I, X>(&self, idx: I) -> NdArray<(usize, ), T>
    where
        I: IntoIterator<Item = X>,
        X: std::borrow::Borrow<(usize, usize)>,
    {
        let mut data = vec![];
        let (h, w) = self.shape;
        for p in idx {
            let &(y, x) = p.borrow();
            assert!(y < h && x < w);
            data.push(self.data[y * w + x].clone());
        }

        NdArray {
            shape: (data.len(),),
            data,
        }
    }

    pub fn slice_fixed_y<X: RangeBounds<usize>>(&self, idx: (usize, X)) -> NdArray<(usize, ), T> {
        let (y, xs) = idx;
        let (_, w) = self.shape;
        let (x_start, x_end) = resolve_range(w, &xs);

        let items: Vec<_> = (x_start..x_end).map(|x| self.at_raw((y, x)).clone()).collect();
        NdArray {
            shape: (items.len(),),
            data: items,
        }
    }

    pub fn slice_fixed_x<Y: RangeBounds<usize>>(&self, idx: (Y, usize)) -> NdArray<(usize, ), T> {
        let (ys, x) = idx;
        let (h, _) = self.shape;
        let (y_start, y_end) = resolve_range(h, &ys);

        let items: Vec<T> = (y_start..y_end).map(|y| self.at_raw((y, x)).clone()).collect();
        NdArray {
            shape: (items.len(),),
            data: items,
        }
    }

    pub fn slice<Y: RangeBounds<usize>, X: RangeBounds<usize>>(
        &self,
        idx: (Y, X),
    ) -> NdArray<(usize, usize), T> {
        let (ys, xs) = idx;
        let (h, w) = self.shape;
        let (y_start, y_end) = resolve_range(h, &ys); // [y_start, y_end)
        let (x_start, x_end) = resolve_range(w, &xs); // [x_start, x_end)

        let slice_shape = (y_end - y_start, x_end - x_start);
        let mut items = vec![];
        for y in y_start..y_end {
            for x in x_start..x_end {
                items.push(self.at_raw((y, x)).clone());
            }
        }
        NdArray {
            shape: slice_shape,
            data: items,
        }
    }

    pub fn flatten(&self) -> NdArray<(usize, ), T> {
        NdArray {
            shape: (self.shape.0 * self.shape.1,),
            data: self.data.clone(),
        }
    }

    pub fn reshape(&self, shape: (usize, usize)) -> NdArray<(usize, usize), T> {
        let (height, width) = shape;
        assert_eq!(height * width, self.data.len());
        NdArray {
            shape,
            data: self.data.clone(),
        }
    }

    pub fn four_neighbors(&self, idx: (usize, usize)) -> NdArray<(usize, ), T> {
        self.select(self.four_neighbor_indices(idx))
    }

    pub fn pointing_cells(
        &self,
        cell: (usize, usize),
        arrow: Arrow,
    ) -> Option<NdArray<(usize, ), T>> {
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
// Operators
// ==========

macro_rules! binary_op_overload {
    ($trait_name:ident, $trait_func:ident, $input_type:ty, $output_type:ty, $op:tt) => {
        use std::ops::$trait_name;

        impl<A, B, S> $trait_name<B> for NdArray<S, A>
        where
            (NdArray<S, A>, B): PropagateBinary<$input_type, $input_type, $output_type>,
        {
            type Output = <(NdArray<S, A>, B) as PropagateBinary<$input_type, $input_type, $output_type>>::Output;

            fn $trait_func(self, rhs: B) -> Self::Output {
                (self, rhs).propagate_binary(|x, y| {
                    x $op y
                })
            }
        }

        impl<'a, A, B, S> $trait_name<B> for &'a NdArray<S, A>
        where
            (&'a NdArray<S, A>, B): PropagateBinary<$input_type, $input_type, $output_type>,
        {
            type Output = <(&'a NdArray<S, A>, B) as PropagateBinary<$input_type, $input_type, $output_type>>::Output;

            fn $trait_func(self, rhs: B) -> Self::Output {
                (self, rhs).propagate_binary(|x, y| {
                    x $op y
                })
            }
        }
    }
}

binary_op_overload!(BitAnd, bitand, CSPBoolExpr, CSPBoolExpr, &);
binary_op_overload!(BitOr, bitor, CSPBoolExpr, CSPBoolExpr, |);
binary_op_overload!(BitXor, bitxor, CSPBoolExpr, CSPBoolExpr, ^);
binary_op_overload!(Add, add, CSPIntExpr, CSPIntExpr, +);
binary_op_overload!(Sub, sub, CSPIntExpr, CSPIntExpr, -);

macro_rules! binary_op {
    ($func_name:ident, $input_type:ty, $output_type:ty, $op_func:expr) => {
        impl<S, A> NdArray<S, A> {
            pub fn $func_name<'a, B>(&'a self, rhs: B) -> <(&'a NdArray<S, A>, B) as PropagateBinary<$input_type, $input_type, $output_type>>::Output
            where
                (&'a NdArray<S, A>, B): PropagateBinary<$input_type, $input_type, $output_type>,
            {
                (self.clone(), rhs).propagate_binary($op_func)
            }
        }
    }
}

binary_op!(eq, CSPIntExpr, CSPBoolExpr, |x, y| x.eq(y));
binary_op!(ne, CSPIntExpr, CSPBoolExpr, |x, y| x.ne(y));
binary_op!(ge, CSPIntExpr, CSPBoolExpr, |x, y| x.ge(y));
binary_op!(gt, CSPIntExpr, CSPBoolExpr, |x, y| x.gt(y));
binary_op!(le, CSPIntExpr, CSPBoolExpr, |x, y| x.le(y));
binary_op!(lt, CSPIntExpr, CSPBoolExpr, |x, y| x.lt(y));
binary_op!(imp, CSPBoolExpr, CSPBoolExpr, |x, y| x.imp(y));
binary_op!(iff, CSPBoolExpr, CSPBoolExpr, |x, y| x.iff(y));

impl<S, A> NdArray<S, A> {
    pub fn ite<'a, B, C>(&'a self, if_true: B, if_false: C) -> <(&'a NdArray<S, A>, B, C) as PropagateTernary<CSPBoolExpr, CSPIntExpr, CSPIntExpr, CSPIntExpr>>::Output
    where
        (&'a NdArray<S, A>, B, C): PropagateTernary<CSPBoolExpr, CSPIntExpr, CSPIntExpr, CSPIntExpr>,
        B: Operand<Value = CSPIntExpr>,
        C: Operand<Value = CSPIntExpr>,
    {
        (self, if_true, if_false).propagate_ternary(|cond, true_val, false_val| {
            cond.ite(true_val, false_val)
        })
    }
}
