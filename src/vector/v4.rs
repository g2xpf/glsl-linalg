use crate::float::Float;
use crate::matrix::M4;
use crate::numeric::Numeric;
use crate::vector::{FloatVector, Vector, V4};
use std::ops::{Add, Deref, Div, Mul, Sub};

impl<T> Deref for V4<T>
where
    T: Numeric,
{
    type Target = [T; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Vector<T> for V4<T>
where
    T: Numeric,
{
    #[inline]
    fn dot(&self, rhs: Self) -> T {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2] + self[3] * rhs[3]
    }
}

impl<F> FloatVector<F> for V4<F> where F: Float {}

impl<T> Add for V4<T>
where
    T: Numeric,
{
    type Output = V4<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        V4([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ])
    }
}

impl<T> Sub for V4<T>
where
    T: Numeric,
{
    type Output = V4<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        V4([
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        ])
    }
}

impl<T> Mul for V4<T>
where
    T: Numeric,
{
    type Output = V4<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        V4([
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
            self[3] * rhs[3],
        ])
    }
}

impl<T> Mul<T> for V4<T>
where
    T: Numeric,
{
    type Output = V4<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        V4([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs])
    }
}

impl<T> Mul<M4<T>> for V4<T>
where
    T: Numeric,
{
    type Output = V4<T>;

    #[inline]
    fn mul(self, rhs: M4<T>) -> Self::Output {
        V4([
            self[0] * rhs[0][0] + self[1] * rhs[1][0] + self[2] * rhs[2][0] + self[3] * rhs[3][0],
            self[0] * rhs[0][1] + self[1] * rhs[1][1] + self[2] * rhs[2][1] + self[3] * rhs[3][1],
            self[0] * rhs[0][2] + self[1] * rhs[1][2] + self[2] * rhs[2][2] + self[3] * rhs[3][2],
            self[0] * rhs[0][3] + self[1] * rhs[1][3] + self[2] * rhs[2][3] + self[3] * rhs[3][3],
        ])
    }
}

impl<T> Div for V4<T>
where
    T: Numeric,
{
    type Output = V4<T>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        V4([
            self[0] / rhs[0],
            self[1] / rhs[1],
            self[2] / rhs[2],
            self[3] / rhs[3],
        ])
    }
}

impl<T> Div<T> for V4<T>
where
    T: Numeric,
{
    type Output = V4<T>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        V4([self[0] / rhs, self[1] / rhs, self[2] / rhs, self[3] / rhs])
    }
}
