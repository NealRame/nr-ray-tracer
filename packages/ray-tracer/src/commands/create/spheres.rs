use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::Write;

use anyhow::Result;

use glam::DVec3;

use itertools::Itertools;

use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;

use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;

use nr_ray_tracer_lib::prelude::*;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

fn generate_dielectric(
    materials: &mut VecDeque<MaterialConfig>
) {
    materials.push_back(MaterialConfig::Dielectric { refraction_index: 1.5 });
}

fn generate_lambertian(
    rng: &mut ChaCha8Rng,
    materials: &mut VecDeque<MaterialConfig>,
    textures: &mut VecDeque<TextureConfig>,
) {
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::SolidColor {
        color: DVec3::from_rng(rng),
    });
}

fn generate_metal(
    rng: &mut ChaCha8Rng,
    materials: &mut VecDeque<MaterialConfig>,
    textures: &mut VecDeque<TextureConfig>,
) {
    materials.push_back(MaterialConfig::Metal {
        fuzz: rng.random(),
        texture: textures.len(),
    });
    textures.push_back(TextureConfig::SolidColor {
        color: DVec3::from_rng(rng),
    });
}

fn generate_objects(
    textures: &mut VecDeque<TextureConfig>,
    materials: &mut VecDeque<MaterialConfig>,
    objects: &mut VecDeque<ObjectConfig>,
) {
    const SEED: u64 = 1;
    const GROUND_SPHERE_RADIUS: f64 = 100000.0;
    const SMALL_SPHERE_RADIUS: f64 = 0.2;
    const LARGE_SPHERE_RADIUS: f64 = 1.0;
    const MATERIAL_DISTRIBUTION: &[usize; 3] = &[5, 80, 15];

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let dist = WeightedIndex::new(MATERIAL_DISTRIBUTION).unwrap();

    objects.push_back(ObjectConfig::Sphere {
        center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
        radius: GROUND_SPHERE_RADIUS,
        material: materials.len(),
    });
    materials.push_back(MaterialConfig::Lambertian { texture: textures.len() });
    textures.push_back(TextureConfig::SolidColor { color: 0.5*DVec3::ONE });

    objects.push_back(ObjectConfig::Sphere {
        center: LARGE_SPHERE_RADIUS*DVec3::Y,
        radius: LARGE_SPHERE_RADIUS,
        material: materials.len(),
    });
    generate_dielectric(materials);

    objects.push_back(ObjectConfig::Sphere {
        center: LARGE_SPHERE_RADIUS*DVec3::Y - 4.0*DVec3::X,
        radius: LARGE_SPHERE_RADIUS,
        material: materials.len(),
    });
    generate_lambertian(&mut rng, materials, textures);

    objects.push_back(ObjectConfig::Sphere {
        center: LARGE_SPHERE_RADIUS*DVec3::Y + 4.0*DVec3::X,
        radius: LARGE_SPHERE_RADIUS,
        material: materials.len(),
    });
    generate_metal(&mut rng, materials, textures);

    for (a, b) in Itertools::cartesian_product(-11..11, -11..11) {
        let radius = SMALL_SPHERE_RADIUS;
        let center = DVec3::new(
            a as f64 + 0.9*rng.random::<f64>(),
            SMALL_SPHERE_RADIUS,
            b as f64 + 0.9*rng.random::<f64>(),
        );

        objects.push_back(ObjectConfig::Sphere {
            center,
            radius,
            material: materials.len(),
        });

        match dist.sample(&mut rng) {
            0 => generate_dielectric(materials),
            1 => generate_lambertian(&mut rng, materials, textures),
            _ => generate_metal(&mut rng, materials, textures),
        };
    }
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut textures = VecDeque::<TextureConfig>::new();
    let mut materials = VecDeque::<MaterialConfig>::new();
    let mut objects = VecDeque::<ObjectConfig>::new();

    generate_objects(&mut textures, &mut materials, &mut objects);

    let mut camera = CameraConfig {
        background_color: Some(DVec3::new(0.7, 0.8, 1.0)),
        look_from: Some(13.*DVec3::X + 2.*DVec3::Y + 3.*DVec3::Z),
        look_at: Some(DVec3::ZERO),
        field_of_view: Some(20.),
        focus_distance: Some(10.0),
        defocus_angle: Some(0.5),
        ray_max_bounces: Some(50),
        samples_per_pixel: Some(100),
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
