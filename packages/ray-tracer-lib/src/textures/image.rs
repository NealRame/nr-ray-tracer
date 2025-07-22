use std::path::PathBuf;

use image::{
    ImageError,
    ImageReader,
    Rgb32FImage
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename = "Image")]
pub struct ImageConfig {
    file: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ImageConfig" )]
pub struct Image {
    file: PathBuf,

    #[serde(skip)]
    pub(super) image: Rgb32FImage,
}

impl TryFrom<ImageConfig> for Image {
    type Error = ImageError;
    fn try_from(value: ImageConfig) -> Result<Self, Self::Error> {
        let image = ImageReader::open(&value.file)?.decode()?.into_rgb32f();

        return Ok(Self {
            file: value.file,
            image,
        })
    }
}
