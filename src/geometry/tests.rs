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

// [/, /=]
#[test]
fn vector3_div_by_scalar() {
    let vector3 = Vector3::new(2, 4, 6);
    let expected = Vector3::new(1, 2, 3);

    let got = vector3 / 2;
    assert_eq!(expected, got);
}

#[test]
fn vector3_div_assign_by_scalar() {
    let mut vector3 = Vector3::new(2, 4, 6);
    let expected = Vector3::new(1, 2, 3);
    vector3 /= 2;

    assert_eq!(expected, vector3);
}

// [negation]
#[test]
fn vector3_neg_operator() {
    let vector3 = Vector3::new(1, 2, 3);
    let expected = Vector3::new(-1, -2, -3);
    let got = -vector3;
    assert_eq!(expected, got);
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

// [abs]
#[test]
fn vector3_abs() {
    let mut vector3 = Vector3::new(-1, -2, -3);
    vector3.abs();

    assert_eq!(vector3, Vector3::new(1, 2, 3));
}

#[test]
fn vector3_length_f64() {
    let vector3 = Vector3::new(0.0, 3.0, 4.0);
    assert_eq!(vector3.length(), 5.0);
}

#[test]
fn vector3_length_i64() {
    let vector3 = Vector3::new(0, 3, 4);
    assert_eq!(vector3.length(), 5);
}

#[test]
fn vector3_dot() {
    let vector3_1i = Vector3::new(1, 2, 3);
    let vector3_2i = Vector3::new(4, 5, 6);
    let expected_i: i32 = 4 + 10 + 18;

    assert_eq!(expected_i, vector3_1i.dot(&vector3_2i));

    let vector3_1f = Vector3::new(1.0, 2.0, 3.0);
    let vector3_2f = Vector3::new(4.0, 5.0, 6.0);
    let expected_f = 4.0 + 10.0 + 18.0;

    assert_eq!(expected_f, vector3_1f.dot(&vector3_2f));
}

#[test]
fn vector3_cross() {
    let vector3_1i = Vector3::new(1, 2, 3);
    let vector3_2i = Vector3::new(4, 5, 6);

    // (12 - 15)i, (12 - 6)j, (5 - 8)k;

    let expected_i = Vector3::new(-3, 6, -3);

    assert_eq!(expected_i, vector3_1i.cross(&vector3_2i));

    let vector3_1f = Vector3::new(1.0, 2.0, 3.0);
    let vector3_2f = Vector3::new(4.0, 5.0, 6.0);
    let expected_f = Vector3::new(-3.0, 6.0, -3.0);

    assert_eq!(expected_f, vector3_1f.cross(&vector3_2f));
}

// Vector2
#[test]
fn vector2_index_operator() {
    let vector2: Vector2<i32> = Vector2::new(1, 2);

    assert_eq!(vector2.x, vector2[0]);
    assert_eq!(vector2.y, vector2[1]);
}
