use glam::{
    DVec2,
    DVec3,
};

use noise::{
    Abs,
    Fbm,
    MultiFractal,
    NoiseFn,
    Perlin,
};

use super::texture::Texture;

type Noise = Fbm<Perlin>;

#[derive(Clone)]
pub struct PerlinRidgedNoise {
    seed: u32,
    octaves: usize,
    lacunarity: f64,
    persistence: f64,
    frequency: f64,
    perlin: Abs<f64, Fbm<Perlin>, 3>,
}

#[derive(Default)]
pub struct PerlinRidgedNoiseBuilder {
    seed: Option<u32>,
    octaves: Option<usize>,
    lacunarity: Option<f64>,
    persistence: Option<f64>,
    frequency: Option<f64>,
}

impl PerlinRidgedNoiseBuilder {
    pub fn with_seed(self, value: u32) -> Self {
        Self { seed: Some(value), ..self }
    }

    pub fn with_octaves(self, value: usize) -> Self {
        Self { octaves: Some(value), ..self }
    }

    pub fn with_lacunarity(self, value: f64) -> Self {
        Self { lacunarity: Some(value), ..self }
    }

    pub fn with_persistence(self, value: f64) -> Self {
        Self { persistence: Some(value), ..self }
    }

    pub fn with_frequency(self, value: f64) -> Self {
        Self { frequency: Some(value), ..self }
    }

    pub fn build(self) -> PerlinRidgedNoise {
        let seed = self.seed.unwrap_or(0);

        let octaves = self.octaves.unwrap_or(1);
        let persistence = self.persistence.unwrap_or(Noise::DEFAULT_PERSISTENCE);
        let lacunarity = self.lacunarity.unwrap_or(Noise::DEFAULT_LACUNARITY);
        let frequency = self.frequency.unwrap_or(Noise::DEFAULT_FREQUENCY);

        let perlin = Abs::new(
            Noise::new(seed)
                .set_octaves(octaves)
                .set_lacunarity(lacunarity)
                .set_frequency(frequency)
                .set_persistence(persistence)
        );

        PerlinRidgedNoise {
            seed,
            octaves,
            lacunarity,
            frequency,
            persistence,
            perlin,
        }
    }
}

impl PartialEq for PerlinRidgedNoise {
    fn eq(&self, other: &Self) -> bool {
        self.seed == other.seed
        && self.octaves == other.octaves
        && self.lacunarity == other.lacunarity
        && self.persistence == other.persistence
        && self.frequency == other.frequency
    }
}

impl Default for PerlinRidgedNoise {
    fn default() -> Self {
        PerlinRidgedNoiseBuilder::default().build()
    }
}

impl Texture for PerlinRidgedNoise {
    fn get_color(
        &self,
        _: DVec2,
        point: DVec3
    ) -> DVec3 {
        let v = self.perlin.get(point.to_array());

        v*DVec3::ONE
    }
}
