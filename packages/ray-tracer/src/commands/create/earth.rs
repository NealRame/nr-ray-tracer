use std::fs;
use std::io;
use std::io::Write;

use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

pub fn run(args: &CreateArgs) -> Result<()> {
    const GROUND_SPHERE_RADIUS: f64 = 1000.0;

    let textures = vec![
        TextureConfig::SolidColor { color: 0.5*DVec3::ONE },
        TextureConfig::Image { path: "scenes/textures/earth.jpg".into(), },
        TextureConfig::Image { path: "scenes/textures/moon.jpg".into(), },
    ].into();

    let materials = vec![
        MaterialConfig::Lambertian { texture: 0 },
        MaterialConfig::Lambertian { texture: 1 },
        MaterialConfig::Lambertian { texture: 2 },
    ].into();

    let objects = vec![
        ObjectConfig::Sphere {
            center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
            radius: GROUND_SPHERE_RADIUS,
            material: 0,
        },
        ObjectConfig::Sphere {
            center: DVec3::new(0., 10., 0.),
            radius: 10.,
            material: 1,
        },
        ObjectConfig::Sphere {
            center: DVec3::new(-12., 12., -20.),
            radius: 3.,
            material: 2,
        },
    ].into();

    let mut camera = CameraConfig {
        background_color: Some(DVec3::new(0.7, 0.8, 1.0)),
        look_from: Some(DVec3::new(60.0, 20.0, 3.0)),
        look_at: Some(10.*DVec3::Y),
        field_of_view: Some(20.),
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
