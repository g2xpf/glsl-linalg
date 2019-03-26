#[macro_use]
extern crate glsl_linalg;

#[test]
fn f_mul_v() {
    let f = 2.0;
    let v = vec4!(3.0);

    assert_eq!(vec4!(6.0), f * v);
}

#[test]
fn f_mul_m() {
    let f = 3.0;
    let m = mat4!(2.0);

    assert_eq!(mat4!(6.0), f * m);
}
