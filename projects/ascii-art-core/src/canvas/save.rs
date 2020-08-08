use crate::{AsciiCanvas, Result};
use std::{fs, path::Path};
use image::ImageFormat;

impl AsciiCanvas {
    pub fn save_svg(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(fs::write(path, self.draw_svg().as_bytes())?)
    }
    pub fn save_image(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(self.draw_image().save_with_format(path, ImageFormat::Png)?)
    }
    pub fn save_text(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(fs::write(path, self.draw_text().as_bytes())?)
    }
}
