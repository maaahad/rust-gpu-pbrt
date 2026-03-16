// use crate::geometry::Vector3;
use super::*;

// Vector3
// [+, +=]
#[test]
fn vector3_add_operator() {
    let vector3_1: Vector3<i32> = Vector3::new(1, 2, 3);
    let vector3_2: Vector3<i32> = Vector3::new(3, 2, 1);
    let expected: Vector3<i32> = Vector3::new(4, 4, 4);
    let got = vector3_1 + vector3_2;

    assert_eq!(expected.x, got.x);
    assert_eq!(expected.y, got.y);
    assert_eq!(expected.z, got.z);
}

#[test]
fn vector3_add_assign_operator() {
    let mut vector3 = Vector3::new(1, 2, 3);
    vector3 += Vector3::new(1, 2, 3);

    let expected = Vector3::new(2, 4, 6);
    assert_eq!(expected, vector3);
}

// [-, -=]
#[test]
fn vector3_sub_operator() {
    let vector3_1: Vector3<i32> = Vector3::new(1, 2, 3);
    let vector3_2: Vector3<i32> = Vector3::new(3, 2, 1);
    let expected: Vector3<i32> = Vector3::new(-2, 0, 2);
    let got = vector3_1 - vector3_2;

    assert_eq!(expected.x, got.x);
    assert_eq!(expected.y, got.y);
    assert_eq!(expected.z, got.z);
}

#[test]
fn vector3_sub_assign_operator() {
    let mut vector3: Vector3<i32> = Vector3::new(1, 2, 3);
    vector3 -= Vector3::new(3, 2, 1);

    let expected = Vector3::new(-2, 0, 2);
    assert_eq!(expected, vector3);
}

// [*, *=]
#[test]
fn vector3_mul_by_scalar() {
    let vector3: Vector3<i32> = Vector3::new(1, 2, 3);
    let expected = Vector3::new(2, 4, 6);
    let got = vector3 * 2;
    assert_eq!(expected, got);
}

#[test]
fn vector3_mul_assign_by_scalar() {
    let mut vector3: Vector3<i32> = Vector3::new(1, 2, 3);
    let expected = Vector3::new(2, 4, 6);
    vector3 *= 2;

    assert_eq!(expected, vector3);
}

// [=]
#[test]
fn vector3_eq_operator() {
    let vector3_1: Vector3<i32> = Vector3::new(1, 2, 3);
    let vector3_2: Vector3<i32> = Vector3::new(3, 2, 1);
    let vec3_3: Vector3<i32> = Vector3::new(1, 2, 3);

    let equal_vector = vector3_1 == vec3_3;
    let expected_equal = true;
    assert_eq!(expected_equal, equal_vector);

    let not_equal_vector = vector3_1 == vector3_2;
    let expected_not_equal = false;
    assert_eq!(expected_not_equal, not_equal_vector)
}

// [[]]
#[test]
fn vector3_index_operator() {
    let vector3: Vector3<i32> = Vector3::new(1, 2, 3);

    assert_eq!(vector3.x, vector3[0]);
    assert_eq!(vector3.y, vector3[1]);
    assert_eq!(vector3.z, vector3[2]);
}

// Vector2
#[test]
fn vector2_index_operator() {
    let vector2: Vector2<i32> = Vector2::new(1, 2);

    assert_eq!(vector2.x, vector2[0]);
    assert_eq!(vector2.y, vector2[1]);
}
