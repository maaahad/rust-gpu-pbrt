use super::*;

#[test]
fn ray_at() {
    let ray = Ray::new(
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    );

    assert_eq!(
        ray.at(1.0),
        Vector3 {
            x: 2.0,
            y: 3.0,
            z: 4.0
        }
    );

    assert_eq!(
        ray.at(2.0),
        Vector3 {
            x: 3.0,
            y: 5.0,
            z: 7.0
        }
    );

    assert_eq!(
        ray.at(-2.0),
        Vector3 {
            x: -1.0,
            y: -3.0,
            z: -5.0
        }
    );
}
