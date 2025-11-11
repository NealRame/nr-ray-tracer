use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

fn generate_textures(scene_config: &mut SceneConfig) {
    scene_config.textures.insert(
        Box::from("solid_red"),
        TextureConfig::SolidColor { color: DVec3::new(1.0, 0.2, 0.2) },
    );
    scene_config.textures.insert(
        Box::from("solid_green"),
        TextureConfig::SolidColor { color: DVec3::new(0.2, 1.0, 0.2) },
    );
    scene_config.textures.insert(
        Box::from("solid_blue"),
        TextureConfig::SolidColor { color: DVec3::new(0.2, 0.2, 1.0) },
    );
    scene_config.textures.insert(
        Box::from("solid_orange"),
        TextureConfig::SolidColor { color: DVec3::new(1.0, 0.5, 0.0) },
    );
    scene_config.textures.insert(
        Box::from("solid_cyan"),
        TextureConfig::SolidColor { color: DVec3::new(0.2, 0.8, 0.8) },
    );
}

fn generate_materials(scene_config: &mut SceneConfig) {
    scene_config.materials.insert(
        Box::from("lambertian_red"),
        MaterialConfig::Lambertian { texture: Some(Box::from("solid_red")) },
    );
    scene_config.materials.insert(
        Box::from("lambertian_green"),
        MaterialConfig::Lambertian { texture: Some(Box::from("solid_green")) }
    );
    scene_config.materials.insert(
        Box::from("lambertian_blue"),
        MaterialConfig::Lambertian { texture: Some(Box::from("solid_blue")) },
    );
    scene_config.materials.insert(
        Box::from("lambertian_orange"),
        MaterialConfig::Lambertian { texture: Some(Box::from("solid_orange")) },
    );
    scene_config.materials.insert(
        Box::from("lambertian_cyan"),
        MaterialConfig::Lambertian { texture: Some(Box::from("solid_cyan")) },
    );
}

fn generate_objects(scene_config: &mut SceneConfig) {
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::new(-3.0, -2.0, 5.0),
        u: 4.0*DVec3::NEG_Z,
        v: 4.0*DVec3::Y,
        material: Some(Box::from("lambertian_red")),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::new(-2.0, -2.0, 0.),
        u: 4.0*DVec3::X,
        v: 4.0*DVec3::Y,
        material: Some(Box::from("lambertian_green")),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::new(3.0, -2.0, 1.),
        u: 4.0*DVec3::Z,
        v: 4.0*DVec3::Y,
        material: Some(Box::from("lambertian_blue")),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::new(-2.0, 3.0, 1.),
        u: 4.0*DVec3::X,
        v: 4.0*DVec3::Z,
        material: Some(Box::from("lambertian_orange")),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::new(-2.0, -3.0, 5.0),
        u: 4.0*DVec3::X,
        v: 4.0*DVec3::NEG_Z,
        material: Some(Box::from("lambertian_cyan")),
    });
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut scene_config = SceneConfig::default();

    scene_config.camera
        .merge_with(&CameraConfig {
            background_color: Some(DVec3::new(0.7, 0.8, 1.0)),
            look_from: Some(9.*DVec3::Z),
            look_at: Some(DVec3::ZERO),
            field_of_view: Some(80.),
            ray_max_bounces: Some(10),
            samples_per_pixel: Some(10),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

    generate_textures(&mut scene_config);
    generate_materials(&mut scene_config);
    generate_objects(&mut scene_config);

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
