use rand::Rng;

use serde::{
    Deserialize,
    Serialize,
};

use glam::{
    DVec2,
    DVec3,
    U64Vec2,
    Vec3,
};

use crate::vector::FromRng;

use super::image::Image;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Checker {
    even: DVec3,
    odd: DVec3,
    scale: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Texture {
    Checker(Checker),
    Image(Image),
    SolidColor(DVec3),
}

impl Texture {
    pub fn new_checker(
        even: DVec3,
        odd: DVec3,
        scale: f64,
    ) -> Self {
        Self::Checker(Checker { even, odd, scale })
    }

    pub fn default_checker() -> Self {
        Self::new_checker(DVec3::ONE, DVec3::ZERO, 0.5)
    }

    pub fn new_solid_color(color: DVec3) -> Self {
        Self::SolidColor(color)
    }

    pub fn default_solid_color() -> Self {
        Self::SolidColor(DVec3::ONE)
    }
}

impl Texture {
    pub fn checker_from_rng<R: Rng>(rng: &mut R) -> Self {
        Self::new_checker(
            DVec3::from_rng_ranged(rng, 0.0..=1.0),
            DVec3::from_rng_ranged(rng, 0.0..=1.0),
            rng.random_range(0.0..=1.0),
        )
    }

    pub fn solid_color_from_rng<R: Rng>(rng: &mut R) -> Self {
        Self::SolidColor(DVec3::from_rng_ranged(rng, 0.0..=1.0))
    }
}

impl Texture {
    pub fn get_color(
        &self,
        uv: DVec2,
        _: DVec3,
    ) -> DVec3 {
        match self {
            Self::Checker(Checker { even, odd, scale }) => {
                let v = (uv*scale).as_u64vec2().dot(U64Vec2::ONE);

                if v%2 == 0 {
                    *even
                } else {
                    *odd
                }
            },
            Self::Image(Image { image, .. }) => {
                let x = (uv.x.clamp(0., 1.)*(image.width() as f64)) as u32;
                let y = ((1.0 - uv.y.clamp(0., 1.))*(image.height() as f64)) as u32;

                Vec3::from_array(image.get_pixel(x, y).0).as_dvec3()
            },
            Self::SolidColor(albedo) => {
                *albedo
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use serde_test::{
        Token,
        assert_tokens,
    };
    use crate::textures::{Checker, Texture};

    #[test]
    fn test_solid_color_serde() {
        let solid_color_texture = Texture::SolidColor(DVec3::ONE);

        assert_tokens(&solid_color_texture, &[
            Token::NewtypeVariant { name: "Texture", variant: "SolidColor" },
            Token::TupleStruct { name: "DVec3", len: 3, },
            Token::F64(1.0),
            Token::F64(1.0),
            Token::F64(1.0),
            Token::TupleStructEnd,
        ]);
    }

    #[test]
    fn test_checker_serde() {
        let solid_color_texture = Texture::Checker(Checker {
            even: DVec3::ONE,
            odd: DVec3::ZERO,
            scale: 1.0,
        });

        assert_tokens(&solid_color_texture, &[
            Token::NewtypeVariant { name: "Texture", variant: "Checker" },
            Token::Struct { name: "Checker", len: 3 },
            Token::Str("even"),
            Token::TupleStruct { name: "DVec3", len: 3 },
            Token::F64(1.0),
            Token::F64(1.0),
            Token::F64(1.0),
            Token::TupleStructEnd,
            Token::Str("odd"),
            Token::TupleStruct { name: "DVec3", len: 3 },
            Token::F64(0.0),
            Token::F64(0.0),
            Token::F64(0.0),
            Token::TupleStructEnd,
            Token::Str("scale"),
            Token::F64(1.0),
            Token::StructEnd,
        ]);
    }
}
