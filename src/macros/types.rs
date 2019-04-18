use crate::numeric::Numeric;
use std::ops::Deref;

pub struct V1<T>(pub T);
pub struct EmptyConverter;

impl<T> Deref for V1<T>
where
    T: Numeric,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
