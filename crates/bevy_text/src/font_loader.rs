use crate::Font;
use bevy_asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext};
use thiserror::Error;

#[derive(Default)]
/// FontLoader for usage by the [`AssetServer`]
pub struct FontLoader;

/// Possible errors that can be produced by [`FontLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum FontLoaderError {
    /// The contents that could not be parsed
    #[error(transparent)]
    Content(#[from] cosmic_text::ttf_parser::FaceParsingError),
    /// An [IO](std::io) Error
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl AssetLoader for FontLoader {
    type Asset = Font;
    type Settings = ();
    type Error = FontLoaderError;
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Font, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let font = Font::try_from_bytes(bytes)?;
        Ok(font)
    }

    fn extensions(&self) -> &[&str] {
        &["ttf", "otf"]
    }
}
