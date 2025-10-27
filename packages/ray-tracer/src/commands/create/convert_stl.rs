use std::collections::VecDeque;
use std::fs::{
    self,
    File,
};
use std::io::{
    self,
    Read,
    Seek,
    SeekFrom,
    Write,
};
use std::iter::repeat_with;

use anyhow::Result;

use glam::DVec3;

use crate::cli::CameraConfig;
use crate::scene_config::*;

use super::create::*;

fn read_binary_stl_header(input: &mut File) -> Result<[u8; 80]> {
    let mut buf = [0u8; 80];

    input.read_exact(&mut buf)?;
    Ok(buf)
}

fn read_binary_stl_u32(input: &mut File) -> Result<u32> {
    let mut buf = [0u8; size_of::<u32>()];

    input.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_binary_stl_f32(input: &mut File) -> Result<f32> {
    let mut buf = [0u8; size_of::<f32>()];

    input.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

fn read_binary_stl_vec3(input: &mut File) -> Result<DVec3> {
    let x = read_binary_stl_f32(input)? as f64;
    let y = read_binary_stl_f32(input)? as f64;
    let z = read_binary_stl_f32(input)? as f64;

    Ok(DVec3::new(x, z, y))
}

fn read_binary_stl_triangle(input: &mut File) -> Result<(DVec3, DVec3, DVec3, DVec3)> {
    let normal = read_binary_stl_vec3(input)?;
    let vertex1 = read_binary_stl_vec3(input)?;
    let vertex2 = read_binary_stl_vec3(input)?;
    let vertex3 = read_binary_stl_vec3(input)?;

    let _ = input.seek(SeekFrom::Current(size_of::<u16>() as i64))?;

    Ok((normal, vertex1, vertex2, vertex3))
}

pub fn run(args: &ConvertSTLArgs) -> Result<()> {
    let mut input = File::open(&args.stl_file)?;

    let _ = read_binary_stl_header(&mut input)?;
    let count = read_binary_stl_u32(&mut input)? as usize;

    let stl_triangles =
        repeat_with(|| read_binary_stl_triangle(&mut input))
            .take(count)
            .collect::<Result<Vec<_>, _>>()?;

    let (p_min, p_max) =
        stl_triangles
            .iter().copied()
            .fold(
                (DVec3::INFINITY, -DVec3::INFINITY),
                |(p_min, p_max), (_, a, b, c)| (
                    p_min.min(a).min(b).min(c),
                    p_max.max(a).max(b).max(c),
                ),
            );

    let l = p_max.x - p_min.x;
    let h = p_max.y - p_min.y;
    let w = p_max.z - p_min.z;

    let k = args.scale/l.max(w).max(h);

    let objects =
        stl_triangles
            .iter().copied()
            .map(|(_, a, b, c)| {
                let point = k*(a - p_min);
                let u = k*(b - a);
                let v = k*(c - a);

                ObjectConfig::Triangle {
                    point,
                    u,
                    v,
                    material: 0,
                }
            })
            .collect::<VecDeque<_>>();

    let look_at = DVec3::new(k*l/2.0, k*h/2.0, 0.0);
    let look_from = look_at - args.scale*DVec3::Z;

    let mut camera = CameraConfig {
        background_color: Some(DVec3::ONE),
        look_at: Some(look_at),
        look_from: Some(look_from),
        field_of_view: Some(50.),
        ray_max_bounces: Some(50),
        samples_per_pixel: Some(200),
        ..CameraConfig::default()
    };

    camera.merge_with(&args.camera);

    let scene_config = SceneConfig {
        camera,
        textures: VecDeque::from(vec![
            TextureConfig::SolidColor { color: DVec3::X + DVec3::Y },
        ]),
        materials: VecDeque::from(vec![
            MaterialConfig::Lambertian { texture: 0 },
        ]),
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
