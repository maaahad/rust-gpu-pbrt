pub use super::geometry::Color;

pub fn write_color(pixel_color: &Color) {
    let Color { x: r, y: g, z: b } = pixel_color;

    // Translate [0,1] component values to byte range [0, 255]
    let r_byte: i32 = (255.999 * r) as i32;
    let g_byte: i32 = (255.999 * g) as i32;
    let b_byte: i32 = (255.999 * b) as i32;

    println!("{r_byte} {g_byte} {b_byte}")
}
