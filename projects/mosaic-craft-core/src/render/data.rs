
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
