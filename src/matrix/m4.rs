use crate::float::Float;
use crate::matrix::{FloatMatrix, FromVectors, IntoVectors, Matrix, M4};
use crate::numeric::Numeric;
use crate::vector::{Vector, V4};
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

impl<T> Deref for M4<T>
where
    T: Numeric,
{
    type Target = [[T; 4]; 4];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for M4<T>
where
    T: Numeric,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoVectors<(V4<T>, V4<T>, V4<T>, V4<T>)> for M4<T>
where
    T: Numeric,
{
    #[inline]
    fn into_cols(&self) -> (V4<T>, V4<T>, V4<T>, V4<T>) {
        (
            V4([self[0][0], self[1][0], self[2][0], self[3][0]]),
            V4([self[0][1], self[1][1], self[2][1], self[3][1]]),
            V4([self[0][2], self[1][2], self[2][2], self[3][2]]),
            V4([self[0][3], self[1][3], self[2][3], self[3][3]]),
        )
    }

    #[inline]
    fn into_rows(&self) -> (V4<T>, V4<T>, V4<T>, V4<T>) {
        (V4(self[0]), V4(self[1]), V4(self[2]), V4(self[3]))
    }
}

impl<T> FromVectors<(V4<T>, V4<T>, V4<T>, V4<T>)> for M4<T>
where
    T: Numeric,
{
    #[inline]
    fn from_cols(v: (V4<T>, V4<T>, V4<T>, V4<T>)) -> Self {
        let (r1, r2, r3, r4) = v;
        M4([
            [r1[0], r2[0], r3[0], r4[0]],
            [r1[1], r2[1], r3[1], r4[1]],
            [r1[2], r2[2], r3[2], r4[2]],
            [r1[3], r2[3], r3[3], r4[3]],
        ])
    }

    #[inline]
    fn from_rows(v: (V4<T>, V4<T>, V4<T>, V4<T>)) -> Self {
        let (V4(c1), V4(c2), V4(c3), V4(c4)) = v;
        M4([c1, c2, c3, c4])
    }
}

impl<T> Matrix for M4<T>
where
    T: Numeric,
{
    #[inline]
    fn transpose(&mut self) {
        unsafe {
            (&mut self[0][1] as *mut T).swap(&mut self[1][0]);
            (&mut self[0][2] as *mut T).swap(&mut self[2][0]);
            (&mut self[0][3] as *mut T).swap(&mut self[3][0]);
            (&mut self[1][2] as *mut T).swap(&mut self[2][1]);
            (&mut self[1][3] as *mut T).swap(&mut self[3][1]);
            (&mut self[2][3] as *mut T).swap(&mut self[3][2]);
        }
    }
}

impl<F> FloatMatrix<F> for M4<F>
where
    F: Float,
{
    #[inline]
    fn determinant(&self) -> F {
        self[0][3] * self[1][2] * self[2][1] * self[3][0]
            - self[0][2] * self[1][3] * self[2][1] * self[3][0]
            - self[0][3] * self[1][1] * self[2][2] * self[3][0]
            + self[0][1] * self[1][3] * self[2][2] * self[3][0]
            + self[0][2] * self[1][1] * self[2][3] * self[3][0]
            - self[0][1] * self[1][2] * self[2][3] * self[3][0]
            - self[0][3] * self[1][2] * self[2][0] * self[3][1]
            + self[0][2] * self[1][3] * self[2][0] * self[3][1]
            + self[0][3] * self[1][0] * self[2][2] * self[3][1]
            - self[0][0] * self[1][3] * self[2][2] * self[3][1]
            - self[0][2] * self[1][0] * self[2][3] * self[3][1]
            + self[0][0] * self[1][2] * self[2][3] * self[3][1]
            + self[0][3] * self[1][1] * self[2][0] * self[3][2]
            - self[0][1] * self[1][3] * self[2][0] * self[3][2]
            - self[0][3] * self[1][0] * self[2][1] * self[3][2]
            + self[0][0] * self[1][3] * self[2][1] * self[3][2]
            + self[0][1] * self[1][0] * self[2][3] * self[3][2]
            - self[0][0] * self[1][1] * self[2][3] * self[3][2]
            - self[0][2] * self[1][1] * self[2][0] * self[3][3]
            + self[0][1] * self[1][2] * self[2][0] * self[3][3]
            + self[0][2] * self[1][0] * self[2][1] * self[3][3]
            - self[0][0] * self[1][2] * self[2][1] * self[3][3]
            - self[0][1] * self[1][0] * self[2][2] * self[3][3]
            + self[0][0] * self[1][1] * self[2][2] * self[3][3]
    }

    fn cofactor(&self) -> Self {
        M4([
            [
                self[1][2] * self[2][3] * self[3][1] - self[1][3] * self[2][2] * self[3][1]
                    + self[1][3] * self[2][1] * self[3][2]
                    - self[1][1] * self[2][3] * self[3][2]
                    - self[1][2] * self[2][1] * self[3][3]
                    + self[1][1] * self[2][2] * self[3][3],
                self[0][3] * self[2][2] * self[3][1]
                    - self[0][2] * self[2][3] * self[3][1]
                    - self[0][3] * self[2][1] * self[3][2]
                    + self[0][1] * self[2][3] * self[3][2]
                    + self[0][2] * self[2][1] * self[3][3]
                    - self[0][1] * self[2][2] * self[3][3],
                self[0][2] * self[1][3] * self[3][1] - self[0][3] * self[1][2] * self[3][1]
                    + self[0][3] * self[1][1] * self[3][2]
                    - self[0][1] * self[1][3] * self[3][2]
                    - self[0][2] * self[1][1] * self[3][3]
                    + self[0][1] * self[1][2] * self[3][3],
                self[0][3] * self[1][2] * self[2][1]
                    - self[0][2] * self[1][3] * self[2][1]
                    - self[0][3] * self[1][1] * self[2][2]
                    + self[0][1] * self[1][3] * self[2][2]
                    + self[0][2] * self[1][1] * self[2][3]
                    - self[0][1] * self[1][2] * self[2][3],
            ],
            [
                self[1][3] * self[2][2] * self[3][0]
                    - self[1][2] * self[2][3] * self[3][0]
                    - self[1][3] * self[2][0] * self[3][2]
                    + self[1][0] * self[2][3] * self[3][2]
                    + self[1][2] * self[2][0] * self[3][3]
                    - self[1][0] * self[2][2] * self[3][3],
                self[0][2] * self[2][3] * self[3][0] - self[0][3] * self[2][2] * self[3][0]
                    + self[0][3] * self[2][0] * self[3][2]
                    - self[0][0] * self[2][3] * self[3][2]
                    - self[0][2] * self[2][0] * self[3][3]
                    + self[0][0] * self[2][2] * self[3][3],
                self[0][3] * self[1][2] * self[3][0]
                    - self[0][2] * self[1][3] * self[3][0]
                    - self[0][3] * self[1][0] * self[3][2]
                    + self[0][0] * self[1][3] * self[3][2]
                    + self[0][2] * self[1][0] * self[3][3]
                    - self[0][0] * self[1][2] * self[3][3],
                self[0][2] * self[1][3] * self[2][0] - self[0][3] * self[1][2] * self[2][0]
                    + self[0][3] * self[1][0] * self[2][2]
                    - self[0][0] * self[1][3] * self[2][2]
                    - self[0][2] * self[1][0] * self[2][3]
                    + self[0][0] * self[1][2] * self[2][3],
            ],
            [
                self[1][1] * self[2][3] * self[3][0] - self[1][3] * self[2][1] * self[3][0]
                    + self[1][3] * self[2][0] * self[3][1]
                    - self[1][0] * self[2][3] * self[3][1]
                    - self[1][1] * self[2][0] * self[3][3]
                    + self[1][0] * self[2][1] * self[3][3],
                self[0][3] * self[2][1] * self[3][0]
                    - self[0][1] * self[2][3] * self[3][0]
                    - self[0][3] * self[2][0] * self[3][1]
                    + self[0][0] * self[2][3] * self[3][1]
                    + self[0][1] * self[2][0] * self[3][3]
                    - self[0][0] * self[2][1] * self[3][3],
                self[0][1] * self[1][3] * self[3][0] - self[0][3] * self[1][1] * self[3][0]
                    + self[0][3] * self[1][0] * self[3][1]
                    - self[0][0] * self[1][3] * self[3][1]
                    - self[0][1] * self[1][0] * self[3][3]
                    + self[0][0] * self[1][1] * self[3][3],
                self[0][3] * self[1][1] * self[2][0]
                    - self[0][1] * self[1][3] * self[2][0]
                    - self[0][3] * self[1][0] * self[2][1]
                    + self[0][0] * self[1][3] * self[2][1]
                    + self[0][1] * self[1][0] * self[2][3]
                    - self[0][0] * self[1][1] * self[2][3],
            ],
            [
                self[1][2] * self[2][1] * self[3][0]
                    - self[1][1] * self[2][2] * self[3][0]
                    - self[1][2] * self[2][0] * self[3][1]
                    + self[1][0] * self[2][2] * self[3][1]
                    + self[1][1] * self[2][0] * self[3][2]
                    - self[1][0] * self[2][1] * self[3][2],
                self[0][1] * self[2][2] * self[3][0] - self[0][2] * self[2][1] * self[3][0]
                    + self[0][2] * self[2][0] * self[3][1]
                    - self[0][0] * self[2][2] * self[3][1]
                    - self[0][1] * self[2][0] * self[3][2]
                    + self[0][0] * self[2][1] * self[3][2],
                self[0][2] * self[1][1] * self[3][0]
                    - self[0][1] * self[1][2] * self[3][0]
                    - self[0][2] * self[1][0] * self[3][1]
                    + self[0][0] * self[1][2] * self[3][1]
                    + self[0][1] * self[1][0] * self[3][2]
                    - self[0][0] * self[1][1] * self[3][2],
                self[0][1] * self[1][2] * self[2][0] - self[0][2] * self[1][1] * self[2][0]
                    + self[0][2] * self[1][0] * self[2][1]
                    - self[0][0] * self[1][2] * self[2][1]
                    - self[0][1] * self[1][0] * self[2][2]
                    + self[0][0] * self[1][1] * self[2][2],
            ],
        ])
    }
}

impl<T> Add for M4<T>
where
    T: Numeric,
{
    type Output = M4<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        M4([
            [
                self[0][0] + rhs[0][0],
                self[0][1] + rhs[0][1],
                self[0][2] + rhs[0][2],
                self[0][3] + rhs[0][3],
            ],
            [
                self[1][0] + rhs[1][0],
                self[1][1] + rhs[1][1],
                self[1][2] + rhs[1][2],
                self[1][3] + rhs[1][3],
            ],
            [
                self[2][0] + rhs[2][0],
                self[2][1] + rhs[2][1],
                self[2][2] + rhs[2][2],
                self[2][3] + rhs[2][3],
            ],
            [
                self[3][0] + rhs[3][0],
                self[3][1] + rhs[3][1],
                self[3][2] + rhs[3][2],
                self[3][3] + rhs[3][3],
            ],
        ])
    }
}

