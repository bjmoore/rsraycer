use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::sphere::Sphere;
use crate::vec3::Point;
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // Treat these as input parameters; height is derived from width and aspect
    let aspect: f64 = 16.0 / 9.0;
    let img_width: u32 = 1920;

    let stone = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let blue_metal = Rc::new(Metal::new(Color::new(0.1, 0.1, 0.8)));

    // world of spheres.
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        blue_metal.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        stone.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        stone.clone(),
    )));

    let mut camera = camera::Camera::new(aspect, img_width);
    camera.render(&world);
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
