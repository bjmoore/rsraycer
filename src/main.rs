use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::sphere::Sphere;
use crate::vec3::Point;
use crate::vec3::Vec3;
use crate::vec3::random;
use rand::prelude::*;
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

    let ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    // world of spheres.
    let mut world = HittableList::new();

    // ground
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -10_000.0, 0.0),
        10_000.0,
        ground.clone(),
    )));

    // one of each big guy, at (-1, 0), (0, 0), (1, 0)
    let blue = Rc::new(Lambertian::new(Color::new(0.1, 0.1, 0.8)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 2.0, 0.0),
        2.0,
        blue.clone(),
    )));

    let glass = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        glass.clone(),
    )));

    let shiny = Rc::new(Metal::new(Color::new(0.7, 0.7, 0.7), 0.03));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 2.0, 0.0),
        2.0,
        shiny.clone(),
    )));

    // lots of random marbles checkerboarded around.
    for i in -11..11 {
        for j in -11..11 {
            let mat = rand::rng().random::<f64>();
            let center = Point::new(
                i as f64 + 0.9 * rand::rng().random::<f64>(),
                0.2,
                j as f64 + 0.9 * rand::rng().random::<f64>(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).norm() > 1.0
                && (center - Point::new(-4.0, 0.2, 0.0)).norm() > 1.0
                && (center - Point::new(0.0, 0.2, 0.0)).norm() > 1.0
            {
                if mat < 0.8 {
                    // diffuse
                    let albedo = random() * random();
                    let sphere_mat = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat.clone())));
                } else if mat < 0.95 {
                    // shiny
                    let albedo = random() * random();
                    let fuzz = rand::rng().random_range(0.0..0.5f64);
                    let sphere_mat = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat.clone())));
                } else {
                    // glass
                    let sphere_mat = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat.clone())));
                }
            };
        }
    }

    let mut camera = camera::Camera::new(
        aspect,
        img_width,
        30.0,
        Point::new(13.0, 2.0, 13.0),
        Point::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    camera.render(&world);
}
