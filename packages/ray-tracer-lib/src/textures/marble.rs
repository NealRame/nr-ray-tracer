use std::fmt::Debug;

use glam::{
    DVec2,
    DVec3,
};

use noise::{
    Abs, Fbm, MultiFractal, NoiseFn, Perlin
};

use super::texture::Texture;

type Noise = Fbm<Perlin>;

#[derive(Clone)]
pub struct Marble {
    seed: u32,
    frequency: f64,
    perlin: Abs<f64, Noise, 3>,
}

#[derive(Clone, Default)]
pub struct MarbleBuilder {
    seed: Option<u32>,
    frequency: Option<f64>,
}

impl MarbleBuilder {
    pub fn with_seed(
        &mut self,
        value: Option<u32>,
    ) -> &mut Self {
        self.seed = value;
        self
    }

    pub fn with_frequency(
        &mut self,
        value: Option<f64>,
    ) -> &mut Self {
        self.frequency = value;
        self
    }

    pub fn build(self) -> Marble {
        let seed = self.seed.unwrap_or(0);
        let frequency = self.frequency.unwrap_or(Noise::DEFAULT_FREQUENCY);

        let perlin = Abs::new(
            Noise::new(seed)
                .set_octaves(7)
                .set_frequency(frequency)
        );

        Marble {
            seed,
            frequency,
            perlin,
        }
    }
}

impl Debug for Marble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Marble")
            .field("seed", &self.seed)
            .field("frequency", &self.frequency)
            .finish()
    }
}

impl Default for Marble {
    fn default() -> Self {
        MarbleBuilder::default().build()
    }
}

impl PartialEq for Marble {
    fn eq(&self, other: &Self) -> bool {
        self.seed == other.seed && self.frequency == other.frequency
    }
}

impl Texture for Marble {
    fn get_color(
        &self,
        _: DVec2,
        point: DVec3,
    ) -> DVec3 {
        let n = self.perlin.get(point.to_array());
        let v = (1. + f64::sin(self.frequency*point.z + 10.*n))/2.;

        v*DVec3::ONE
    }
}
