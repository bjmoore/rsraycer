use crate::hittable::HitResult;
use crate::hittable::Hittable;
use crate::ray::Ray;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }
    
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let mut result = None;
        let mut current_max = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, t_min, current_max) {
                current_max = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}
