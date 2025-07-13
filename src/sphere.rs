use crate::hittable::Hit;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point;
use crate::vec3::dot;

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
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
        if !ray_t.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut result = Hit::default();
        result.p = ray.at(root);
        result.t = root;
        let outward_normal = (result.p - self.center).unit();
        result.set_face_normal(ray, &outward_normal);

        Some(result)
    }
}
