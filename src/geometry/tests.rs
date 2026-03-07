// use crate::geometry::Vector3;
use super::*;

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
