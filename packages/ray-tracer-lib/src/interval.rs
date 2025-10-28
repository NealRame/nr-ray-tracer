use std::f64;
use std::ops::{AddAssign, MulAssign};

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

    pub const fn ensure(a: f64, b: f64) -> Self {
        if a < b {
            Self::new(a, b)
        } else {
            Self::new(b, a)
        }
    }

    pub const fn union(&self, other: &Interval) -> Self {
        Self {
            min: f64::min(self.min, other.min),
            max: f64::max(self.max, other.max),
        }
    }

    pub const fn intersection(&self, other: &Interval) -> Self {
        Self {
            min: f64::max(self.min, other.min),
            max: f64::min(self.max, other.max),
        }
    }

    pub const fn is_empty(&self) -> bool {
        return self.min > self.max
    }

    pub const fn pad(&self, padding: f64) -> Self {
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub const fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    pub const fn with_lower_bound(&self, min: f64) -> Self {
        Self { min, max: self.max }
    }

    pub const fn with_upper_bound(&self, max: f64) -> Self {
        Self { min: self.min, max }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl AddAssign<f64> for Interval {
    fn add_assign(&mut self, rhs: f64) {
        self.min += rhs;
        self.max += rhs;
    }
}

impl MulAssign<f64> for Interval {
    fn mul_assign(&mut self, rhs: f64) {
        self.min *= rhs;
        self.max *= rhs;
    }
}
