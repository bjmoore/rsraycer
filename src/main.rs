use crate::color::Color;
use std::fmt::Write;
use std::fs;

mod color;
mod vec3;

const OUT_PATH: &str = "out.ppm";

fn main() {
    let mut ppm = String::new();

    let img_width = 256;
    let img_height = 256;

    writeln!(ppm, "P3"); // magic number
    writeln!(ppm, "{} {}", img_width, img_height); // width <SP> height
    writeln!(ppm, "255"); // maxval
    // pixel array
    for i in 0..img_width {
        for j in 0..img_height {
            let color = Color::new(
                i as f64 / img_width as f64,
                j as f64 / img_height as f64,
                0.0,
            );
            writeln!(ppm, "{}", color);
        }
    }

    fs::write(OUT_PATH, ppm);
}
