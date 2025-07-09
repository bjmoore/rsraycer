use crate::vec3::Point;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitResult {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}
