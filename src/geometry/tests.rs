// use crate::geometry::Vector3;
use super::*;

#[test]
fn vector3_add_operator() {
    let vec3_1: Vector3<i32> = Vector3::new(1, 2, 3);
    let vec3_2: Vector3<i32> = Vector3::new(3, 2, 1);
    let expected: Vector3<i32> = Vector3::new(4, 4, 4);
    let got = vec3_1 + vec3_2;

    assert_eq!(expected.x, got.x);
    assert_eq!(expected.y, got.y);
    assert_eq!(expected.z, got.z);
}

#[test]
fn vector3_sub_operator() {
    let vec3_1: Vector3<i32> = Vector3::new(1, 2, 3);
    let vec3_2: Vector3<i32> = Vector3::new(3, 2, 1);
    let expected: Vector3<i32> = Vector3::new(-2, 0, 2);
    let got = vec3_1 - vec3_2;

    assert_eq!(expected.x, got.x);
    assert_eq!(expected.y, got.y);
    assert_eq!(expected.z, got.z);
}

#[test]
fn vector3_eq_operator() {
    let vec3_1: Vector3<i32> = Vector3::new(1, 2, 3);
    let vec3_2: Vector3<i32> = Vector3::new(3, 2, 1);
    let expected: Vector3<i32> = Vector3::new(4, 4, 4);
    let got = vec3_1 + vec3_2;
    assert_eq!(expected, got)
}

#[test]
fn vector2_mul_by_scalar() {
    let vec3: Vector3<i32> = Vector3::new(1, 2, 3);
    let expected = Vector3::new(2, 4, 6);
    let got = vec3 * 2;
    assert_eq!(expected, got);
}

#[test]
fn vector3_index_operator() {
    let vec3: Vector3<i32> = Vector3::new(1, 2, 3);

    assert_eq!(vec3.x, vec3[0]);
    assert_eq!(vec3.y, vec3[1]);
    assert_eq!(vec3.z, vec3[2]);
}

#[test]
fn vector2_index_operator() {
    let vector2: Vector2<i32> = Vector2::new(1, 2);

    assert_eq!(vector2.x, vector2[0]);
    assert_eq!(vector2.y, vector2[1]);
}
