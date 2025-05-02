use glam::DVec3;

pub struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self {
            origin,
            direction,
        }
    }
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t*self.direction
    }

    pub fn get_direction(&self) -> DVec3 {
        self.direction
    }

    pub fn get_origin(&self) -> DVec3 {
        self.origin
    }
}
