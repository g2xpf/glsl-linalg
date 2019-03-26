use crate::float::Float;
use crate::matrix::M3;
use crate::numeric::Numeric;
use crate::vector::{Cross, FloatVector, Vector, V3};
use std::ops::{Add, Deref, Div, Mul, Sub};

impl<T> Deref for V3<T>
where
    T: Numeric,
{
    type Target = [T; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Vector<T> for V3<T>
where
    T: Numeric,
{
    fn dot(&self, rhs: Self) -> T {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

impl<F> FloatVector<F> for V3<F> where F: Float {}

impl<T> Cross<T> for V3<T>
where
    T: Numeric,
{
    fn cross(&self, rhs: Self) -> V3<T> {
        V3([
            self[1] * rhs[2] - self[2] - rhs[1],
            self[2] * rhs[0] - self[0] - rhs[2],
            self[0] * rhs[1] - self[1] - rhs[0],
        ])
    }
}

impl<T> Add for V3<T>
where
    T: Numeric,
{
    type Output = V3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        V3([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl<T> Sub for V3<T>
where
    T: Numeric,
{
    type Output = V3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        V3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl<T> Mul for V3<T>
where
    T: Numeric,
{
    type Output = V3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        V3([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
    }
}

impl<T> Mul<T> for V3<T>
where
    T: Numeric,
{
    type Output = V3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        V3([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}

impl<T> Mul<M3<T>> for V3<T>
where
    T: Numeric,
{
    type Output = V3<T>;

    fn mul(self, rhs: M3<T>) -> Self::Output {
        V3([
            self[0] * rhs[0][0] + self[1] * rhs[1][0] + self[2] * rhs[2][0],
            self[0] * rhs[0][1] + self[1] * rhs[1][1] + self[2] * rhs[2][1],
            self[0] * rhs[0][2] + self[1] * rhs[1][2] + self[2] * rhs[2][2],
        ])
    }
}

impl<T> Div for V3<T>
where
    T: Numeric,
{
    type Output = V3<T>;
    fn div(self, rhs: Self) -> Self::Output {
        V3([self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]])
    }
}

impl<T> Div<T> for V3<T>
where
    T: Numeric,
{
    type Output = V3<T>;
    fn div(self, rhs: T) -> Self::Output {
        V3([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}
