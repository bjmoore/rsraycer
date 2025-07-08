use crate::vec3::Vec3;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

pub type Color = Vec3;

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let r = (self.x() * 255.999) as u8;
        let g = (self.y() * 255.999) as u8;
        let b = (self.z() * 255.999) as u8;

        write!(f, "{} {} {}", r, g, b)
    }
}
