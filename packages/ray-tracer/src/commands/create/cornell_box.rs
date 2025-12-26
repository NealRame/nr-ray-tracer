use std::io::Write;

use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

fn generate_solid_color_texture(
    scene_config: &mut SceneConfig,
    color: DVec3,
) -> Box<str> {
    let id = get_next_texture_id();

    scene_config.textures.push((
        id.clone(),
        TextureConfig::SolidColor { color },
    ));
    id
}

fn generate_lambertian_material(
    scene_config: &mut SceneConfig,
    texture: Box<str>,
) -> Box<str> {
    let id = get_next_material_id();

    scene_config.materials.push((
        id.clone(),
        MaterialConfig::Lambertian {
            texture: Some(texture),
        },
    ));
    id
}

fn generate_light(
    scene_config: &mut SceneConfig,
    texture: Box<str>,
    intensity: f64,
) -> Box<str> {
    let id = get_next_material_id();

    scene_config.materials.push((
        id.clone(),
        MaterialConfig::DiffuseLight {
            texture: Some(texture),
            intensity,
        }
    ));
    id
}

fn generate_scene(scene_config: &mut SceneConfig) {
    let tex_white = generate_solid_color_texture(scene_config, DVec3::new(0.3450980392, 0.3568627451, 0.4392156863));
    let mat_white = generate_lambertian_material(scene_config, tex_white.clone());

    let tex_green = generate_solid_color_texture(scene_config, DVec3::new(0.6509803922, 0.8901960784, 0.631372549));
    let mat_green = generate_lambertian_material(scene_config, tex_green.clone());

    let tex_red = generate_solid_color_texture(scene_config, DVec3::new(0.9529411765, 0.5450980392, 0.6588235294));
    let mat_red = generate_lambertian_material(scene_config, tex_red.clone());

    let tex_ligth = generate_solid_color_texture(scene_config, DVec3::ONE);
    let mat_ligth = generate_light(scene_config, tex_ligth.clone(), 15.0);

    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Z,
        material: Some(mat_white.clone()),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::ONE,
        u: DVec3::NEG_X,
        v: DVec3::NEG_Z,
        material: Some(mat_white.clone()),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::Z,
        u: DVec3::X,
        v: DVec3::Y,
        material: Some(mat_white.clone()),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::X,
        u: DVec3::Y,
        v: DVec3::Z,
        material: Some(mat_green),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::ZERO,
        u: DVec3::Y,
        v: DVec3::Z,
        material: Some(mat_red),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::new(0.670, 0.998, 0.598),
        u: 0.234*DVec3::NEG_X,
        v: 0.189*DVec3::NEG_Z,
        material: Some(mat_ligth),
    });
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut scene_config = SceneConfig::default();

    scene_config.camera
        .merge_with(&CameraConfig {
            background_color: Some(DVec3::ZERO),
            look_from: Some(DVec3::new(0.0, 0.5, 0.0)),
            look_at: Some(DVec3::new(0.0, 0.0, 0.0)),
            field_of_view: Some(40.),
            ray_max_bounces: Some(50),
            samples_per_pixel: Some(200),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

    generate_scene(&mut scene_config);

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
