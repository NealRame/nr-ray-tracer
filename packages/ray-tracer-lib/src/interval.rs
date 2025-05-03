use std::f64;

#[derive(Clone, Copy)]
pub struct Interval{
    min: f64,
    max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f64::INFINITY, -f64::INFINITY);
    pub const UNIVERSE: Self = Self::new(-f64::INFINITY, f64::INFINITY);
    pub const POSITIVE: Self = Self::new(0.0, f64::INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    pub fn with_lower_bound(&self, min: f64) -> Self {
        Self { min, max: self.max }
    }

    pub fn with_upper_bound(&self, max: f64) -> Self {
        Self { min: self.min, max }
    }
}
