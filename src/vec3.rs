use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub type Point = Vec3;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn norm(&self) -> f64 {
        (self[0].powf(2.0) + self[1].powf(2.0) + self[2].powf(2.0)).sqrt()
    }

    pub fn norm_sq(&self) -> f64 {
        self[0].powf(2.0) + self[1].powf(2.0) + self[2].powf(2.0)
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(&self, other: Vec3) -> Self {
        Self::new(
            self[1] * other[2] - self[2] * other[1],
            self[0] * other[2] - self[2] * other[0],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    pub fn unit(&self) -> Self {
        *self / self.norm()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self[0] + other[0], self[1] + other[1], self[2] + other[2]);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self[0] - other[0], self[1] - other[1], self[2] - other[2]);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scale: f64) -> Self {
        Self::new(self[0] * scale, self[1] * scale, self[2] * scale)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scale: f64) {
        *self = Self::new(self[0] * scale, self[1] * scale, self[2] * scale);
    }
}

impl Mul<Vec3> for u32 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        Self::Output::new(
            vector[0] * self as f64,
            vector[1] * self as f64,
            vector[2] * self as f64,
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Self::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Self::new(self[0] * other[0], self[1] * other[1], self[2] * other[2]);
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scale: f64) -> Self {
        Self::new(self[0] / scale, self[1] / scale, self[2] / scale)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scale: f64) {
        *self = Self::new(self[0] / scale, self[1] / scale, self[2] / scale);
    }
}

// normalize to 1

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_negate() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(-v, expected);
    }

    #[test]
    fn test_add() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.clone() + v, expected);
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(2.0, 2.0, 2.0);

        v += v.clone();
        assert_eq!(v, expected);
    }

    #[test]
    fn test_sub() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let w = Vec3::new(2.0, 3.0, 4.0);
        let expected = Vec3::new(-1.0, -2.0, -3.0);

        assert_eq!(v - w, expected);
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vec3::new(1.0, 1.0, 1.0);
        let w = Vec3::new(2.0, 3.0, 4.0);
        let expected = Vec3::new(-1.0, -2.0, -3.0);

        v -= w;
        assert_eq!(v, expected);
    }

    #[test]
    fn test_scalar_mul() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v * 2.0, expected);
    }

    #[test]
    fn test_scalar_mul_assign() {
        let mut v = Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(2.0, 2.0, 2.0);

        v *= 2.0;
        assert_eq!(v, expected);
    }

    #[test]
    fn test_pairwise_mul() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let w = Vec3::new(2.0, 2.0, 2.0);
        let expected = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v * w, expected);
    }

    #[test]
    fn test_pairwise_mul_assign() {
        let mut v = Vec3::new(1.0, 1.0, 1.0);
        let w = Vec3::new(2.0, 2.0, 2.0);
        let expected = Vec3::new(2.0, 2.0, 2.0);

        v *= w;
        assert_eq!(v, expected);
    }

    #[test]
    fn test_scalar_div() {
        let v = Vec3::new(2.0, 2.0, 2.0);
        let expected = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v / 2.0, expected);
    }

    #[test]
    fn test_scalar_div_assign() {
        let mut v = Vec3::new(2.0, 2.0, 2.0);
        let expected = Vec3::new(1.0, 1.0, 1.0);

        v /= 2.0;
        assert_eq!(v, expected);
    }

    #[test]
    fn test_norm() {
        let v = Vec3::new(2.0, 2.0, 2.0);
        let expected = (2.0_f64.powf(2.0) * 3.0).sqrt();

        assert_eq!(v.norm(), expected);
    }

    #[test]
    fn test_norm_sq() {
        let v = Vec3::new(2.0, 2.0, 2.0);
        let expected = 2.0_f64.powf(2.0) * 3.0;

        assert_eq!(v.norm_sq(), expected);
    }

    #[test]
    fn test_dot() {
        let v = Vec3::new(2.0, 2.0, 2.0);
        let w = Vec3::new(2.0, 2.0, 2.0);
        let expected = 2.0 * 2.0 + 2.0 * 2.0 + 2.0 * 2.0;

        assert_eq!(v.dot(w), expected);
    }

    #[test]
    fn test_cross() {
        let u = Vec3::new(2.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 2.0, 0.0);
        let expected = Vec3::new(0.0, 0.0, 4.0);

        assert_eq!(u.cross(v), expected);
    }

    #[test]
    fn test_unit() {
        let v = Vec3::new(100.0, 0.0, 0.0);
        let expected = Vec3::new(1.0, 0.0, 0.0);

        assert_eq!(v.unit(), expected);
    }
}
