mod builder;
use builder::{repack_all_theme, repack_directory};
use image::{DynamicImage, Rgb};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MosaicCraftThemeConfig {
    name: String,
    designer: String,
    designer_url: String,
    images_path: Vec<String>,
    images_pack: Option<String>,
    preview: Option<String>,
}

#[derive(Clone)]
pub struct MosaicCraftTheme {
    name: String,
    designer: String,
    designer_url: String,
    images: Vec<(Rgb<u8>, DynamicImage)>,
}

impl Debug for MosaicCraftTheme {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("MosaicCraftTheme")
            .field("name", &self.name)
            .field("designer", &self.designer)
            .field("designer_url", &self.designer_url)
            .field("images", &self.images.len())
            .finish()
    }
}

impl Default for MosaicCraftThemeConfig {
    fn default() -> Self {
        Self {
            name: String::from("unknown"),
            designer: String::from("Anonymous"),
            designer_url: String::from("unknown"),
            images_path: vec![],
            images_pack: None,
            preview: None,
        }
    }
}

impl From<MosaicCraftThemeConfig> for MosaicCraftTheme {
    fn from(cfg: MosaicCraftThemeConfig) -> Self {
        Self {
            name: cfg.name,
            designer: cfg.designer,
            designer_url: cfg.designer_url,
            images: vec![]
        }
    }
}

impl MosaicCraftTheme {
    pub fn load_buildin() -> Self {
        unimplemented!()
    }
}
