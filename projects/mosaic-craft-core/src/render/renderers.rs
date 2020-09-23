use crate::{MosaicCraft, MosaicCraftCanvas, MosaicCraftCanvasItem, Result};
use image::{imageops::FilterType, io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use itertools::Itertools;
use std::{io::Cursor, path::Path, rc::Rc};

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
        let mut theme = self.theme.images.iter().map(|e| Rc::new(e.image_resized(self.grid_size))).collect_vec();

        match self.background {
            None => (),
            Some(c) => {
                let bg = ImageBuffer::from_pixel(self.grid_size, self.grid_size, c);
                theme.push(Rc::new(DynamicImage::ImageRgb8(bg)))
            }
        }
        let w = img.width() / self.grid_size;
        let h = img.height() / self.grid_size;
        let mut data = Vec::with_capacity(w as uszie * h as usize);
        let resized = img.resize_exact(w, h, FilterType::Triangle).to_rgb();
        for (x, y, c) in resized.enumerate_pixels() {
            MosaicCraftCanvasItem { x1: x, y1: y, data: Rc::new(()) }
        }

        MosaicCraftCanvas { data }
    }
}

fn find_nearest_img() {}
