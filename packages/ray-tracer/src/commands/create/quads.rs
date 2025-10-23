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
    objects.push_back(ObjectConfig::Quad {
        point: DVec3::new(-3.0, -2.0, 5.0),
        u: 4.0*DVec3::NEG_Z,
        v: 4.0*DVec3::Y,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::SolidColor { color: DVec3::new(1.0, 0.2, 0.2) });

    objects.push_back(ObjectConfig::Quad {
        point: DVec3::new(-2.0, -2.0, 0.),
        u: 4.0*DVec3::X,
        v: 4.0*DVec3::Y,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::SolidColor { color: DVec3::new(0.2, 1.0, 0.2) });

    objects.push_back(ObjectConfig::Quad {
        point: DVec3::new(3.0, -2.0, 1.),
        u: 4.0*DVec3::Z,
        v: 4.0*DVec3::Y,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::SolidColor { color: DVec3::new(0.2, 0.2, 1.0) });

    objects.push_back(ObjectConfig::Quad {
        point: DVec3::new(-2.0, 3.0, 1.),
        u: 4.0*DVec3::X,
        v: 4.0*DVec3::Z,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::SolidColor { color: DVec3::new(1.0, 0.5, 0.0) });

    objects.push_back(ObjectConfig::Quad {
        point: DVec3::new(-2.0, -3.0, 5.0),
        u: 4.0*DVec3::X,
        v: 4.0*DVec3::NEG_Z,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::SolidColor { color: DVec3::new(0.2, 0.8, 0.8) });
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut textures = VecDeque::<TextureConfig>::new();
    let mut materials = VecDeque::<MaterialConfig>::new();
    let mut objects = VecDeque::<ObjectConfig>::new();

    generate_objects(&mut textures, &mut materials, &mut objects);

    let mut camera = CameraConfig {
        background_color: Some(DVec3::new(0.7, 0.8, 1.0)),
        look_from: Some(9.*DVec3::Z),
        look_at: Some(DVec3::ZERO),
        field_of_view: Some(80.),
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
