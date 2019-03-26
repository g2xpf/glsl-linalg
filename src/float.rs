use crate::numeric::Numeric;

pub trait Float: Numeric {
    fn f_sqrt(self) -> Self;
}

impl Float for f32 {
    fn f_sqrt(self) -> f32 {
        self.sqrt()
    }
}

impl Float for f64 {
    fn f_sqrt(self) -> f64 {
        self.sqrt()
    }
}
