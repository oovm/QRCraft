use image::ImageError;

#[derive(Debug, Clone)]
pub enum LegoArtError {
    NoneError,
    IOError(String),
    ImageError(String),
}

pub type Result<T> = std::result::Result<T, LegoArtError>;

impl From<std::io::Error> for LegoArtError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(format!("{}", e))
    }
}

impl From<ImageError> for LegoArtError {
    fn from(e: ImageError) -> Self {
        match e {
            ImageError::IoError(e) => Self::IOError(format!("{}", e)),
            _ => Self::ImageError(format!("{}", e)),
        }
    }
}
