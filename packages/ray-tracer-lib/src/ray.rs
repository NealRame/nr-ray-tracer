use glam::DVec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    origin: DVec3,
    direction: DVec3,
    bounce: usize,
    time: f64,
}

impl Ray {
    pub fn new_at_time(origin: DVec3, direction: DVec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            bounce: 0,
            time,
        }
    }

    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self::new_at_time(origin, direction, 0.0)
    }
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t*self.direction
    }

    pub fn bounce(&mut self) -> &mut Self {
        self.bounce += 1;
        self
    }

    pub fn get_bounce(&self) -> usize {
        self.bounce
    }

    pub fn get_direction(&self) -> DVec3 {
        self.direction
    }

    pub fn get_origin(&self) -> DVec3 {
        self.origin
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }
}
