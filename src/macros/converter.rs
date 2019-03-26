use super::types::{EmptyConverter, V1};
use crate::numeric::Numeric;
use crate::vector::{V2, V3, V4};
use crate::matrix::{M2, M3, M4};

pub trait Converter<F, T> {
    fn convert(self, f: F) -> T;
}

impl<F> Converter<F, V1<F>> for EmptyConverter
where
    F: Numeric,
{
    fn convert(self, f: F) -> V1<F> {
        V1(f)
    }
}

macro_rules! impl_identity_converter {
    ($tt: tt) => {
        impl<F> Converter<$tt<F>, $tt<F>> for EmptyConverter
            where
                F: Numeric,
            {
                fn convert(self, f: $tt<F>) -> $tt<F> {
                    f
                }
            }
    };
}

impl_identity_converter!(V2);
impl_identity_converter!(V3);
impl_identity_converter!(V4);
impl_identity_converter!(M2);
impl_identity_converter!(M3);
impl_identity_converter!(M4);


impl<F> Converter<F, V2<F>> for V1<F>
where
    F: Numeric,
{
    fn convert(self, f: F) -> V2<F> {
        V2([*self, f])
    }
}

impl<F> Converter<V2<F>, V3<F>> for V1<F>
where
    F: Numeric,
{
    fn convert(self, f: V2<F>) -> V3<F> {
        let s = &f;
        V3([*self, s[0], s[1]])
    }
}

impl<F> Converter<V3<F>, V4<F>> for V1<F>
where
    F: Numeric,
{
    fn convert(self, f: V3<F>) -> V4<F> {
        let s = &f;
        V4([*self, s[0], s[1], s[2]])
    }
}

impl<F> Converter<F, V3<F>> for V2<F>
where
    F: Numeric,
{
    fn convert(self, f: F) -> V3<F> {
        let s = &self;
        V3([s[0], s[1], f])
    }
}

impl<F> Converter<V2<F>, V4<F>> for V2<F>
where
    F: Numeric,
{
    fn convert(self, f: V2<F>) -> V4<F> {
        let s = &self;
        let t = &f;
        V4([s[0], s[1], t[0], t[1]])
    }
}

impl<F> Converter<F, V4<F>> for V3<F>
where
    F: Numeric,
{
    fn convert(self, f: F) -> V4<F> {
        let s = &self;
        V4([s[0], s[1], s[2], f])
    }
}
