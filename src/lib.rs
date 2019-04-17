use amethyst::{
    assets::SimpleFormat,
    error::Error,
    renderer::{Texture, TextureData, TextureFormat, TextureMetadata},
};
use detect_filetype::{detect_filetype, FileType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, Hash)]
pub struct DetectTextureFormat;

impl SimpleFormat<Texture> for DetectTextureFormat {
    const NAME: &'static str = "DetectTextureFormat";

    type Options = TextureMetadata;

    fn import(&self, bytes: Vec<u8>, options: TextureMetadata) -> Result<TextureData, Error> {
        let format = match detect_filetype(&bytes) {
            Some(FileType::Jpeg) => TextureFormat::Jpg,
            Some(FileType::Png) => TextureFormat::Png,
            Some(FileType::Bmp) => TextureFormat::Bmp,
            Some(FileType::Tga) => TextureFormat::Tga,
            _ => return Err(Error::from_string("Not an image")),
        };

        format.import(bytes, options)
    }
}
