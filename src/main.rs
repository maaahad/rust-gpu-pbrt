use pbrt::image;

// TODO: this shuold come from user input or config file
const WIDTH: u32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;

#[derive(Debug)]
struct Config {
    image_width: u32,
    image_height: u32,
    viewport_height: f64,
    viewport_width: f64,
}

fn main() {
    let config = get_config();

    image::render(config.image_width, config.image_height);
}

fn get_config() -> Config {
    let image_width = WIDTH;
    let height: u32 = (image_width as f64 / ASPECT_RATIO).round() as u32;
    let image_height = if height > 0 { height } else { 1 };

    let viewport_height = VIEWPORT_HEIGHT;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    Config {
        image_width,
        image_height,
        viewport_width,
        viewport_height,
    }
}
