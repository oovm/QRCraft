mod data;
use crate::{canvas::LegoCanvasItem, LegoArt, LegoCanvas, Result};
pub use data::{LegoData, AsciiSet};
use fontdue::Font;
use image::{imageops::FilterType, io::Reader, DynamicImage, GenericImageView, Pixel};
use std::{io::Cursor, path::Path};

#[derive(Copy, Clone, Debug)]
pub enum LegoColorMode {
    Gray = 0,
    Color = 1,
    Mask = 2,
}

impl Default for LegoColorMode {
    fn default() -> Self {
        Self::Gray
    }
}

impl LegoArt {
    pub fn build_font_cache(&mut self, font: &Font, chars: &str) {
        self.char_set.load_string(font, chars)
    }
}

impl LegoArt {
    pub fn render_path(&self, path: impl AsRef<Path>) -> Result<LegoCanvas> {
        let img = Reader::open(path)?.decode()?;
        Ok(self.render(img))
    }
    pub fn render_bytes(&self, bytes: &[u8]) -> Result<LegoCanvas> {
        let img = Reader::new(Cursor::new(bytes)).decode()?;
        Ok(self.render(img))
    }

    pub fn render(&self, img: DynamicImage) -> LegoCanvas {
        unsafe {
            match self.pixel_aligned {
                true => self.render_grid(img),
                false => self.render_mono(),
            }
        }
    }
    unsafe fn render_grid(&self, img: DynamicImage) -> LegoCanvas {
       unimplemented!()
    }
    fn render_mono(&self) -> LegoCanvas {
        unimplemented!()
    }
}
