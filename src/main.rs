use pbrt::geometry::Vector3;
fn main() {
    let vec3: Vector3<i32> = Vector3::new(1, 2, 3);

    println!("{vec3:?}");
    // checking index operator
    println!("{}", vec3[0]);
    println!("{}", vec3[1]);
    println!("{}", vec3[2]);
}
