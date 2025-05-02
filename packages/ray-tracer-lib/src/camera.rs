use glam::{
    DVec2, DVec3, Vec3Swizzles
};

use crate::image::Image;
use crate::ray::Ray;

pub struct Camera {
    eye: DVec3,

    image: Image,

    viewport_pixel_delta_u: DVec3,
    viewport_pixel_delta_v: DVec3,
    viewport_top_left: DVec3,
}

impl Camera {
    pub fn new_with_image(
        image: Image,
        eye: DVec3,
        focal_length: f64,
    ) -> Self {
        let viewport_height = 2.0;
        let viewport_width = image.get_aspect_ratio()*viewport_height;

        let viewport_u =  DVec3::X*viewport_width;
        let viewport_v = -DVec3::Y*viewport_height;

        let viewport_pixel_delta_u = viewport_u/(image.get_width() as f64);
        let viewport_pixel_delta_v = viewport_v/(image.get_height() as f64);

        let viewport_top_left =
                eye - DVec3::Z*focal_length
                    - viewport_u/2.
                    - viewport_v/2.
                    + (viewport_pixel_delta_u + viewport_pixel_delta_v)/2.
                ;

        Self {
            eye,

            image,

            viewport_pixel_delta_u,
            viewport_pixel_delta_v,
            viewport_top_left,
        }
    }
}

impl Camera {
    pub fn map<F>(
        &mut self,
        mut f: F
    ) -> &mut Self where F: FnMut(&Ray, DVec2) -> DVec3 {
        self.image.map(|x, y| {
            let pixel =
                self.viewport_top_left
                    + (x as f64)*self.viewport_pixel_delta_u
                    + (y as f64)*self.viewport_pixel_delta_v
                ;

            let direction = pixel - self.eye;
            let ray = Ray::new(self.eye, direction);

            f(&ray, pixel.xy())
        });
        self
    }
}

impl Camera {
    pub fn take_image(self) -> Image {
        self.image
    }
}
