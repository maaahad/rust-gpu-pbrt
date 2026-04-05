use super::geometry::Color;

fn write_color(pixel_color: &Color) {
    let Color { x: r, y: g, z: b } = pixel_color;

    // Translate [0,1] component values to byte range [0, 255]
    let r_byte: i32 = (255.999 * r) as i32;
    let g_byte: i32 = (255.999 * g) as i32;
    let b_byte: i32 = (255.999 * b) as i32;

    println!("{r_byte} {g_byte} {b_byte}")
}
pub fn render(width: u32, height: u32) {
    println!("P3");
    println!("# The P3 means the color are in ASCII, then 3 columns and 2 rows");
    println!("# then 255 for max color, then RGB color triplets");
    println!("{width} {height}");
    println!("255");

    for row in 0..height {
        for col in 0..width {
            write_color(&Color {
                x: col as f64 / width as f64,
                y: row as f64 / height as f64,
                z: 0.0,
            });
        }
    }
}
