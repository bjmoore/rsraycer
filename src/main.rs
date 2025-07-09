use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Point;
use crate::vec3::Vec3;
use crate::vec3::dot;
use std::fmt::Write;
use std::fs;
use std::rc::Rc;

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

const OUT_PATH: &str = "out.ppm";

fn main() {
    let mut ppm = String::new();

    // Treat these as input parameters; height is derived from width and aspect
    let aspect: f64 = 16.0 / 9.0;
    let img_width: u32 = 1920;

    // img_* values are the dimensions of the rendered image in pixels.
    let img_height: u32 = ((img_width as f64) / aspect) as u32;
    let img_height = if img_height == 0 { 1 } else { img_height };

    // viewport_* values are the dimensions of the viewing rectangle in world-space.
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (img_width as f64) / (img_height as f64);

    // world of spheres.
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // other camera parameters.
    let focal_length = 1.0;
    let camera_center = Point::new(0.0, 0.0, 0.0);

    // w/l vectors for the viewport rectangle.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // pixel-to-pixel spacing vectors.
    let pixel_delta_u = viewport_u / (img_width as f64);
    let pixel_delta_v = viewport_v / (img_height as f64);

    // anchor loc for top-left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let anchor_pixel = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    writeln!(ppm, "P3"); // magic number
    writeln!(ppm, "{} {}", img_width, img_height); // width <SP> height
    writeln!(ppm, "255"); // maxval
    // pixel array
    for j in 0..img_height {
        eprintln!("Scanlines remaining: {}", (img_height - j));
        for i in 0..img_width {
            let pixel_center = anchor_pixel + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            writeln!(ppm, "{}", ray_color(&ray, &world));
        }
    }

    fs::write(OUT_PATH, ppm);
}

fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, f64::MAX) {
        if hit.t > 0.0 {
            return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
        }
    }

    let unit_dir = ray.dir.unit();
    let a = 0.5 * (unit_dir.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
