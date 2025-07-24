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
#[serde(rename = "Marble")]
#[skip_serializing_none]
pub struct MarbleConfig {
    seed: u32,
    frequency: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "MarbleConfig")]
#[serde(into = "MarbleConfig")]
pub struct Marble {
    seed: u32,
    frequency: f64,

    #[serde(skip)]
    pub perlin: Abs<f64, Fbm<Perlin>, 3>,
}

impl Debug for Marble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&MarbleConfig::from(self), f)
    }
}

impl PartialEq for Marble {
    fn eq(&self, other: &Self) -> bool {
        MarbleConfig::from(self) == MarbleConfig::from(other)
    }
}

impl From<&Marble> for MarbleConfig {
    fn from(value: &Marble) -> Self {
        MarbleConfig {
            seed: value.seed,
            frequency: Some(value.frequency),
        }
    }
}

impl From<Marble> for MarbleConfig {
    fn from(value: Marble) -> Self {
        MarbleConfig::from(&value)
    }
}

impl From<MarbleConfig> for Marble {
    fn from(value: MarbleConfig) -> Self {
        type Noise = Fbm<Perlin>;

        let seed = value.seed;
        let frequency = value.frequency.unwrap_or(Noise::DEFAULT_FREQUENCY);

        let perlin = Abs::new(
            Noise::new(seed)
                .set_octaves(7)
                .set_frequency(frequency)
        );

        Self {
            seed,
            frequency,
            perlin,
        }
    }
}

impl Marble {
    pub fn at(&self, point: &DVec3) -> f64 {
        let n = self.perlin.get(point.to_array());
        (1. + f64::sin(self.frequency*point.z + 10.*n))/2.
    }
}
