use std::fmt::Debug;

use glam::{
    DVec2,
    DVec3,
};

pub trait Texture: Debug {
    fn get_color(&self, uv_coord: DVec2, point: DVec3) -> DVec3;
}
