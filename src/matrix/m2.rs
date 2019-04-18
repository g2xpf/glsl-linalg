use crate::float::Float;
use crate::matrix::{FloatMatrix, FromVectors, IntoVectors, Matrix, M2};
use crate::numeric::Numeric;
use crate::vector::{Vector, V2};
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

impl<T> Deref for M2<T>
where
    T: Numeric,
{
    type Target = [[T; 2]; 2];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for M2<T>
where
    T: Numeric,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoVectors<(V2<T>, V2<T>)> for M2<T>
where
    T: Numeric,
{
    #[inline]
    fn into_cols(&self) -> (V2<T>, V2<T>) {
        (V2([self[0][0], self[1][0]]), V2([self[0][1], self[1][1]]))
    }

    #[inline]
    fn into_rows(&self) -> (V2<T>, V2<T>) {
        (V2([self[0][0], self[0][1]]), V2([self[1][0], self[1][1]]))
    }
}

impl<T> FromVectors<(V2<T>, V2<T>)> for M2<T>
where
    T: Numeric,
{
    #[inline]
    fn from_cols(v: (V2<T>, V2<T>)) -> Self {
        let (r1, r2) = v;
        M2([[r1[0], r2[0]], [r1[1], r2[1]]])
    }

    #[inline]
    fn from_rows(v: (V2<T>, V2<T>)) -> Self {
        let (V2(c1), V2(c2)) = v;
        M2([c1, c2])
    }
}

impl<T> Matrix for M2<T>
where
    T: Numeric,
{
    #[inline]
    fn transpose(&mut self) {
        unsafe {
            (&mut self[0][1] as *mut T).swap(&mut self[1][0]);
        }
    }
}

impl<F> FloatMatrix<F> for M2<F>
where
    F: Float,
{
    #[inline]
    fn determinant(&self) -> F {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    #[inline]
    fn cofactor(&self) -> Self {
        M2([[self[1][1], -self[0][1]], [-self[1][0], self[0][0]]])
    }
}

impl<T> Add for M2<T>
where
    T: Numeric,
{
    type Output = M2<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        M2([
            [self[0][0] + rhs[0][0], self[0][1] + rhs[0][1]],
            [self[1][0] + rhs[1][0], self[1][1] + rhs[1][1]],
        ])
    }
}

impl<T> Sub for M2<T>
where
    T: Numeric,
{
    type Output = M2<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        M2([
            [self[0][0] - rhs[0][0], self[0][1] - rhs[0][1]],
            [self[1][0] - rhs[1][0], self[1][1] - rhs[1][1]],
        ])
    }
}

impl<T> Mul for M2<T>
where
    T: Numeric,
{
    type Output = M2<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let (c1, c2) = self.into_rows();
        let (r1, r2) = rhs.into_cols();
        M2([[c1.dot(r1), c1.dot(r2)], [c2.dot(r1), c2.dot(r2)]])
    }
}

impl<T> Mul<V2<T>> for M2<T>
where
    T: Numeric,
{
    type Output = V2<T>;

    #[inline]
    fn mul(self, rhs: V2<T>) -> Self::Output {
        let (c1, c2) = self.into_rows();
        V2([c1.dot(rhs), c2.dot(rhs)])
    }
}

impl<T> Div<T> for M2<T>
where
    T: Numeric,
{
    type Output = M2<T>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        M2([
            [self[0][0] / rhs, self[0][1] / rhs],
            [self[1][0] / rhs, self[1][1] / rhs],
        ])
    }
}
