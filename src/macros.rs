pub mod converter;
pub mod finalizer;
pub mod types;

#[macro_export]
macro_rules! vec2 {
    ($($e: expr),+) => {
        {
            use $crate::macros::types;
            use $crate::macros::converter::Converter;
            use $crate::macros::finalizer::Finalizer;

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e);
            )+
            let f: $crate::vector::V2<_> = e.finalize();
            f
        }
    }
}

#[macro_export]
macro_rules! vec3 { ($($e: expr), +) => {
        {
            use $crate::macros::types;
            use $crate::macros::converter::Converter;
            use $crate::macros::finalizer::Finalizer;

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e);
            )+
            let f: $crate::vector::V3<_> = e.finalize();
            f
        }
    }
}

#[macro_export]
macro_rules! vec4 {
    ($($e: expr), +) => {
        {
            use $crate::macros::types; use $crate::macros::converter::Converter;
            use $crate::macros::finalizer::Finalizer;

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e);
            )+
            let f: $crate::vector::V4<_> = e.finalize();
            f
        }
    }
}

#[macro_export]
macro_rules! mat2 {
    ($e: expr) => {
        $crate::matrix::M2([[$e; 2]; 2])
    };
    ($e1: expr, $e2: expr, $e3: expr, $e4: expr) => {
        $crate::matrix::M2([[$e1, $e2], [$e3, $e4]])
    };
    ($($e1: expr),+; $($e2: expr),+) => {
        {
            use $crate::macros::types;
            use $crate::macros::converter::Converter;
            use $crate::macros::finalizer::Finalizer;
            use $crate::matrix::{self, FromVectors};

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e1);
            )+
            let v1: $crate::vector::V2<_> = e.finalize();

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e2);
            )+
            let v2: $crate::vector::V2<_> = e.finalize();

            matrix::M2::from_rows((v1, v2))
        }
    };
}

#[macro_export]
macro_rules! mat3 {
    ($e: expr) => {
        $crate::matrix::M3([[$e; 3]; 3])
    };
    ($e1: expr, $e2: expr, $e3: expr, $e4: expr, $e5: expr, $e6: expr, $e7: expr, $e8: expr, $e9: expr) => {
        $crate::matrix::M3([[$e1, $e2, $e3], [$e4, $e5, $e6], [$e7, $e8, $e9]])
    };
    ($($e1: expr),+; $($e2: expr),+; $($e3: expr),+) => {
        {
            use $crate::macros::types;
            use $crate::macros::converter::Converter;
            use $crate::macros::finalizer::Finalizer;
            use $crate::matrix::{self, FromVectors};

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e1);
            )+
            let v1: $crate::vector::V3<_> = e.finalize();

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e2);
            )+
            let v2: $crate::vector::V3<_> = e.finalize();

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e3);
            )+
            let v3: $crate::vector::V3<_> = e.finalize();

            matrix::M3::from_rows((v1, v2, v3))
        }
    };
}

#[macro_export]
macro_rules! mat4 {
    ($e: expr) => {
        $crate::matrix::M4([[$e; 4]; 4])
    };
    ($e1: expr, $e2: expr, $e3: expr, $e4: expr, $e5: expr, $e6: expr, $e7: expr, $e8: expr, $e9: expr, $e10: expr, $e11: expr, $e12: expr, $e13: expr, $e14: expr, $e15: expr, $e16: expr) => {
        $crate::matrix::M4([[$e1, $e2, $e3, $e4], [$e5, $e6, $e7, $e8], [$e9, $e10, $e11, $e12], [$e13, $e14, $e15, $e16]])
    };
    ($($e1: expr),+; $($e2: expr),+; $($e3: expr),+; $($e4: expr),+) => {
        {
            use $crate::macros::types;
            use $crate::macros::converter::Converter;
            use $crate::macros::finalizer::Finalizer;
            use $crate::matrix::{self, FromVectors};

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e1);
            )+
            let v1: $crate::vector::V4<_> = e.finalize();

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e2);
            )+
            let v2: $crate::vector::V4<_> = e.finalize();

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e3);
            )+
            let v3: $crate::vector::V4<_> = e.finalize();

            let e = types::EmptyConverter;
            $(
                let e = e.convert($e4);
            )+
            let v4: $crate::vector::V4<_> = e.finalize();

            matrix::M4::from_rows((v1, v2, v3, v4))
        }
    };
}
