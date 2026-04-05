use pbrt::{geometry::Vector3, image, ray::Ray};

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

    render(&config);
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

fn hit_sphere(center: &Vector3<f64>, radius: f64, ray: &Ray) -> bool {
    let oc = *center - ray.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = ray.direction.dot(&oc) * (-2.0);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}

fn ray_color(ray: &Ray) -> image::Color {
    if hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return image::Color {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);

    let start_color = image::Color {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };

    let end_color = image::Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    start_color * (1.0 - a) + end_color * a
}

fn render(config: &Config) {
    let Config {
        image_width: width,
        image_height: height,
        viewport_width,
        viewport_height,
    } = *config;

    // camera
    let focal_length = 1.0;
    let camera_center: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);

    // calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u: Vector3<f64> = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vector3<f64> = Vector3::new(0.0, viewport_height, 0.0);

    // calculate the horizontal and vertival delta vectors from pixel to pixel
    let pixel_delta_u: Vector3<f64> = viewport_u / width as f64;
    let pixel_delta_v: Vector3<f64> = viewport_v / height as f64;

    // calculate the location of the upper left pixel
    let viewport_upper_left: Vector3<f64> =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.50;

    println!("P3");
    println!("# The P3 means the color are in ASCII, then 3 columns and 2 rows");
    println!("# then 255 for max color, then RGB color triplets");
    println!("{width} {height}");
    println!("255");

    for row in 0..height {
        for col in 0..width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * col as f64) + (pixel_delta_v * row as f64);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);

            image::write_color(&ray_color(&ray));
        }
    }
}
