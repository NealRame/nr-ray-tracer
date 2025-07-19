mod cli;
mod constants;

use std::f64::consts::PI;
use std::fs;

use chrono::Utc;

use clap::{
    CommandFactory,
    Parser,
};
use clap::error::ErrorKind;

use image::{
    DynamicImage,
    ImageFormat,
    Rgb32FImage,
};

use indicatif::ProgressStyle;

use nr_ray_tracer_lib::prelude::*;

use crate::cli::*;
use crate::constants::*;

fn load_scene(
    cli: &Cli,
) -> Scene {
    let image_size = cli.image.get_size();

    let content =
        fs::read_to_string(&cli.scene)
            .unwrap_or_else(|err| {
                Cli::command().error(ErrorKind::Io, format!(
                    "Fail to read '{}' content. {}.",
                    cli.scene.to_string_lossy(),
                    err.to_string(),
                )).exit();
            })
        ;

    let scene_config_ext =
        cli.scene.extension()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_lowercase());

    let mut scene_config: SceneConfig =
        match scene_config_ext.as_ref().map(|s| s.as_str()) {
            Some("json") => {
                serde_json::from_str(&content).unwrap_or_else(|err| {
                    Cli::command().error(ErrorKind::InvalidValue, format!(
                        "Fail to load scene '{}' content. {}.",
                        cli.scene.to_string_lossy(),
                        err.to_string(),
                    )).exit()
                })
            },
            Some("toml") => {
                toml::from_str(&content).unwrap_or_else(|err| {
                    Cli::command().error(ErrorKind::InvalidValue, format!(
                        "Fail to load scene '{}' content. {}.",
                        cli.scene.to_string_lossy(),
                        err.to_string(),
                    )).exit()
                })
            },
            _ => {
                Cli::command().error(ErrorKind::Io, format!(
                    "Unsupported scene file format '{}'.",
                    cli.scene.to_string_lossy(),
                )).exit();
            }
        };

    scene_config.camera
        .with_image_size(image_size)
        .with_samples_per_pixel(cli.image.samples_per_pixel)
        .with_ray_max_bounce(cli.image.ray_max_bounce)
        .with_field_of_view(cli.camera.field_of_view*(PI/180.))
        .with_focus_dist(cli.camera.focus_distance)
        .with_defocus_angle(cli.camera.defocus_angle*(PI/180.))
    ;

    if let Some(value) = cli.camera.look_at {
        scene_config.camera.with_look_at(value);
    }

    if let Some(value) = cli.camera.look_from {
        scene_config.camera.with_look_from(value);
    }

    if let Some(value) = cli.camera.view_up {
        scene_config.camera.with_view_up(value);
    }

    Scene::from(scene_config)
}

fn render_scene(
    cli: &Cli,
    scene: &Scene,
) -> Rgb32FImage {
    let bar = cli.get_progress("Rendering").map(|bar| {
        bar.set_position(0);
        bar.set_length(scene.camera.get_image_size().get_pixel_count() as u64);
        bar
    });

    let start = Utc::now();

    let image = scene.render(bar.as_ref().map(|bar| || bar.inc(1)));

    let stop = Utc::now();
    let duration = stop - start;

    if let Some(bar) = bar.as_ref() {
        bar.set_style(ProgressStyle::with_template(PROGRESS_TEMPLATE_FINISHED).unwrap());
        bar.finish_with_message(format!("Done in {}.{:0<3} secs",
            duration.num_seconds(),
            duration.num_milliseconds()%1000,
        ));
    }
    image
}

fn dump_image(
    cli: &Cli,
    file: &mut fs::File,
    mut image: Rgb32FImage,
    image_format: ImageFormat,
) {
    let start = Utc::now();
    let progress = cli.get_spinner("Exporting");

    gamma_correction(&mut image, cli.image.gamma_value);

    DynamicImage::ImageRgb32F(image)
        .to_rgb8()
        .write_to(file, image_format)
        .unwrap_or_else(|err| {
            Cli::command().error(ErrorKind::Io, format!(
                "Fail to write image. {}.",
                err.to_string(),
            )).exit();
        });

    let stop = Utc::now();
    let duration = stop - start;

    // rgb_image
    if let Some(bar) = progress.as_ref() {
        bar.set_style(ProgressStyle::with_template(PROGRESS_TEMPLATE_FINISHED).unwrap());
        bar.finish_with_message(format!("Done in {}.{:0<3} secs",
            duration.num_seconds(),
            duration.num_milliseconds()%1000,
        ));
    }
}

fn main() {
    let cli = Cli::parse();

    let (mut file, format) = cli.get_file();

    let scene = load_scene(&cli);
    let image = render_scene(&cli, &scene);

    dump_image(&cli, &mut file, image, format);

    // let camera =
    //     CameraConfig::default()
    //         .with_look_at(DVec3::new(0., 0., 0.))
    //         .with_look_from(DVec3::new(8.0, 8.0, 8.0))
    //         .with_samples_per_pixel(500)
    //         .with_ray_max_bounce(50)
    //         .clone()
    //         ;

    // let textures = vec![
    //     Texture::SolidColor(DVec3::new(3./255., 155./255., 229./255.)),
    //     Texture::SolidColor(DVec3::new(229./255., 57./255., 53./255.)),
    // ];

    // let materials = vec![
    //     Material::Metal { texture: 0, fuzz: 0.5, },
    //     Material::Metal { texture: 1, fuzz: 0.,  },
    // ];

    // let scene = Scene::from(SceneConfig {
    //     camera,
    //     objects: vec![
    //         Object::Sphere(Sphere::new(
    //             DVec3::new(0.0, -1000.0, 0.0),
    //             1000.0,
    //             0,
    //         )),
    //         Object::Sphere(Sphere::new(
    //             DVec3::new(0., 4., 0.),
    //             4.0,
    //             1,
    //         )),
    //     ],
    //     materials,
    //     textures,
    // });

    // let image = render_scene(&cli, &scene);
    // let mut file =
    //     fs::File::options()
    //         .create(true)
    //         .truncate(true)
    //         .write(true)
    //         .open("out.png")
    //         .expect("Failed");

    // dump_image(&cli, &mut file, image, ImageFormat::Png);

    // let json_s = serde_json::to_string_pretty(&scene).unwrap();

    // stdout().write(json_s.as_bytes()).unwrap();
}
