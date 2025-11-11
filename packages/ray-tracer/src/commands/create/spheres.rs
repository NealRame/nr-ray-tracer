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
    scene_config: &mut SceneConfig,
) -> Box<str> {
    let id = get_next_material_id();

    scene_config.materials.insert(
        id.clone(),
        MaterialConfig::Dielectric { refraction_index: 1.5 },
    );
    id
}

fn generate_lambertian(
    scene_config: &mut SceneConfig,
    color: DVec3,
) -> Box<str> {
    let mat_id = get_next_material_id();
    let tex_id = get_next_texture_id();

    scene_config.textures.insert(
        tex_id.clone(),
        TextureConfig::SolidColor { color },
    );
    scene_config.materials.insert(
        mat_id.clone(),
        MaterialConfig::Lambertian {
            texture: Some(tex_id),
        },
    );
    mat_id
}

fn generate_metal(
    scene_config: &mut SceneConfig,
    color: DVec3,
    fuzz: f64,
) -> Box<str> {
    let mat_id = get_next_material_id();
    let tex_id = get_next_texture_id();

    scene_config.textures.insert(
        tex_id.clone(),
        TextureConfig::SolidColor { color },
    );
    scene_config.materials.insert(
        mat_id.clone(),
        MaterialConfig::Metal {
            fuzz,
            texture: Some(tex_id),
        },
    );
    mat_id
}

fn generate_objects(scene_config: &mut SceneConfig, seed: u64) {
    const GROUND_SPHERE_RADIUS: f64 = 100000.0;
    const SMALL_SPHERE_RADIUS: f64 = 0.2;
    const LARGE_SPHERE_RADIUS: f64 = 1.0;
    const MATERIAL_DISTRIBUTION: &[usize; 3] = &[5, 80, 15];

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let dist = WeightedIndex::new(MATERIAL_DISTRIBUTION).unwrap();

    {
        let material = Some(generate_lambertian(scene_config, 0.5*DVec3::ONE));
        scene_config.scene.push(ObjectConfig::Sphere {
            center: GROUND_SPHERE_RADIUS*DVec3::NEG_Y,
            radius: GROUND_SPHERE_RADIUS,
            material,
        });
    } {
        let material = Some(generate_dielectric(scene_config));
        scene_config.scene.push(ObjectConfig::Sphere {
            center: LARGE_SPHERE_RADIUS*DVec3::Y,
            radius: LARGE_SPHERE_RADIUS,
            material,
        });
    } {
        let color = DVec3::from_rng(&mut rng);
        let material = Some(generate_lambertian(scene_config, color));

        scene_config.scene.push(ObjectConfig::Sphere {
            center: LARGE_SPHERE_RADIUS*DVec3::Y - 4.0*DVec3::X,
            radius: LARGE_SPHERE_RADIUS,
            material,
        });
    } {
        let color = DVec3::from_rng(&mut rng);
        let fuzz = rng.random();
        let material = Some(generate_metal(scene_config, color, fuzz));

        scene_config.scene.push(ObjectConfig::Sphere {
            center: LARGE_SPHERE_RADIUS*DVec3::Y + 4.0*DVec3::X,
            radius: LARGE_SPHERE_RADIUS,
            material,
        });
    }

    for (a, b) in Itertools::cartesian_product(-11..11, -11..11) {
        let radius = SMALL_SPHERE_RADIUS;
        let center = DVec3::new(
            a as f64 + 0.9*rng.random::<f64>(),
            SMALL_SPHERE_RADIUS,
            b as f64 + 0.9*rng.random::<f64>(),
        );

        let material = Some(match dist.sample(&mut rng) {
            0 => generate_dielectric(scene_config),
            1 => generate_lambertian(scene_config, DVec3::from_rng(&mut rng)),
            _ => generate_metal(scene_config, DVec3::from_rng(&mut rng), rng.random()),
        });

        scene_config.scene.push(ObjectConfig::Sphere {
            center,
            radius,
            material,
        });
    }
}

pub fn run(args: &CreateArgs) -> Result<()> {
    let mut scene_config = SceneConfig::default();

    scene_config.camera
        .merge_with(&CameraConfig {
            background_color: Some(DVec3::new(0.7, 0.8, 1.0)),
            look_from: Some(13.*DVec3::X + 2.*DVec3::Y + 3.*DVec3::Z),
            look_at: Some(DVec3::ZERO),
            field_of_view: Some(20.),
            focus_distance: Some(10.0),
            defocus_angle: Some(0.5),
            ray_max_bounces: Some(10),
            samples_per_pixel: Some(10),
            ..CameraConfig::default()
        })
        .merge_with(&args.camera);

    generate_objects(&mut scene_config, args.seed);

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
