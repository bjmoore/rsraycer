use crate::color::Color;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vec3::dot;
use crate::vec3::random_unit_vector;
use crate::vec3::reflect;
use crate::vec3::refract;

use rand::prelude::*;

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
    fn scatter(&self, _r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
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
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected_dir = reflect(r_in.dir, hit.normal);
        let reflected_dir = reflected_dir.unit() + (self.fuzz * random_unit_vector());
        let scattered_ray = Ray::new(hit.p, reflected_dir);
        if dot(scattered_ray.dir, hit.normal) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0.powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_index = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = r_in.dir.unit();
        let cos_theta = 1.0f64.min(dot(-unit_dir, hit.normal));
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        let scattered_dir = if sin_theta * refraction_index > 1.0
            || Self::reflectance(cos_theta, refraction_index) > rand::rng().random::<f64>()
        {
            reflect(unit_dir, hit.normal)
        } else {
            refract(unit_dir, hit.normal, refraction_index)
        };
        let scattered_ray = Ray::new(hit.p, scattered_dir);

        Some((attenuation, scattered_ray))
    }
}
