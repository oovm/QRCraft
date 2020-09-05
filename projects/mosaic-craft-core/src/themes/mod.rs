mod builder;
use crate::ColorAverage;
pub use builder::{repack_all_theme, repack_directory};
use image::{DynamicImage, Rgb};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MosaicCraftThemeConfig {
    name: String,
    authors: Vec<String>,
    images_path: Vec<String>,
    images_pack: Option<String>,
    color_average: ColorAverage,
    preview: Option<String>,
}

#[derive(Clone)]
pub struct MosaicCraftTheme {
    name: String,
    authors: Vec<String>,
    color_average: ColorAverage,
    images: Vec<(Rgb<u8>, DynamicImage)>,
}

impl Debug for MosaicCraftTheme {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("MosaicCraftTheme")
            .field("name", &self.name)
            .field("authors", &self.authors)
            .field("images", &self.images.len())
            .finish()
    }
}

impl Default for MosaicCraftThemeConfig {
    fn default() -> Self {
        Self {
            name: String::from("anonymous"),
            authors: vec![],
            images_path: vec![],
            images_pack: None,
            color_average: Default::default(),
            preview: None,
        }
    }
}

impl From<MosaicCraftThemeConfig> for MosaicCraftTheme {
    fn from(cfg: MosaicCraftThemeConfig) -> Self {
        Self { name: cfg.name, authors: cfg.authors, color_average: cfg.color_average, images: vec![] }
    }
}

impl MosaicCraftTheme {
    pub fn load_buildin() -> Self {
        unimplemented!()
    }
}
