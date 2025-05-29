use std::f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval{
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f64::INFINITY, -f64::INFINITY);
    pub const UNIVERSE: Self = Self::new(-f64::INFINITY, f64::INFINITY);
    pub const POSITIVE: Self = Self::new(0.0, f64::INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn ensure(a: f64, b: f64) -> Self {
        if a < b {
            Self::new(a, b)
        } else {
            Self::new(b, a)
        }
    }

    pub fn union(&self, other: &Interval) -> Self {
        Self {
            min: f64::min(self.min, other.min),
            max: f64::max(self.max, other.max),
        }
    }

    pub fn intersection(&self, other: &Interval) -> Self {
        Self {
            min: f64::max(self.min, other.min),
            max: f64::min(self.max, other.max),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.min > self.max
    }

    pub fn pad(&self, padding: f64) -> Self {
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
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

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
