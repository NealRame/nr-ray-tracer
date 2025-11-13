use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

fn generate_lambertian(
    scene_config: &mut SceneConfig,
    color: DVec3,
) -> Box<str> {
    let tex_id = get_next_texture_id();
    let mat_id = get_next_material_id();

    scene_config.textures.push((
        tex_id.clone(),
       TextureConfig::SolidColor { color },
    ));
    scene_config.materials.push((
        mat_id.clone(),
        MaterialConfig::Lambertian {
            texture: Some(tex_id),
        },
    ));
    mat_id
}

fn generate_marble(
    scene_config: &mut SceneConfig,
) -> Box<str> {
    let tex_id = get_next_texture_id();
    let mat_id = get_next_material_id();

    scene_config.textures.push((
        tex_id.clone(),
        TextureConfig::Marble { seed: None, frequency: Some(0.2) },
    ));
    scene_config.materials.push((
        mat_id.clone(),
        MaterialConfig::Lambertian {
            texture: Some(tex_id),
        },
    ));
    mat_id
}

fn generate_light(
    scene_config: &mut SceneConfig,
    color: DVec3,
    intensity: f64,
) -> Box<str> {
    let tex_id = get_next_texture_id();
    let mat_id = get_next_material_id();

    scene_config.textures.push((
        tex_id.clone(),
        TextureConfig::SolidColor { color },
    ));
    scene_config.materials.push((
        mat_id.clone(),
        MaterialConfig::DiffuseLight {
            intensity,
            texture: Some(tex_id),
        },
    ));
    mat_id
}

fn generate_objects(
    scene_config: &mut SceneConfig,
) {
    const GROUND_SPHERE_RADIUS: f64 = 1_000_000.0;
    const SMALL_SPHERE_RADIUS: f64 = 2.0;

    {
        let material = Some(generate_lambertian(scene_config, 0.4*DVec3::ONE));

        scene_config.scene.push(ObjectConfig::Sphere {
            center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
            radius: GROUND_SPHERE_RADIUS,
            material,
        });
    } {
        let material = Some(generate_marble(scene_config));

        scene_config.scene.push(ObjectConfig::Sphere {
            center: SMALL_SPHERE_RADIUS*DVec3::Y,
            radius: SMALL_SPHERE_RADIUS,
            material,
        });
    } {
        let material = Some(generate_light(scene_config, DVec3::new(1.00, 0.50, 0.25), 4.0));

        scene_config.scene.push(ObjectConfig::Quad {
            point: DVec3::new(3.0, 1.0, -2.0),
            u: 2.0*DVec3::X,
            v: 2.0*DVec3::Y,
            material,
        });
    } {
        let material = Some(generate_light(scene_config, DVec3::new(0.25, 0.50, 1.00), 4.0));

        scene_config.scene.push(ObjectConfig::Sphere {
            center: 7.0*DVec3::Y,
            radius: 1.0,
            material,
        });
    }
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut scene_config = SceneConfig::default();

    scene_config.camera
        .merge_with(&CameraConfig {
            background_color: Some(0.001*DVec3::ONE),
            look_from: Some(DVec3::new(26.0, 3.0, 6.0)),
            look_at: Some(2.0*DVec3::Y),
            field_of_view: Some(20.),
            ray_max_bounces: Some(10),
            samples_per_pixel: Some(10),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

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
