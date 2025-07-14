use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point;
use crate::vec3::Vec3;
use crate::vec3::cross;

use std::fmt::Write;
use std::fs;

use rand::prelude::*;

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
    samples_per_pixel: u32,
    pixel_sample_scale: f64,
    max_depth: u32,
    vfov: f64,

    look_from: Point,
    look_at: Point,
    v_up: Vec3,

    // Camera basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(aspect: f64, img_width: u32, vfov: f64, look_from: Point, look_at: Point, v_up: Vec3) -> Self {
        Self {
            aspect,
            img_width,
            vfov,
            look_from,
            look_at,
            v_up,
            samples_per_pixel: 10,
            max_depth: 10,
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
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color += self.ray_color(&r, self.max_depth, world);
                }
                writeln!(ppm, "{}", self.pixel_sample_scale * color);
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

        self.center = self.look_from;

        // other camera parameters.
        let focal_length = (self.look_from - self.look_at).norm();
        let theta = deg_to_rad(self.vfov);
        let h = (theta/2.0).tan();

        self.w = (self.look_from - self.look_at).unit();
        self.u = cross(self.v_up, self.w).unit();
        self.v = cross(self.w, self.u);

        // viewport_* values are the dimensions of the viewing rectangle in world-space.
        let viewport_height: f64 = 2.0 * h * focal_length;
        let viewport_width: f64 =
            viewport_height * (self.img_width as f64) / (self.img_height as f64);


        // w/l vectors for the viewport rectangle.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // pixel-to-pixel spacing vectors.
        self.pixel_delta_u = viewport_u / (self.img_width as f64);
        self.pixel_delta_v = viewport_v / (self.img_height as f64);

        // anchor loc for top-left pixel.
        let viewport_upper_left =
            self.center - (focal_length * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.anchor = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;
    }

    fn ray_color<T: Hittable>(&self, r: &Ray, depth: u32, world: &T) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered_ray)) = hit.mat.scatter(r, &hit) {
                return attenuation * self.ray_color(&scattered_ray, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_dir = r.dir.unit();
        let a = 0.5 * (unit_dir.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.anchor
            + (i as f64 + offset.x()) * self.pixel_delta_u
            + (j as f64 + offset.y()) * self.pixel_delta_v;

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        rand::rng().random::<f64>() - 0.5f64,
        rand::rng().random::<f64>() - 0.5f64,
        0.0,
    )
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
