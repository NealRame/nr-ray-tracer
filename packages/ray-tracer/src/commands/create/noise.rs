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
            look_from: Some(DVec3::new(30., 20., -60.)),
            look_at: Some(DVec3::new(20., 10., -20.)),
            field_of_view: Some(30.),
            ray_max_bounces: Some(10),
            samples_per_pixel: Some(10),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

    {
        let id: Box<str> = Box::from("ground");

        scene_config.textures.push((
            id.clone(),
            TextureConfig::SolidColor { color: 0.5*DVec3::ONE },
        ));
        scene_config.materials.push((
            id.clone(),
            MaterialConfig::Lambertian { texture: Some(id.clone()) },
        ));
        scene_config.scene.push(ObjectConfig::Sphere {
            center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
            radius: GROUND_SPHERE_RADIUS,
            material: Some(id.clone()),
        });
    } {
        let id: Box<str> = Box::from("sphere1");

        scene_config.textures.push((
            id.clone(),
            TextureConfig::Noise {
                seed: None,
                frequency: Some(0.2),
                octaves: Some(8),
                lacunarity: None,
                persistence: None,
            },
        ));
        scene_config.materials.push((
            id.clone(),
            MaterialConfig::Metal {
                texture: Some(id.clone()),
                fuzz: 0.05,
            },
        ));
        scene_config.scene.push(ObjectConfig::Sphere {
            center: DVec3::new(-30., 10., 10.),
            radius: 10.,
            material: Some(id.clone()),
        });
    } {
        let id: Box<str> = Box::from("sphere2");

        scene_config.textures.push((
            id.clone(),
            TextureConfig::Marble {
                seed: None,
                frequency: Some(0.2),
            },
        ));
        scene_config.materials.push((
            id.clone(),
            MaterialConfig::Metal {
                texture: Some(id.clone()),
                fuzz: 0.9,
            }
        ));
        scene_config.scene.push(ObjectConfig::Sphere {
            center: DVec3::new(20., 10., -20.),
            radius: 10.,
            material: Some(id.clone()),
        });
    } {
        let id: Box<str> = Box::from("sphere3");

        scene_config.textures.push((
            id.clone(),
            TextureConfig::SolidColor { color: DVec3::new(1.0, 0.5, 0.65) },
        ));
        scene_config.materials.push((
            id.clone(),
            MaterialConfig::Metal {
                texture: Some(id.clone()),
                fuzz: 0.8,
            },
        ));
        scene_config.scene.push(ObjectConfig::Sphere {
            center: DVec3::new(10., 10., 25.),
            radius: 10.,
            material: Some(id.clone()),
        });
    }

    let contents = match get_format(args.format, args.output.as_ref()) {
        SceneConfigFormat::Json => serde_json::to_string_pretty(&scene_config)?,
        SceneConfigFormat::Toml => toml::to_string_pretty(&scene_config)?,
    };

    get_output(
        args.output.as_ref(),
        args.force_overwrite,
    )?.write_all(contents.as_bytes())?;

    Ok(())
}
