// use crate::geometry::Vector3;
use super::*;

#[test]
fn vector3_index_operator() {
    let vec3: Vector3<i32> = Vector3::new(1, 2, 3);

    assert_eq!(vec3.x, vec3[0]);
    assert_eq!(vec3.y, vec3[1]);
    assert_eq!(vec3.z, vec3[2]);
}
