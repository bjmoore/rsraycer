use std::fs;

mod vec3;

const OUT_PATH: &str = "out.ppm";

fn main() {
    let mut ppm = String::new();

    ppm.push_str("P3\n"); // magic number
    ppm.push_str("5 5\n"); // width <SP> height
    ppm.push_str("255\n"); // maxval
    // pixel array
    ppm.push_str("0 0 0   0 0 0   0 0 0   0 0 0   0 0 0\n");
    ppm.push_str("0 0 0   0 0 0   0 0 0   0 0 0   0 0 0\n");
    ppm.push_str("0 0 0   0 0 0   255 0 255   0 0 0   0 0 0\n");
    ppm.push_str("0 0 0   0 0 0   0 0 0   0 0 0   0 0 0\n");
    ppm.push_str("0 0 0   0 0 0   0 0 0   0 0 0   0 0 0\n");

    fs::write(OUT_PATH, ppm);
}

