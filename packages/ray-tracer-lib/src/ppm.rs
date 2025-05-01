use std::io::Write;

use anyhow::{
    anyhow,
    Result,
};

use glam::U8Vec4;

pub fn write_ppm<T: Write>(
    out: &mut T,
    width: usize,
    height: usize,
    pixels: &[U8Vec4],
) -> Result<()> {
    if width*height != pixels.len() {
        return Err(anyhow!("Invalid sized"));
    }

    writeln!(out, "P3")?;
    writeln!(out, "{width} {height}")?;
    writeln!(out, "255")?;

    for color in pixels {
        writeln!(out, "{:03} {:03} {:03}", color.x, color.y, color.z)?;
    }

    Ok(())
}
