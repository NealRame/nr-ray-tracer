use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

pub fn run(args: &CreateArgs) -> Result<()> {
    const GROUND_SPHERE_RADIUS: f64 = 1_000_000.0;

    let mut scene_config = SceneConfig::default();

    scene_config.camera
        .merge_with(&CameraConfig {
            background_color: Some(DVec3::new(0.7, 0.8, 1.0)),
            look_from: Some(70.*DVec3::X + 30.*DVec3::Y),
            look_at: Some(10.*DVec3::Y + 2.*DVec3::NEG_Z),
            field_of_view: Some(30.),
            ray_max_bounces: Some(10),
            samples_per_pixel: Some(10),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

    {
        let id: Box<str> = Box::from("ground");

        scene_config.textures.insert(
            id.clone(),
            TextureConfig::SolidColor { color: 0.5*DVec3::ONE },
        );
        scene_config.materials.insert(
            id.clone(),
            MaterialConfig::Lambertian { texture: id.clone() },
        );
        scene_config.scene.push(ObjectConfig::Sphere {
            center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
            radius: GROUND_SPHERE_RADIUS,
            material: id.clone(),
        });
    } {
        let id: Box<str> = Box::from("sphere1");

        scene_config.textures.insert(
            id.clone(),
            TextureConfig::Noise {
                seed: None,
                frequency: Some(0.2),
                octaves: Some(8),
                lacunarity: None,
                persistence: None,
            },
        );
        scene_config.materials.insert(
            id.clone(),
            MaterialConfig::Metal { texture: id.clone(), fuzz: 0.05 },
        );
        scene_config.scene.push(ObjectConfig::Sphere {
            center: DVec3::new(-40., 10., 20.),
            radius: 10.,
            material: id.clone(),
        });
    } {
        let id: Box<str> = Box::from("sphere2");

        scene_config.textures.insert(
            id.clone(),
            TextureConfig::Marble {
                seed: None,
                frequency: Some(0.2),
            },
        );
        scene_config.materials.insert(
            id.clone(),
            MaterialConfig::Metal { texture: id.clone(), fuzz: 0.9 }
        );
        scene_config.scene.push(ObjectConfig::Sphere {
            center: DVec3::new(30., 10., -20.),
            radius: 10.,
            material: id.clone(),
        });
    } {
        let id: Box<str> = Box::from("sphere3");

        scene_config.textures.insert(
            id.clone(),
            TextureConfig::SolidColor { color: DVec3::new(1.0, 0.5, 0.65) },
        );
        scene_config.materials.insert(
            id.clone(),
            MaterialConfig::Metal { texture: id.clone(), fuzz: 0.8 },
        );
        scene_config.scene.push(ObjectConfig::Sphere {
            center: DVec3::new(10., 10., 25.),
            radius: 10.,
            material: id.clone(),
        });
    }

    let contents = match args.format {
        SceneConfigFormat::Json => serde_json::to_string_pretty(&scene_config)?,
        SceneConfigFormat::Toml => toml::to_string_pretty(&scene_config)?,
    };

    get_output(
        args.output.as_ref(),
        args.force_overwrite,
    )?.write_all(contents.as_bytes())?;

    Ok(())
}
