use std::fmt::Debug;

use glam::DVec3;
use noise::{
    Abs, Fbm, MultiFractal, NoiseFn, Perlin
};

use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "PerlinRidged")]
#[skip_serializing_none]
pub struct PerlinRidgedNoiseConfig {
    seed: u32,
    octaves: Option<usize>,
    lacunarity: Option<f64>,
    persistence: Option<f64>,
    frequency: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "PerlinRidgedNoiseConfig")]
#[serde(into = "PerlinRidgedNoiseConfig")]
pub struct PerlinRidgedNoise {
    seed: u32,
    octaves: usize,
    lacunarity: f64,
    persistence: f64,
    frequency: f64,

    #[serde(skip)]
    pub perlin: Abs<f64, Fbm<Perlin>, 3>,
}

impl Debug for PerlinRidgedNoise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&PerlinRidgedNoiseConfig::from(self), f)
    }
}

impl PartialEq for PerlinRidgedNoise {
    fn eq(&self, other: &Self) -> bool {
        PerlinRidgedNoiseConfig::from(self) == PerlinRidgedNoiseConfig::from(other)
    }
}

impl From<&PerlinRidgedNoise> for PerlinRidgedNoiseConfig {
    fn from(value: &PerlinRidgedNoise) -> Self {
        PerlinRidgedNoiseConfig {
            seed: value.seed,
            octaves: Some(value.octaves),
            lacunarity: Some(value.lacunarity),
            frequency: Some(value.frequency),
            persistence: Some(value.persistence),
        }
    }
}

impl From<PerlinRidgedNoise> for PerlinRidgedNoiseConfig {
    fn from(value: PerlinRidgedNoise) -> Self {
        PerlinRidgedNoiseConfig::from(&value)
    }
}

impl From<PerlinRidgedNoiseConfig> for PerlinRidgedNoise {
    fn from(value: PerlinRidgedNoiseConfig) -> Self {
        type Noise = Fbm<Perlin>;

        let seed = value.seed;

        let octaves = value.octaves.unwrap_or(1);
        let persistence = value.persistence.unwrap_or(Noise::DEFAULT_PERSISTENCE);
        let lacunarity = value.lacunarity.unwrap_or(Noise::DEFAULT_LACUNARITY);
        let frequency = value.frequency.unwrap_or(Noise::DEFAULT_FREQUENCY);

        let perlin = Abs::new(
            Noise::new(seed)
                .set_octaves(octaves)
                .set_lacunarity(lacunarity)
                .set_frequency(frequency)
                .set_persistence(persistence)
        );

        Self {
            seed,
            octaves,
            lacunarity,
            frequency,
            persistence,
            perlin,
        }
    }
}

impl PerlinRidgedNoise {
    pub fn at(&self, point: &DVec3) -> f64 {
        self.perlin.get(point.to_array())
    }
}
