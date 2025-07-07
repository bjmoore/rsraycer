use crate::color::Color;
use crate::color::color_pixel;
use std::fs;

mod color;
mod vec3;

const OUT_PATH: &str = "out.ppm";

fn main() {
    let mut ppm = String::new();

    let img_width = 256;
    let img_height = 256;

    ppm.push_str("P3\n"); // magic number
    ppm.push_str(&format!("{} {}\n", img_width, img_height)); // width <SP> height
    ppm.push_str("255\n"); // maxval
    // pixel array
    for i in 0..img_width {
        for j in 0..img_height {
            let color = Color::new(
                i as f64 / img_width as f64,
                j as f64 / img_height as f64,
                0.0,
            );
            let color = color_pixel(color);
            ppm.push_str(&format!("{}\n", color));
        }
    }

    fs::write(OUT_PATH, ppm);
}
