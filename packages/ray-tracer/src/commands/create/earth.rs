use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

pub fn run(args: &CreateArgs) -> Result<()> {
    const GROUND_SPHERE_RADIUS: f64 = 1000.0;

    let mut scene_config = SceneConfig::default();

    scene_config.camera
        .merge_with(&CameraConfig {
            background_color: Some(DVec3::new(0.7, 0.8, 1.0)),
            look_from: Some(DVec3::new(60.0, 20.0, 3.0)),
            look_at: Some(10.*DVec3::Y),
            field_of_view: Some(20.),
            ray_max_bounces: Some(10),
            samples_per_pixel: Some(10),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

    let ground_id = Box::<str>::from("ground");
    let earth_id = Box::<str>::from("earth");
    let moon_id = Box::<str>::from("moon");

    scene_config.textures.insert(
        ground_id.clone(),
        TextureConfig::SolidColor { color: 0.5*DVec3::ONE },
    );
    scene_config.textures.insert(
        earth_id.clone(),
        TextureConfig::Image { path: "scenes/textures/earth.jpg".into(), },
    );
    scene_config.textures.insert(
        moon_id.clone(),
        TextureConfig::Image { path: "scenes/textures/moon.jpg".into(), },
    );

    scene_config.materials.insert(
        ground_id.clone(),
        MaterialConfig::Lambertian { texture: Some(ground_id.clone()) },
    );
    scene_config.materials.insert(
        earth_id.clone(),
        MaterialConfig::Lambertian { texture: Some(earth_id.clone()) },
    );
    scene_config.materials.insert(
        moon_id.clone(),
        MaterialConfig::Lambertian { texture: Some(moon_id.clone()) },
    );

    scene_config.scene.push(ObjectConfig::Sphere {
        center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
        radius: GROUND_SPHERE_RADIUS,
        material: Some(ground_id.clone()),
    });
    scene_config.scene.push(ObjectConfig::Sphere {
        center: DVec3::new(0., 10., 0.),
        radius: 10.,
        material: Some(earth_id.clone()),
    });
    scene_config.scene.push(ObjectConfig::Sphere {
        center: DVec3::new(-12., 12., -20.),
        radius: 3.,
        material: Some(moon_id.clone()),
    });

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
