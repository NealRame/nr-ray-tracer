use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::Write;

use anyhow::Result;

use glam::DVec3;


use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

fn generate_objects(
    textures: &mut VecDeque<TextureConfig>,
    materials: &mut VecDeque<MaterialConfig>,
    objects: &mut VecDeque<ObjectConfig>,
) {
    const GROUND_SPHERE_RADIUS: f64 = 1_000_000.0;
    const SMALL_SPHERE_RADIUS: f64 = 2.0;

    objects.push_back(ObjectConfig::Sphere {
        center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
        radius: GROUND_SPHERE_RADIUS,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::SolidColor { color: 0.4*DVec3::ONE });

    objects.push_back(ObjectConfig::Sphere {
        center: SMALL_SPHERE_RADIUS*DVec3::Y,
        radius: SMALL_SPHERE_RADIUS,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::Marble { seed: None, frequency: Some(0.2) });

    objects.push_back(ObjectConfig::Quad {
        top_left: DVec3::new(3.0, 1.0, -2.0),
        u: 2.0*DVec3::X,
        v: 2.0*DVec3::Y,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::DiffuseLight {
        intensity: 4.0,
        texture: textures.len(),
    });
    textures.push_back(TextureConfig::SolidColor { color: DVec3::new(1.00, 0.50, 0.25) });

    objects.push_back(ObjectConfig::Sphere {
        center: 7.0*DVec3::Y,
        radius: 1.0,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::DiffuseLight {
        intensity: 4.0,
        texture: textures.len(),
    });
    textures.push_back(TextureConfig::SolidColor { color: DVec3::new(0.25, 0.50, 1.00) });
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut textures = VecDeque::<TextureConfig>::new();
    let mut materials = VecDeque::<MaterialConfig>::new();
    let mut objects = VecDeque::<ObjectConfig>::new();

    generate_objects(&mut textures, &mut materials, &mut objects);

    let mut camera = CameraConfig {
        background_color: Some(0.001*DVec3::ONE),
        look_from: Some(DVec3::new(26.0, 3.0, 6.0)),
        look_at: Some(2.0*DVec3::Y),
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
