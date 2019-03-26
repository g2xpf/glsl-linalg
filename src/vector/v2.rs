use crate::float::Float;
use crate::matrix::M2;
use crate::numeric::Numeric;
use crate::vector::{Cross, FloatVector, Vector, V2, V3};
use std::ops::{Add, Deref, Div, Mul, Sub};

impl<T> Deref for V2<T>
where
    T: Numeric,
{
    type Target = [T; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Vector<T> for V2<T>
where
    T: Numeric,
{
    fn dot(&self, rhs: Self) -> T {
        self[0] * rhs[0] + self[1] * rhs[1]
    }
}

impl<F> FloatVector<F> for V2<F> where F: Float {}

impl<T> Cross<T> for V2<T>
where
    T: Numeric,
{
    fn cross(&self, rhs: Self) -> V3<T> {
        let default: T = Default::default();
        V3([default, default, self[0] * rhs[1] - self[1] * rhs[0]])
    }
}

impl<T> Add for V2<T>
where
    T: Numeric,
{
    type Output = V2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        V2([self[0] + rhs[0], self[1] + rhs[1]])
    }
}

impl<T> Sub for V2<T>
where
    T: Numeric,
{
    type Output = V2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        V2([self[0] - rhs[0], self[1] - rhs[1]])
    }
}

impl<T> Mul for V2<T>
where
    T: Numeric,
{
    type Output = V2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        V2([self[0] * rhs[0], self[1] * rhs[1]])
    }
}

impl<T> Mul<T> for V2<T>
where
    T: Numeric,
{
    type Output = V2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        V2([self[0] * rhs, self[1] * rhs])
    }
}

impl<T> Mul<M2<T>> for V2<T>
where
    T: Numeric,
{
    type Output = V2<T>;

    fn mul(self, rhs: M2<T>) -> Self::Output {
        V2([
            self[0] * rhs[0][0] + self[1] * rhs[1][0],
            self[0] * rhs[0][1] + self[1] * rhs[1][1],
        ])
    }
}

impl<T> Div for V2<T>
where
    T: Numeric,
{
    type Output = V2<T>;
    fn div(self, rhs: Self) -> Self::Output {
        V2([self[0] / rhs[0], self[1] / rhs[1]])
    }
}

impl<T> Div<T> for V2<T>
where
    T: Numeric,
{
    type Output = V2<T>;

    fn div(self, rhs: T) -> Self::Output {
        V2([self[0] / rhs, self[1] / rhs])
    }
}
