#[macro_use]
extern crate glsl_linalg;

use glsl_linalg::{FloatMatrix, Matrix};

#[test]
fn m_div_f() {
    let m = mat4!(0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30);
    assert_eq!(
        mat4!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
        m / 2
    )
}

#[test]
fn m_add_m() {
    let m1 = mat4!(vec4!(1, 2, -1, 2); vec4!(0, 1, 0, 2); vec4!(2, 3, 4, 0); vec4!(1, 2, 0, 3));
    let m2 = mat4!(vec4!(1, 2, 3, 0); vec4!(0, 1, 4, 3); vec4!(1, 2, 4, 0); vec4!(-1, -3, 3, 2));

    assert_eq!(
        mat4!(vec4!(2, 4, 2, 2); vec4!(0, 2, 4, 5); vec4!(3, 5, 8, 0); vec4!(0, -1, 3, 5)),
        m1 + m2
    );
}

#[test]
fn m_sub_m() {
    let m1 = mat4!(vec4!(1, 2, -1, 2); vec4!(0, 1, 0, 2); vec4!(2, 3, 4, 0); vec4!(1, 2, 0, 3));
    let m2 = mat4!(vec4!(1, 2, 3, 0); vec4!(0, 1, 4, 3); vec4!(1, 2, 4, 0); vec4!(-1, -3, 3, 2));

    assert_eq!(
        mat4!(vec4!(0, 0, -4, 2); vec4!(0, 0, -4, -1); vec4!(1, 1, 0, 0); vec4!(2, 5, -3, 1)),
        m1 - m2
    );
}

#[test]
fn m_mul_m() {
    let m1 = mat2!(1, 2, 3, 4);
    let m2 = mat2!(-2, 1, 3, 1);

    assert_eq!(mat2!(4, 3, 6, 7), m1 * m2);
}

#[test]
fn m_mul_v() {
    let m = mat3!(1, 2, 3, 0, 1, 2, 0, 3, 4);
    let v = vec3!(1, 2, 3);

    assert_eq!(vec3!(14, 8, 18), m * v);
}

#[test]
fn m_transpose() {
    let mut m = mat4!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    m.transpose();

    assert_eq!(
        mat4!(0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15),
        m
    );
}

#[test]
fn m_inverse() {
    let m = mat4!(1.0, 0.0, 1.0, 2.0, 1.0, 3.0, 3.0, 4.0, 3.0, 2.0, 3.0, 5.0, 1.0, 2.0, 3.0, 4.0);

    assert_eq!(
        mat4!(
            -1.5, -1.0, 1.0, 0.5, 0.0, 1.0, 0.0, -1.0, -3.5, -3.0, 1.0, 3.5, 3.0, 2.0, -1.0, -2.0
        ),
        m.inverse()
    );

    let m = mat3!(2.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 2.0);

    assert_eq!(
        mat3!(0.0, 2.0, -1.0, -1.0, 4.0, -2.0, 0.0, -1.0, 1.0),
        m.inverse()
    );

    let m = mat2!(1.0, 2.0; 0.0, 1.0);

    assert_eq!(mat2!(1.0, -2.0, 0.0, 1.0), m.inverse());
}
