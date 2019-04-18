use crate::matrix::{M2, M3, M4};
use crate::vector::{V2, V3, V4};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Numeric
where
    Self: Default
        + Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Div<Output = Self>
        + Mul<Output = Self>
        + Neg<Output = Self>,
{
}

macro_rules! impl_mul {
    ($t: ty) => {
        impl std::ops::Mul<V2<$t>> for $t {
            type Output = V2<$t>;

            #[inline]
            fn mul(self, rhs: V2<$t>) -> Self::Output {
                V2([self * rhs[0], self * rhs[1]])
            }
        }

        impl std::ops::Mul<V3<$t>> for $t {
            type Output = V3<$t>;

            #[inline]
            fn mul(self, rhs: V3<$t>) -> Self::Output {
                V3([self * rhs[0], self * rhs[1], self * rhs[2]])
            }
        }

        impl std::ops::Mul<V4<$t>> for $t {
            type Output = V4<$t>;

            #[inline]
            fn mul(self, rhs: V4<$t>) -> Self::Output {
                V4([self * rhs[0], self * rhs[1], self * rhs[2], self * rhs[3]])
            }
        }

        impl std::ops::Mul<M2<$t>> for $t {
            type Output = M2<$t>;

            #[inline]
            fn mul(self, rhs: M2<$t>) -> Self::Output {
                M2([
                    [self * rhs[0][0], self * rhs[0][1]],
                    [self * rhs[1][0], self * rhs[1][1]],
                ])
            }
        }

        impl std::ops::Mul<M3<$t>> for $t {
            type Output = M3<$t>;

            #[inline]
            fn mul(self, rhs: M3<$t>) -> Self::Output {
                M3([
                    [self * rhs[0][0], self * rhs[0][1], self * rhs[0][2]],
                    [self * rhs[1][0], self * rhs[1][1], self * rhs[1][2]],
                    [self * rhs[2][0], self * rhs[2][1], self * rhs[2][2]],
                ])
            }
        }

        impl std::ops::Mul<M4<$t>> for $t {
            type Output = M4<$t>;

            #[inline]
            fn mul(self, rhs: M4<$t>) -> Self::Output {
                M4([
                    [
                        self * rhs[0][0],
                        self * rhs[0][1],
                        self * rhs[0][2],
                        self * rhs[0][3],
                    ],
                    [
                        self * rhs[1][0],
                        self * rhs[1][1],
                        self * rhs[1][2],
                        self * rhs[1][3],
                    ],
                    [
                        self * rhs[2][0],
                        self * rhs[2][1],
                        self * rhs[2][2],
                        self * rhs[2][3],
                    ],
                    [
                        self * rhs[3][0],
                        self * rhs[3][1],
                        self * rhs[3][2],
                        self * rhs[3][3],
                    ],
                ])
            }
        }
    };
}

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

impl_mul!(i8);
impl_mul!(i16);
impl_mul!(i32);
impl_mul!(i64);
impl_mul!(f32);
impl_mul!(f64);
