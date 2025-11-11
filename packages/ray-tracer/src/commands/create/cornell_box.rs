use std::f64::consts::PI;
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

    scene_config.textures.insert(
        id.clone(),
        TextureConfig::SolidColor { color },
    );
    id
}

fn generate_lambertian_material(
    scene_config: &mut SceneConfig,
    texture: Box<str>,
) -> Box<str> {
    let id = get_next_material_id();

    scene_config.materials.insert(
        id.clone(),
        MaterialConfig::Lambertian {
            texture: Some(texture),
        },
    );
    id
}

fn generate_metal_material(
    scene_config: &mut SceneConfig,
    texture: Box<str>,
    fuzz: f64,
) -> Box<str> {
    let id = get_next_material_id();

    scene_config.materials.insert(
        id.clone(),
        MaterialConfig::Metal {
            texture: Some(texture),
            fuzz,
        },
    );
    id
}

fn generate_light(
    scene_config: &mut SceneConfig,
    texture: Box<str>,
    intensity: f64,
) -> Box<str> {
    let id = get_next_material_id();

    scene_config.materials.insert(
        id.clone(),
        MaterialConfig::DiffuseLight {
            texture: Some(texture),
            intensity,
        }
    );
    id
}

fn generate_box(
    a: DVec3,
    b: DVec3,
    material: Box<str>,
) -> Box<ObjectConfig> {
    let [min_x, min_y, min_z] = a.min(b).to_array();
    let [max_x, max_y, max_z] = a.max(b).to_array();

    let dx = (max_x - min_x)*DVec3::X;
    let dy = (max_y - min_y)*DVec3::Y;
    let dz = (max_z - min_z)*DVec3::Z;

    let objects = vec![
        ObjectConfig::Quad {

            point: DVec3::new(min_x, min_y, max_z),
            u: dx,
            v: dy,
            material: Some(material.clone()),
        },
        ObjectConfig::Quad {

            point: DVec3::new(max_x, min_y, max_z),
            u: -dz,
            v: dy,
            material: Some(material.clone()),
        },
        ObjectConfig::Quad {

            point: DVec3::new(max_x, min_y, min_z),
            u: -dx,
            v: dy,
            material: Some(material.clone()),
        },
        ObjectConfig::Quad {

            point: DVec3::new(min_x, min_y, min_z),
            u: dz,
            v: dy,
            material: Some(material.clone()),
        },
        ObjectConfig::Quad {

            point: DVec3::new(min_x, max_y, max_z),
            u: dx,
            v: -dz,
            material: Some(material.clone()),
        },
        ObjectConfig::Quad {

            point: DVec3::new(min_x, min_y, min_z),
            u: dx,
            v: dz,
            material: Some(material.clone()),
        },
    ];

    Box::new(ObjectConfig::Group {
        objects,
        material: None,
    })
}

fn generate_scene(scene_config: &mut SceneConfig) {
    let tex_white = generate_solid_color_texture(scene_config, DVec3::new(0.73, 0.73, 0.73));
    let mat_white = generate_lambertian_material(scene_config, tex_white.clone());

    let tex_green = generate_solid_color_texture(scene_config, DVec3::new(0.12, 0.45, 0.15));
    let mat_green = generate_lambertian_material(scene_config, tex_green.clone());

    let tex_red = generate_solid_color_texture(scene_config, DVec3::new(0.65, 0.05, 0.05));
    let mat_red = generate_lambertian_material(scene_config, tex_red.clone());

    let tex_ligth = generate_solid_color_texture(scene_config, DVec3::ONE);
    let mat_ligth = generate_light(scene_config, tex_ligth.clone(), 15.0);

    let tex_box1 = generate_solid_color_texture(scene_config, DVec3::new(0.93, 1.00, 0.60));
    let mat_box1 = generate_lambertian_material(scene_config, tex_box1.clone());

    let tex_box2 = generate_solid_color_texture(scene_config, DVec3::new(0.00, 0.82, 1.00));
    let mat_box2 = generate_metal_material(scene_config, tex_box2.clone(), 0.0);

    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::ZERO,
        u: 555.*DVec3::X,
        v: 555.*DVec3::Z,
        material: Some(mat_white.clone()),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: 555.*DVec3::ONE,
        u: 555.*DVec3::NEG_X,
        v: 555.*DVec3::NEG_Z,
        material: Some(mat_white.clone()),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: 555.*DVec3::Z,
        u: 555.*DVec3::X,
        v: 555.*DVec3::Y,
        material: Some(mat_white.clone()),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: 555.*DVec3::X,
        u: 555.*DVec3::Y,
        v: 555.*DVec3::Z,
        material: Some(mat_green),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::ZERO,
        u: 555.*DVec3::Y,
        v: 555.*DVec3::Z,
        material: Some(mat_red),
    });
    scene_config.scene.push(ObjectConfig::Quad {
        point: DVec3::new(343., 554., 332.),
        u: 130.*DVec3::NEG_X,
        v: 105.*DVec3::NEG_Z,
        material: Some(mat_ligth),
    });

    scene_config.scene.push(ObjectConfig::Translate {
        offset: DVec3::new(130.0, 0.0, 65.0),
        object: Box::new(ObjectConfig::RotateY {
            angle: -18.0*PI/180.0,
            object: generate_box(
                DVec3::ZERO,
                DVec3::new(165.0, 165.0, 165.0),
                mat_box1,
            ),
        }),
    });

    scene_config.scene.push(ObjectConfig::Translate {
        offset: DVec3::new(265.0, 0.0, 295.0),
        object: Box::new(ObjectConfig::RotateY {
            angle: 15.0*PI/180.0,
            object: generate_box(
                DVec3::ZERO,
                DVec3::new(165.0, 330.0, 165.0),
                mat_box2,
            ),
        }),
    });
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut scene_config = SceneConfig::default();

    scene_config.camera
        .merge_with(&CameraConfig {
            background_color: Some(DVec3::ZERO),
            look_from: Some(278.*DVec3::X + 278.*DVec3::Y - 800.*DVec3::Z),
            look_at: Some(278.*(DVec3::X + DVec3::Y)),
            field_of_view: Some(40.),
            ray_max_bounces: Some(50),
            samples_per_pixel: Some(200),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

    generate_scene(&mut scene_config);

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
