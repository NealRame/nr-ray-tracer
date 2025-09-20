use std::fs;
use std::io;
use std::io::Write;

use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

pub fn run(args: &CreateArgs) -> Result<()> {
    const GROUND_SPHERE_RADIUS: f64 = 1_000_000.0;

    let textures = vec![
        TextureConfig::SolidColor { color: 0.5*DVec3::ONE },
        TextureConfig::Noise {
            seed: None,
            frequency: Some(0.2),
            octaves: Some(8),
            lacunarity: None,
            persistence: None,
        },
        TextureConfig::Marble {
            seed: None,
            frequency: Some(0.2),
        },
        TextureConfig::SolidColor { color: DVec3::new(1.0, 0.5, 0.65) },
        TextureConfig::SolidColor { color: DVec3::new(0.23, 0.51, 0.88) },
    ].into();

    let materials = vec![
        MaterialConfig::Lambertian { texture: 0 },
        MaterialConfig::Metal { fuzz: 0.05, texture: 1 },
        MaterialConfig::Lambertian { texture: 2 },
        MaterialConfig::Metal { fuzz: 0.9, texture: 3 },
        MaterialConfig::Metal { fuzz: 0.8, texture: 4 },
    ].into();

    let objects = vec![
        ObjectConfig::Sphere {
            center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
            radius: GROUND_SPHERE_RADIUS,
            material: 0,
        },
        ObjectConfig::Sphere {
            center: DVec3::new(-40., 10., 20.),
            radius: 10.,
            material: 2,
        },
        ObjectConfig::Sphere {
            center: DVec3::new(30., 10., -20.),
            radius: 10.,
            material: 3,
        },
        ObjectConfig::Sphere {
            center: DVec3::new(10., 10., 25.),
            radius: 10.,
            material: 4,
        },
    ].into();

    let mut camera = CameraConfig {
        background_color: Some(DVec3::new(0.7, 0.8, 1.0)),
        look_from: Some(70.*DVec3::X + 30.*DVec3::Y),
        look_at: Some(10.*DVec3::Y + 2.*DVec3::NEG_Z),
        field_of_view: Some(30.),
        ray_max_bounces: Some(10),
        samples_per_pixel: Some(10),
        ..CameraConfig::default()
    };

    camera.merge_with(&args.camera);

    let scene_config = SceneConfig {
        camera,
        textures,
        materials,
        objects,
    };

    let contents = match args.format {
        SceneConfigFormat::Json => serde_json::to_string_pretty(&scene_config)?,
        SceneConfigFormat::Toml => toml::to_string_pretty(&scene_config)?,
    };

    if let Some(output) = args.output.as_ref() {
        fs::write(output, &contents)?
    } else {
        io::stdout().write_all(contents.as_bytes())?;
    }

    Ok(())
}
