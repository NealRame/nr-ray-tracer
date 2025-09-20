use std::collections::VecDeque;
use std::f64::consts::PI;
use std::fs;
use std::io;
use std::io::Write;


use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

fn generate_solid_color_texture(
    color: DVec3,
    textures: &mut VecDeque<TextureConfig>
) -> usize {
    textures.push_back(TextureConfig::SolidColor { color });
    textures.len() - 1
}

fn generate_lambertian_material(
    texture: usize,
    materials: &mut VecDeque<MaterialConfig>,
) -> usize {
    materials.push_back(MaterialConfig::Lambertian { texture });
    materials.len() - 1
}

fn generate_metal_material(
    texture: usize,
    fuzz: f64,
    materials: &mut VecDeque<MaterialConfig>,
) -> usize {
    materials.push_back(MaterialConfig::Metal { fuzz, texture });
    materials.len() - 1
}

fn generate_light(
    color: DVec3,
    textures: &mut VecDeque<TextureConfig>,
    materials: &mut VecDeque<MaterialConfig>,
) -> usize {
    let texture = generate_solid_color_texture(color, textures);
    materials.push_back(MaterialConfig::DiffuseLight { texture });
    materials.len() - 1
}

fn generate_box(a: DVec3, b: DVec3, material: usize) -> Vec<ObjectConfig> {
    let [min_x, min_y, min_z] = a.min(b).to_array();
    let [max_x, max_y, max_z] = a.max(b).to_array();

    let dx = (max_x - min_x)*DVec3::X;
    let dy = (max_y - min_y)*DVec3::Y;
    let dz = (max_z - min_z)*DVec3::Z;

    vec![
        ObjectConfig::Group { count: 6 },
        ObjectConfig::Quad {
            top_left: DVec3::new(min_x, min_y, max_z),
            u: dx,
            v: dy,
            material
        },
        ObjectConfig::Quad {
            top_left: DVec3::new(max_x, min_y, max_z),
            u: -dz,
            v: dy,
            material
        },
        ObjectConfig::Quad {
            top_left: DVec3::new(max_x, min_y, min_z),
            u: -dx,
            v: dy,
            material
        },
        ObjectConfig::Quad {
            top_left: DVec3::new(min_x, min_y, min_z),
            u: dz,
            v: dy,
            material
        },
        ObjectConfig::Quad {
            top_left: DVec3::new(min_x, max_y, max_z),
            u: dx,
            v: -dz,
            material
        },
        ObjectConfig::Quad {
            top_left: DVec3::new(min_x, min_y, min_z),
            u: dx,
            v: dz,
            material
        },
    ]
}

fn generate_objects(
    textures: &mut VecDeque<TextureConfig>,
    materials: &mut VecDeque<MaterialConfig>,
    objects: &mut VecDeque<ObjectConfig>,
) {
    let mat_white = generate_lambertian_material(generate_solid_color_texture(
        DVec3::new(0.73, 0.73, 0.73),
        textures
    ), materials);

    let mat_green = generate_lambertian_material(generate_solid_color_texture(
        DVec3::new(0.12, 0.45, 0.15),
        textures,
    ), materials);

    let mat_red = generate_lambertian_material(generate_solid_color_texture(
        DVec3::new(0.65, 0.05, 0.05),
        textures,
    ), materials);

    let mat_ligth = generate_light(15.0*DVec3::ONE, textures, materials);

    let mat_box1 = generate_lambertian_material(generate_solid_color_texture(
        DVec3::new(0.93, 1.00, 0.60),
        textures,
    ), materials);

    let mat_box2 = generate_metal_material(generate_solid_color_texture(
        DVec3::new(0.00, 0.82, 1.00),
        textures,
    ), 0.0, materials,);

    objects.push_back(ObjectConfig::Quad {
        top_left: DVec3::ZERO,
        u: 555.*DVec3::X,
        v: 555.*DVec3::Z,
        material: mat_white,
    });
    objects.push_back(ObjectConfig::Quad {
        top_left: 555.*DVec3::ONE,
        u: 555.*DVec3::NEG_X,
        v: 555.*DVec3::NEG_Z,
        material: mat_white,
    });
    objects.push_back(ObjectConfig::Quad {
        top_left: 555.*DVec3::Z,
        u: 555.*DVec3::X,
        v: 555.*DVec3::Y,
        material: mat_white,
    });
    objects.push_back(ObjectConfig::Quad {
        top_left: 555.*DVec3::X,
        u: 555.*DVec3::Y,
        v: 555.*DVec3::Z,
        material: mat_green,
    });
    objects.push_back(ObjectConfig::Quad {
        top_left: DVec3::ZERO,
        u: 555.*DVec3::Y,
        v: 555.*DVec3::Z,
        material: mat_red,
    });
    objects.push_back(ObjectConfig::Quad {
        top_left: DVec3::new(343., 554., 332.),
        u: 130.*DVec3::NEG_X,
        v: 105.*DVec3::NEG_Z,
        material: mat_ligth,
    });

    objects.push_back(ObjectConfig::Translate { offset: DVec3::new(130.0, 0.0, 65.0) });
    objects.push_back(ObjectConfig::RotateY { angle: -18.0*PI/180.0 });
    objects.extend(generate_box(
        DVec3::ZERO,
        DVec3::new(165.0, 165.0, 165.0),
        mat_box1,
    ));

    objects.push_back(ObjectConfig::Translate { offset: DVec3::new(265.0, 0.0, 295.0) });
    objects.push_back(ObjectConfig::RotateY { angle: 15.0*PI/180.0 });
    objects.extend(generate_box(
        DVec3::ZERO,
        DVec3::new(165.0, 330.0, 165.0),
        mat_box2,
    ));
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut textures = VecDeque::<TextureConfig>::new();
    let mut materials = VecDeque::<MaterialConfig>::new();
    let mut objects = VecDeque::<ObjectConfig>::new();

    generate_objects(&mut textures, &mut materials, &mut objects);

    let mut camera = CameraConfig {
        background_color: Some(DVec3::ZERO),
        look_from: Some(278.*DVec3::X + 278.*DVec3::Y - 800.*DVec3::Z),
        look_at: Some(278.*(DVec3::X + DVec3::Y)),
        field_of_view: Some(40.),
        ray_max_bounces: Some(50),
        samples_per_pixel: Some(200),
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
