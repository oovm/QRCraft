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
    path: Vec<String>,
    pack: Option<String>,
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
        match self {
            MosaicCraftTheme { name, designer, designer_url, images } => f
                .debug_struct("MosaicCraftTheme")
                .field("name", name)
                .field("designer", designer)
                .field("designer_url", designer_url)
                .field("images", &images.len())
                .finish(),
        }
    }
}

impl MosaicCraftTheme {
    pub fn load_buildin() -> Self {
        unimplemented!()
    }
}
