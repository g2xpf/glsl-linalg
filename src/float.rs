use crate::numeric::Numeric;

pub trait Float: Numeric {
    #[inline]
    fn f_sqrt(self) -> Self;
}

impl Float for f32 {
    #[inline]
    fn f_sqrt(self) -> f32 {
        self.sqrt()
    }
}

impl Float for f64 {
    #[inline]
    fn f_sqrt(self) -> f64 {
        self.sqrt()
    }
}
