use crate::hittable::Hit;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<Hit> {
        let mut result = None;
        let mut current_max = ray_t.max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, Interval::new(ray_t.min, current_max)) {
                current_max = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}
