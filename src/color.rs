use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn color_pixel(color: Color) -> String {
    let mut out = String::new();
    let r = (color.x() * 255.999) as u8;
    let g = (color.y() * 255.999) as u8;
    let b = (color.z() * 255.999) as u8;

    out.push_str(&r.to_string());
    out.push(' ');
    out.push_str(&g.to_string());
    out.push(' ');
    out.push_str(&b.to_string());
    
    out
}
