use glam::DVec3;

use crate::interval::*;
use crate::ray::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    const EPSILON: f64 = 0.0001;

    const fn pad_to_minimums(self) -> Self {
        let x = if self.x.size() < Self::EPSILON {
            self.x.pad((Self::EPSILON - self.x.size())/2.)
        } else { self.x };

        let y = if self.y.size() < Self::EPSILON {
            self.y.pad((Self::EPSILON - self.y.size())/2.)
        } else { self.y };

        let z = if self.z.size() < Self::EPSILON {
            self.z.pad((Self::EPSILON - self.z.size())/2.)
        } else { self.z };

        Self { x, y, z }
    }
}

impl AABB {
    pub const fn new(
        x: Interval,
        y: Interval,
        z: Interval,
    ) -> Self {
        Self::pad_to_minimums(Self { x, y, z })
    }

    pub const fn union(
        &self,
        other: &AABB,
    ) -> Self {
        Self::new(
            self.x.union(&other.x),
            self.y.union(&other.y),
            self.z.union(&other.z),
        )
    }

    pub const fn from_points(
        a: DVec3,
        b: DVec3,
    ) -> Self {
        let x = if a.x < b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };

        let y = if a.y < b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };

        let z = if a.z < b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        Self::new(x, y, z)
    }

    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Self = Self {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };
}

impl AABB {
    pub fn axis_interval(&self, index: usize) -> &Interval {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis index")
        }
    }

    pub fn longest_axis(&self) -> usize {
        [self.x.size(), self.y.size(), self.z.size()]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .map(|el| el.0)
            .unwrap()
    }

    pub fn hit(&self, ray: &Ray, hit_range: Interval) -> bool {
        let mut interval = hit_range;

        let origin = ray.get_origin();
        let direction = ray.get_direction();

        for (axis, o, d) in [
            (self.x, origin.x, direction.x),
            (self.y, origin.y, direction.y),
            (self.z, origin.z, direction.z),
        ] {
            interval = interval.intersection(&Interval::ensure(
                (axis.min - o)/d,
                (axis.max - o)/d,
            ));

            if interval.is_empty() {
                return false;
            }
        }

        true
    }
}
