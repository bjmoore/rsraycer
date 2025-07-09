const EMPTY: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);
const UNIVERSE: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }
}
