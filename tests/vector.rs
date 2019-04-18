#[macro_use]
extern crate glsl_linalg;

use glsl_linalg::{FloatVector, Vector};

#[test]
fn v_mul_f() {
    let f = 2.0;
    let v = vec4!(3.0);

    assert_eq!(vec4!(6.0), v * f);
}

#[test]
fn v_div_f() {
    let f = 4.0;
    let v = vec4!(8.0);

    assert_eq!(vec4!(2.0), v / f);
}

#[test]
fn v_add_v() {
    let v1 = vec3!(1.0, 2.0, 3.0);
    let v2 = vec3!(-1.0, -2.0, -3.0);

    assert_eq!(vec3!(0.0), v1 + v2);
}

#[test]
fn v_sub_v() {
    let v1 = vec3!(2.0);
    let v2 = vec3!(2.0);

    assert_eq!(vec3!(0.0), v1 - v2);
}

#[test]
fn v_mul_v() {
    let v1 = vec3!(2.0, 3.0, 4.0);
    let v2 = vec3!(-2.0, 0.0, -1.0);

    assert_eq!(vec3!(-4.0, 0.0, -4.0), v1 * v2);
}

#[test]
fn v_div_v() {
    let v1 = vec3!(6.0, 4.0, 3.0);
    let v2 = vec3!(2.0, 2.0, 3.0);

    assert_eq!(vec3!(3.0, 2.0, 1.0), v1 / v2);
}

#[test]
fn v_mul_m() {
    let v = vec3!(1.0, 2.0, 3.0);
    let m = mat3!(2.0, 3.0, 4.0, 5.0, 1.0, 2.0, 1.0, 3.0, 2.0);

    assert_eq!(vec3!(15.0, 14.0, 14.0), v * m);
}

#[test]
fn v_dot() {
    let v1 = vec3!(1.0, 2.0, 3.0);
    let v2 = vec3!(2.0, 3.0, 4.0);

    assert_eq!(20.0, v1.dot(v2));
}

#[test]
fn v_length() {
    let v = vec2!(3.0, 4.0);

    assert_eq!(5.0, v.length());
}

#[test]
fn v_normalize() {
    let v = vec2!(3.0, 4.0);

    assert_eq!(vec2!(0.6, 0.8), v.normalize());
}

#[test]
fn v_distance() {
    let v1 = vec2!(4.0, 6.0);
    let v2 = vec2!(1.0, 2.0);

    assert_eq!(5.0, v1.distance(&v2));
}
