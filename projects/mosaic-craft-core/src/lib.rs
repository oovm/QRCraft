mod errors;
mod themes;

pub use crate::{
    errors::{LegoArtError, Result},
    themes::MosaicCraftTheme,
};
pub use image::{Luma, Rgb};

pub const MOSAIC_CRAFT_MAX_BLOCK_SIZE: u32 = 100;

#[derive(Debug, Copy, Clone)]
pub enum ColorAverage {
    RGBSpace,
}

#[derive(Debug, Copy, Clone)]
pub enum ColorMetrics {
    Manhattan,
    Euclid,
}

#[derive(Debug, Clone)]
pub struct MosaicCraft {
    pub color_average: ColorAverage,
    pub color_metrics: ColorMetrics,
    pub theme: MosaicCraftTheme,
}

impl Default for MosaicCraft {
    fn default() -> Self {
        let theme = MosaicCraftTheme::load_buildin();
        Self { color_average: ColorAverage::RGBSpace, color_metrics: ColorMetrics::Manhattan, theme }
    }
}
