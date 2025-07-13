use crate::vec3::Vec3;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

pub type Color = Vec3;

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let r = (linear_to_gamma(self.x()) * 255.999) as u8;
        let g = (linear_to_gamma(self.y()) * 255.999) as u8;
        let b = (linear_to_gamma(self.z()) * 255.999) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}

fn linear_to_gamma(component: f64) -> f64 {
    if component > 0.0 { component.sqrt() } else { 0.0 }
}
