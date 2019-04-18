use super::types::V1;
use crate::matrix::{M2, M3, M4};
use crate::numeric::Numeric;
use crate::vector::{V2, V3, V4};

pub trait Finalizer<T> {
    fn finalize(self) -> T;
}

macro_rules! impl_identity_finalizer {
    ($tt: tt) => {
        impl<T> Finalizer<$tt<T>> for $tt<T> {
            #[inline]
            fn finalize(self) -> $tt<T> {
                self
            }
        }
    };
}

impl_identity_finalizer!(V2);
impl_identity_finalizer!(V3);
impl_identity_finalizer!(V4);
impl_identity_finalizer!(M2);
impl_identity_finalizer!(M3);
impl_identity_finalizer!(M4);

impl<T> Finalizer<V2<T>> for V1<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> V2<T> {
        V2([*self; 2])
    }
}

impl<T> Finalizer<V3<T>> for V1<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> V3<T> {
        V3([*self; 3])
    }
}

impl<T> Finalizer<V4<T>> for V1<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> V4<T> {
        V4([*self; 4])
    }
}

impl<T> Finalizer<M2<T>> for V1<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> M2<T> {
        M2([[*self; 2], [*self; 2]])
    }
}

impl<T> Finalizer<M3<T>> for V1<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> M3<T> {
        M3([[*self; 3], [*self; 3], [*self; 3]])
    }
}

impl<T> Finalizer<M4<T>> for V1<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> M4<T> {
        M4([[*self; 4], [*self; 4], [*self; 4], [*self; 4]])
    }
}

impl<T> Finalizer<V2<T>> for V3<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> V2<T> {
        let s = &self;
        V2([s[0], s[1]])
    }
}

impl<T> Finalizer<V2<T>> for V4<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> V2<T> {
        let s = &self;
        V2([s[0], s[1]])
    }
}

impl<T> Finalizer<V3<T>> for V4<T>
where
    T: Numeric,
{
    #[inline]
    fn finalize(self) -> V3<T> {
        let s = &self;
        V3([s[0], s[1], s[2]])
    }
}
