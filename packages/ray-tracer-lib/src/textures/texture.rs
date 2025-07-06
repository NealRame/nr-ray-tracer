use rand::{rngs::ThreadRng, Rng};

use serde::{
    Deserialize,
    Serialize,
};

use glam::{
    DVec2,
    DVec3,
};

use crate::vector::FromRng;

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename = "Checker")]
struct CheckerData {
    even: DVec3,
    odd: DVec3,
    scale: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(from = "CheckerData", into = "CheckerData")]
pub struct Checker {
    even: DVec3,
    odd: DVec3,
    inv_scale: f64,
}

impl Into<CheckerData> for Checker {
    fn into(self) -> CheckerData {
        CheckerData {
            even: self.even,
            odd: self.odd,
            scale: self.inv_scale.recip(),
        }
    }
}

impl From<CheckerData> for Checker {
    fn from(value: CheckerData) -> Self {
        Checker {
            even: value.even,
            odd: value.odd,
            inv_scale: value.scale.recip(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Texture {
    Checker(Checker),
    SolidColor(DVec3),
}

impl Texture {
    pub fn new_checker(
        even: DVec3,
        odd: DVec3,
        scale: f64,
    ) -> Self {
        Self::Checker(Checker { even, odd, inv_scale: 1.0/scale })
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
    pub fn checker_from_rng(rng: &mut ThreadRng) -> Self {
        Self::new_checker(
            DVec3::from_rng_ranged(rng, 0.0..=1.0),
            DVec3::from_rng_ranged(rng, 0.0..=1.0),
            rng.random_range(0.0..=1.0),
        )
    }

    pub fn solid_color_from_rng(rng: &mut ThreadRng) -> Self {
        Self::SolidColor(DVec3::from_rng_ranged(rng, 0.0..=1.0))
    }
}

impl Texture {
    pub fn get_color(
        &self,
        _: DVec2,
        point: DVec3,
    ) -> DVec3 {
        match *self {
            Self::Checker(Checker { even, odd, inv_scale }) => {

                let x_int = (inv_scale*point.x).floor() as i32;
                let y_int = (inv_scale*point.y).floor() as i32;
                let z_int = (inv_scale*point.z).floor() as i32;

                if (x_int + y_int + z_int)%2 == 0 {
                    even
                } else {
                    odd
                }
            },
            Self::SolidColor(albedo) => {
                albedo
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
            inv_scale: 1.0,
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
