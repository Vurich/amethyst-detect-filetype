use amethyst::{
    assets::{Error, ErrorKind, Result, SimpleFormat},
    renderer::{Texture, TextureData, TextureFormat, TextureMetadata},
};
use detect_filetype::{detect_filetype, FileType};

pub struct DetectTextureFormat;

impl SimpleFormat<Texture> for DetectTextureFormat {
    const NAME: &'static str = "DetectTextureFormat";

    type Options = TextureMetadata;

    fn import(&self, bytes: Vec<u8>, options: TextureMetadata) -> Result<TextureData> {
        let format = match detect_filetype(&bytes) {
            Some(FileType::Jpeg) => TextureFormat::Jpg,
            Some(FileType::Png) => TextureFormat::Png,
            Some(FileType::Bmp) => TextureFormat::Bmp,
            Some(FileType::Tga) => TextureFormat::Tga,
            _ => return Err(Error::from_kind(ErrorKind::Asset(format!("Not an image")))),
        };

        format.import(bytes, options)
    }
}