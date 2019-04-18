pub mod v2;
pub mod v3;
pub mod v4;

use crate::float::Float;
use crate::numeric::Numeric;
use std::ops::{Div, Sub};

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct V2<T>(pub [T; 2]);

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct V3<T>(pub [T; 3]);

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct V4<T>(pub [T; 4]);

pub trait Vector<T>
where
    T: Numeric,
{
    fn dot(&self, rhs: Self) -> T;
}

pub trait FloatVector<F>: Vector<F>
where
    F: Float,
    Self: Clone + Div<F, Output = Self> + Sub<Output = Self>,
{
    #[inline]
    fn length(&self) -> F {
        self.dot(self.clone()).f_sqrt()
    }

    #[inline]
    fn normalize(&self) -> Self {
        self.clone() / self.length()
    }

    #[inline]
    fn distance(&self, rhs: &Self) -> F {
        (self.clone() - rhs.clone()).length()
    }
}

pub trait Cross<T, RHS = Self> {
    fn cross(&self, rhs: RHS) -> V3<T>;
}
