use std::io::Write;

use anyhow::{
    anyhow,
    Result,
};

pub fn write_ppm<T: Write>(
    out: &mut T,
    width: usize,
    height: usize,
    pixels: &[u8],
) -> Result<()> {
    if width*height != pixels.len()/4 {
        return Err(anyhow!("Invalid sized"));
    }

    writeln!(out, "P3")?;
    writeln!(out, "{width} {height}")?;
    writeln!(out, "255")?;

    for chunk in pixels.chunks_exact(4) {
        writeln!(out, "{:03} {:03} {:03}", chunk[0], chunk[1], chunk[2])?;
    }

    Ok(())
}
