use crate::float::Float;
use crate::matrix::{FloatMatrix, FromVectors, IntoVectors, Matrix, M3};
use crate::numeric::Numeric;
use crate::vector::{Vector, V3};
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

impl<T> Deref for M3<T>
where
    T: Numeric,
{
    type Target = [[T; 3]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for M3<T>
where
    T: Numeric,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoVectors<(V3<T>, V3<T>, V3<T>)> for M3<T>
where
    T: Numeric,
{
    fn into_cols(&self) -> (V3<T>, V3<T>, V3<T>) {
        (
            V3([self[0][0], self[1][0], self[2][0]]),
            V3([self[0][1], self[1][1], self[2][1]]),
            V3([self[0][2], self[1][2], self[2][2]]),
        )
    }

    fn into_rows(&self) -> (V3<T>, V3<T>, V3<T>) {
        (V3(self[0]), V3(self[1]), V3(self[2]))
    }
}

impl<T> FromVectors<(V3<T>, V3<T>, V3<T>)> for M3<T>
where
    T: Numeric,
{
    fn from_cols(v: (V3<T>, V3<T>, V3<T>)) -> Self {
        let (r1, r2, r3) = v;
        M3([
            [r1[0], r2[0], r3[0]],
            [r1[1], r2[1], r3[1]],
            [r1[2], r2[2], r3[2]],
        ])
    }

    fn from_rows(v: (V3<T>, V3<T>, V3<T>)) -> Self {
        let (V3(c1), V3(c2), V3(c3)) = v;
        M3([c1, c2, c3])
    }
}

impl<T> Matrix for M3<T>
where
    T: Numeric,
{
    fn transpose(&mut self) {
        unsafe {
            (&mut self[0][1] as *mut T).swap(&mut self[1][0]);
            (&mut self[0][2] as *mut T).swap(&mut self[2][0]);
            (&mut self[1][2] as *mut T).swap(&mut self[2][1]);
        }
    }
}

impl<F> FloatMatrix<F> for M3<F>
where
    F: Float,
{
    fn determinant(&self) -> F {
        self[0][0] * self[1][1] * self[2][2]
            + self[0][1] * self[1][2] * self[2][0]
            + self[0][2] * self[1][0] * self[2][1]
            - self[0][0] * self[1][2] * self[2][1]
            - self[0][1] * self[1][0] * self[2][2]
            - self[0][2] * self[1][1] * self[2][0]
    }

    fn cofactor(&self) -> Self {
        M3([
            [
                self[1][1] * self[2][2] - self[1][2] * self[2][1],
                self[0][2] * self[2][1] - self[0][1] * self[2][2],
                self[0][1] * self[1][2] - self[0][2] * self[1][1],
            ],
            [
                self[1][2] * self[2][0] - self[1][0] * self[2][2],
                self[0][0] * self[2][2] - self[0][2] * self[2][0],
                self[0][2] * self[1][0] - self[0][0] * self[1][2],
            ],
            [
                self[1][0] * self[2][1] - self[1][1] * self[2][0],
                self[0][1] * self[2][0] - self[0][0] * self[2][1],
                self[0][0] * self[1][1] - self[0][1] * self[1][0],
            ],
        ])
    }
}

impl<T> Add for M3<T>
where
    T: Numeric,
{
    type Output = M3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        M3([
            [
                self[0][0] + rhs[0][0],
                self[0][1] + rhs[0][1],
                self[0][2] + rhs[0][2],
            ],
            [
                self[1][0] + rhs[1][0],
                self[1][1] + rhs[1][1],
                self[1][2] + rhs[1][2],
            ],
            [
                self[2][0] + rhs[2][0],
                self[2][1] + rhs[2][1],
                self[2][2] + rhs[2][2],
            ],
        ])
    }
}

impl<T> Sub for M3<T>
where
    T: Numeric,
{
    type Output = M3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        M3([
            [
                self[0][0] - rhs[0][0],
                self[0][1] - rhs[0][1],
                self[0][2] - rhs[0][2],
            ],
            [
                self[1][0] - rhs[1][0],
                self[1][1] - rhs[1][1],
                self[1][2] - rhs[1][2],
            ],
            [
                self[2][0] - rhs[2][0],
                self[2][1] - rhs[2][1],
                self[2][2] - rhs[2][2],
            ],
        ])
    }
}

impl<T> Mul for M3<T>
where
    T: Numeric,
{
    type Output = M3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let (c1, c2, c3) = self.into_rows();
        let (r1, r2, r3) = rhs.into_cols();
        M3([
            [c1.dot(r1), c1.dot(r2), c1.dot(r3)],
            [c2.dot(r1), c2.dot(r2), c2.dot(r3)],
            [c3.dot(r1), c3.dot(r2), c3.dot(r3)],
        ])
    }
}

impl<T> Mul<V3<T>> for M3<T>
where
    T: Numeric,
{
    type Output = V3<T>;

    fn mul(self, rhs: V3<T>) -> Self::Output {
        let (c1, c2, c3) = self.into_rows();
        V3([c1.dot(rhs), c2.dot(rhs), c3.dot(rhs)])
    }
}

impl<T> Div<T> for M3<T>
where
    T: Numeric,
{
    type Output = M3<T>;

    fn div(self, rhs: T) -> Self::Output {
        M3([
            [self[0][0] / rhs, self[0][1] / rhs, self[0][2] / rhs],
            [self[1][0] / rhs, self[1][1] / rhs, self[1][2] / rhs],
            [self[2][0] / rhs, self[2][1] / rhs, self[2][2] / rhs],
        ])
    }
}
