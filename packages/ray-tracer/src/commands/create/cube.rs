use std::io::Write;

use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

fn generate_cube(scene_config: &mut SceneConfig) {
    let dx = DVec3::X;
    let dy = DVec3::Y;
    let dz = DVec3::Z;

    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::ZERO,
        u: dx,
        v: dy,
        material: None,
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::ZERO,
        u: dx,
        v: dz,
        material: None,
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::ZERO,
        u: dz,
        v: dy,
        material: None,
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::X,
        u: dz,
        v: dy,
        material: None,
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::Y,
        u: dx,
        v: dz,
        material: None,
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::Z,
        u: dx,
        v: dy,
        material: None,
    });
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut scene_config = SceneConfig::default();

    scene_config.camera
        .merge_with(&CameraConfig {
            background_color: Some(DVec3::ONE),
            look_from: Some(DVec3::new(0.5, 1.0, 2.0)),
            look_at: Some(DVec3::new(0.5, 0.5, 0.0)),
            field_of_view: Some(40.),
            ray_max_bounces: Some(50),
            samples_per_pixel: Some(200),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

    generate_cube(&mut scene_config);

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
