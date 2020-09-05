use image::ImageError;

#[derive(Debug, Clone)]
pub enum LegoArtError {
    NoneError,
    IOError(String),
    ImageError(String),
    SerdeError(String),
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

impl From<serde_json::Error> for LegoArtError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(format!("{}", e))
    }
}

impl From<Box<bincode::ErrorKind>> for LegoArtError {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        Self::SerdeError(format!("{}", e))
    }
}
