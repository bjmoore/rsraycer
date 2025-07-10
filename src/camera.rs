use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point;
use crate::vec3::Vec3;

use std::fmt::Write;
use std::fs;

const OUT_PATH: &str = "out.ppm";

#[derive(Debug, Default, Copy, Clone)]
pub struct Camera {
    pub aspect: f64,
    pub img_width: u32,

    img_height: u32,
    center: Point,
    anchor: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect: f64, img_width: u32) -> Self {
        Self {
            aspect,
            img_width,
            ..Default::default()
        }
    }
    
    pub fn render<T: Hittable>(&mut self, world: &T) {
        self.initialize();

        let mut ppm = String::new();

        writeln!(ppm, "P3"); // magic number
        writeln!(ppm, "{} {}", self.img_width, self.img_height); // width <SP> height
        writeln!(ppm, "255"); // maxval

        for j in 0..self.img_height {
            eprintln!("Scanlines remaining: {}", (self.img_height - j));
            for i in 0..self.img_width {
                let pixel_center = self.anchor + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                writeln!(ppm, "{}", self.ray_color(&ray, world));
            }
        }

        fs::write(OUT_PATH, ppm);
    }

    fn initialize(&mut self) {
        // img_* values are the dimensions of the rendered image in pixels.
        self.img_height = ((self.img_width as f64) / self.aspect) as u32;
        self.img_height = if self.img_height == 0 {
            1
        } else {
            self.img_height
        };

        // viewport_* values are the dimensions of the viewing rectangle in world-space.
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            viewport_height * (self.img_width as f64) / (self.img_height as f64);

        // other camera parameters.
        let focal_length = 1.0;
        self.center = Point::new(0.0, 0.0, 0.0);

        // w/l vectors for the viewport rectangle.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // pixel-to-pixel spacing vectors.
        self.pixel_delta_u = viewport_u / (self.img_width as f64);
        self.pixel_delta_v = viewport_v / (self.img_height as f64);

        // anchor loc for top-left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.anchor = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color<T: Hittable>(&self, r: &Ray, world: &T) -> Color {
        if let Some(hit) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
            if hit.t > 0.0 {
                return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
            }
        }

        let unit_dir = r.dir.unit();
        let a = 0.5 * (unit_dir.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
