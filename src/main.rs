use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::sphere::Sphere;
use crate::vec3::Point;
use crate::vec3::Vec3;
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
    let blue_metal = Rc::new(Metal::new(Color::new(0.1, 0.1, 0.8), 0.05));
    let glass = Rc::new(Dielectric::new(1.5));

    // world of spheres.
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        glass.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.5),
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

    let mut camera = camera::Camera::new(
        aspect,
        img_width,
        90.0,
        Point::new(-2.0, 2.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    camera.render(&world);
}
