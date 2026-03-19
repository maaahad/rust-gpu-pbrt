pub fn render(width: u32, height: u32) {
    println!("P3");
    println!("# The P3 means the color are in ASCII, then 3 columns and 2 rows");
    println!("# then 255 for max color, then RGB color triplets");
    println!("{width} {height}");
    println!("255");

    for _row in 0..width {
        for _col in 0..height {
            println!("255 0 255");
        }
    }
}

