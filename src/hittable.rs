use crate::ray::Ray;
use crate::vec3::Point;
use crate::vec3::Vec3;
use crate::vec3::dot;
use crate::interval::Interval;
use crate::material::Material;
use std::rc::Rc;

pub struct Hit {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>
}

impl Hit {
    pub fn new(r: &Ray, p: Point, t: f64, outward_normal: &Vec3, mat: Rc<dyn Material>) -> Self {
        let front_face = dot(r.dir, *outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
            mat
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<Hit>;
}
