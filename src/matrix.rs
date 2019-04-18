pub mod m2;
pub mod m3;
pub mod m4;

use crate::float::Float;
use std::ops::Div;

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct M2<T>(pub [[T; 2]; 2]);

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct M3<T>(pub [[T; 3]; 3]);

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct M4<T>(pub [[T; 4]; 4]);

pub trait Matrix {
    fn transpose(&mut self);
}

pub trait FloatMatrix<F>: Matrix
where
    F: Float,
    Self: Clone + Div<F, Output = Self>,
{
    fn determinant(&self) -> F;
    #[inline]
    fn cofactor(&self) -> Self;
    #[inline]
    fn inverse(&self) -> Self {
        self.cofactor() / self.determinant()
    }
}

pub trait IntoVectors<V> {
    fn into_cols(&self) -> V;
    fn into_rows(&self) -> V;
}

pub trait FromVectors<V> {
    fn from_cols(v: V) -> Self;
    fn from_rows(v: V) -> Self;
}
