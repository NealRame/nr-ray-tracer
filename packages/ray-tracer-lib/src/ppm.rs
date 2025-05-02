use std::io::Write;

use anyhow::Result;

use crate::image::Image;

pub fn write_ppm<T: Write>(
    image: &Image,
    out: &mut T,
) -> Result<()> {
    writeln!(out, "P3")?;
    writeln!(out, "{} {}", image.get_width(), image.get_height())?;
    writeln!(out, "255")?;

    for (_, color) in image.iter() {
        let color = (255.*color).as_u8vec3();
        writeln!(out, "{:03} {:03} {:03}", color.x, color.y, color.z)?;
    }

    Ok(())
}
