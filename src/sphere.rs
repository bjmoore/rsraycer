use crate::hittable::Hittable;
use crate::hittable::HitResult;
use crate::vec3::Point;
use crate::vec3::dot;
use crate::ray::Ray;

pub struct Sphere {
    center: Point,
    radius: f64
}

impl Sphere {
    pub const fn new(center: Point, radius: f64) -> Self{
        Self {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let oc = self.center - ray.origin;
        let a = ray.dir.norm_sq();
        let h = dot(ray.dir, oc);
        let c = oc.norm_sq() - self.radius.powf(2.0);
        let discriminant = h.powf(2.0) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (h - sqrt_discriminant) / a;
        if a < t_min || a > t_max {
            root = (h + sqrt_discriminant) / a;
            if a < t_min || a > t_max {
                return None
            }
        }

        let p = ray.at(root);

        Some(HitResult {
            p,
            normal: (p - self.center) / self.radius,
            t: root,
        })
    }
}
