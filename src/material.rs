use crate::color::Color;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vec3::random_unit_vector;
use crate::vec3::reflect;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let scatter_dir = hit.normal + random_unit_vector();
        let scattered_ray = if !scatter_dir.near_zero() {
            Ray::new(hit.p, scatter_dir)
        } else {
            Ray::new(hit.p, hit.normal)
        };
        Some((self.albedo, scattered_ray))
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected_dir = reflect(r_in.dir, hit.normal);
        let scattered_ray = Ray::new(hit.p, reflected_dir);
        Some((self.albedo, scattered_ray))
    }
}
