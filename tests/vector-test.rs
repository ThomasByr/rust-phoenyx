use math_vector::Vector;

use std::f64::consts::PI;

#[test]
fn new() {
    let v = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn copy() {
    let v = Vector::new(1.0, 2.0, 3.0);
    let mut v_copy = v;
    assert_eq!(v, v_copy);

    v_copy.x = 0.0;
    assert_ne!(v, v_copy);
}

#[test]
fn clone() {
    let v = Vector::new(1.0, 2.0, 3.0);
    let mut v_clone = v.clone();
    assert_eq!(v, v_clone);

    v_clone.x = 0.0;
    assert_ne!(v, v_clone);
}

#[test]
fn add() {
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let z = Vector::new(0.0, 0.0, 1.0);

    let v = x + y + z;
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);

    let w = &(&x + &y) + &z;
    assert_eq!(v, w);
}

#[test]
fn add_assign() {
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let z = Vector::new(0.0, 0.0, 1.0);

    let mut v = Vector::default();
    v += x + y + z;
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 1.0);
    assert_eq!(v.z, 1.0);
}

#[test]
fn sub() {
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let z = Vector::new(0.0, 0.0, 1.0);

    let v = -x - y - z;
    assert_eq!(v.x, -1.0);
    assert_eq!(v.y, -1.0);
    assert_eq!(v.z, -1.0);

    let w = &(&-x - &y) - &z;
    assert_eq!(v, w);
}

#[test]
fn sub_assign() {
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let z = Vector::new(0.0, 0.0, 1.0);

    let mut v = Vector::default();
    v -= x + y + z;
    assert_eq!(v.x, -1.0);
    assert_eq!(v.y, -1.0);
    assert_eq!(v.z, -1.0);
}

#[test]
fn mul() {
    let x = Vector::new(1.0, 2.0, 3.0);
    let y = Vector::new(2.0, 3.0, 4.0);
    let z = Vector::new(4.0, 5.0, 6.0);

    let v = x * y * z;
    assert_eq!(v.x, 8.0);
    assert_eq!(v.y, 30.0);
    assert_eq!(v.z, 72.0);

    let w = &(&x * &y) * &z;
    assert_eq!(v, w);
}

#[test]
fn mul_assign() {
    let x = Vector::new(1.0, 2.0, 3.0);
    let y = Vector::new(2.0, 3.0, 4.0);
    let z = Vector::new(4.0, 5.0, 6.0);

    let mut v = Vector::one();
    v *= x * y * z;
    assert_eq!(v.x, 8.0);
    assert_eq!(v.y, 30.0);
    assert_eq!(v.z, 72.0);
}

#[test]
fn div() {
    let x = Vector::new(1.0, 2.0, 3.0);
    let y = Vector::new(2.0, 3.0, 4.0);
    let z = Vector::new(4.0, 5.0, 6.0);

    let v = x / y / z;
    assert_eq!(v.x, 1.0 / 8.0);
    assert_eq!(v.y, 2.0 / 15.0);
    assert_eq!(v.z, 3.0 / 24.0);

    let w = &(&x / &y) / &z;
    assert_eq!(v, w);
}

#[test]
fn div_assign() {
    let x = Vector::new(1.0, 2.0, 3.0);
    let y = Vector::new(2.0, 3.0, 4.0);
    let z = Vector::new(4.0, 5.0, 6.0);

    let mut v = Vector::one();
    v /= (y * z) / x;
    assert_eq!(v.x, 1.0 / 8.0);
    assert_eq!(v.y, 2.0 / 15.0);
    assert_eq!(v.z, 3.0 / 24.0);
}

#[test]
fn dot() {
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let z = Vector::new(0.0, 0.0, 1.0);

    assert_eq!(x.dot(y), 0.0);
    assert_eq!(x.dot(z), 0.0);
    assert_eq!(y.dot(z), 0.0);
}

#[test]
fn cross() {
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let z = Vector::new(0.0, 0.0, 1.0);

    assert_eq!(x.cross(y), z);
}

#[test]
fn is_close() {
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(1.0000000001, 0.0, 0.0);
    assert_eq!(y.is_close(x), true);

    let x = Vector::new(1000000000.0, 0.0, 0.0);
    let y = Vector::new(1000000000.1, 0.0, 0.0);
    assert_eq!(x.is_close(y), true);

    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(1.0001, 0.0, 0.0);
    assert_eq!(x.is_close(y), false);
}

#[test]
fn rotate() {
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let z = Vector::new(0.0, 0.0, 1.0);

    assert_eq!(x.rotated(PI / 2f64, z).is_close(y), true);

    let mut v = x;
    v.rotate(PI / 2f64, z);
    assert_eq!(v.is_close(y), true);
    assert_eq!(v.is_close(x), false);
}

#[test]
fn polars() {
    let reference = Vector::new(1.0, 1.0, 1.0).normalized();
    let (theta, phi) = reference.heading3d();
    let v = Vector::from_polar(theta, phi);
    assert_eq!(v.is_close(reference), true);

    let reference = Vector::new(1.0, 1.0, 0.0).normalized();
    let angle = reference.heading2d();
    let v = Vector::from_angle(angle);
    assert_eq!(v.is_close(reference), true);
}
