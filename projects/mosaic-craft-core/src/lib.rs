mod errors;
mod render;
mod themes;

pub use crate::{
    errors::{LegoArtError, Result},
    render::{ColorAverage, ColorMetrics},
    themes::{repack_all_theme, repack_directory, MosaicCraftTheme},
};
pub use image::{Luma, Rgb};

pub const MOSAIC_CRAFT_MAX_BLOCK_SIZE: u32 = 100;
pub const MOSAIC_CRAFT_THEME_CONFIG_NAME: &str = "mosaic-craft-theme.json";

#[derive(Debug, Clone)]
pub struct MosaicCraft {
    pub color_average: ColorAverage,
    pub color_metrics: ColorMetrics,
    pub theme: MosaicCraftTheme,
}

impl Default for MosaicCraft {
    fn default() -> Self {
        let theme = MosaicCraftTheme::load_buildin();
        Self { color_average: Default::default(), color_metrics: Default::default(), theme }
    }
}