impl<T> Sub for M4<T>
where
    T: Numeric,
{
    type Output = M4<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        M4([
            [
                self[0][0] - rhs[0][0],
                self[0][1] - rhs[0][1],
                self[0][2] - rhs[0][2],
                self[0][3] - rhs[0][3],
            ],
            [
                self[1][0] - rhs[1][0],
                self[1][1] - rhs[1][1],
                self[1][2] - rhs[1][2],
                self[1][3] - rhs[1][3],
            ],
            [
                self[2][0] - rhs[2][0],
                self[2][1] - rhs[2][1],
                self[2][2] - rhs[2][2],
                self[2][3] - rhs[2][3],
            ],
            [
                self[3][0] - rhs[3][0],
                self[3][1] - rhs[3][1],
                self[3][2] - rhs[3][2],
                self[3][3] - rhs[3][3],
            ],
        ])
    }
}

impl<T> Mul for M4<T>
where
    T: Numeric,
{
    type Output = M4<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let (c1, c2, c3, c4) = self.into_rows();
        let (r1, r2, r3, r4) = rhs.into_cols();
        M4([
            [c1.dot(r1), c1.dot(r2), c1.dot(r3), c1.dot(r4)],
            [c2.dot(r1), c2.dot(r2), c2.dot(r3), c2.dot(r4)],
            [c3.dot(r1), c3.dot(r2), c3.dot(r3), c3.dot(r4)],
            [c4.dot(r1), c4.dot(r2), c4.dot(r3), c4.dot(r4)],
        ])
    }
}

impl<T> Mul<V4<T>> for M4<T>
where
    T: Numeric,
{
    type Output = V4<T>;

    #[inline]
    fn mul(self, rhs: V4<T>) -> Self::Output {
        let (c1, c2, c3, c4) = self.into_rows();
        V4([c1.dot(rhs), c2.dot(rhs), c3.dot(rhs), c4.dot(rhs)])
    }
}

impl<T> Div<T> for M4<T>
where
    T: Numeric,
{
    type Output = M4<T>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        M4([
            [
                self[0][0] / rhs,
                self[0][1] / rhs,
                self[0][2] / rhs,
                self[0][3] / rhs,
            ],
            [
                self[1][0] / rhs,
                self[1][1] / rhs,
                self[1][2] / rhs,
                self[1][3] / rhs,
            ],
            [
                self[2][0] / rhs,
                self[2][1] / rhs,
                self[2][2] / rhs,
                self[2][3] / rhs,
            ],
            [
                self[3][0] / rhs,
                self[3][1] / rhs,
                self[3][2] / rhs,
                self[3][3] / rhs,
            ],
        ])
    }
}
