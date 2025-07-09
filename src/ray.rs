use crate::vec3::Vec3;
use crate::vec3::Point;

pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self {
        Self {
            origin,
            dir
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray {
            origin: Vec3::new(1.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 1.0, 0.0)
        };

        let expected = Vec3::new(1.0, 5.0, 0.0);

        assert_eq!(ray.at(5.0), expected);
    }
}
