use image::ImageError;

#[derive(Debug, Clone)]
pub enum AsciiArtError {
    NoneError,
    IOError(String),
    ImageError(String),
}

pub type Result<T> = std::result::Result<T, AsciiArtError>;

impl From<std::io::Error> for AsciiArtError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(format!("{}", e))
    }
}

impl From<ImageError> for AsciiArtError {
    fn from(e: ImageError) -> Self {
        match e {
            ImageError::IoError(e) => Self::IOError(format!("{}", e)),
            _ => Self::ImageError(format!("{}", e)),
        }
    }
}
