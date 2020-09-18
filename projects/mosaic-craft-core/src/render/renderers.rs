use crate::{MosaicCraft, MosaicCraftCanvas, Result};
use image::{io::Reader, DynamicImage};
use std::{io::Cursor, path::Path};

impl MosaicCraft {
    pub fn render_path(&self, path: impl AsRef<Path>) -> Result<MosaicCraftCanvas> {
        let img = Reader::open(path)?.decode()?;
        Ok(self.render(img))
    }
    pub fn render_bytes(&self, bytes: &[u8]) -> Result<MosaicCraftCanvas> {
        let img = Reader::new(Cursor::new(bytes)).decode()?;
        Ok(self.render(img))
    }

    pub fn render(&self, img: DynamicImage) -> MosaicCraftCanvas {
        unsafe {
            match self.pixel_aligned {
                true => self.render_grid(img),
                false => self.render_mono(),
            }
        }
    }
    unsafe fn render_grid(&self, img: DynamicImage) -> MosaicCraftCanvas {
        unimplemented!()
    }
    fn render_mono(&self) -> LegoCanvas {
        unimplemented!()
    }
}
