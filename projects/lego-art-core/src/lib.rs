mod errors;
mod themes;

pub use crate::themes::LegoTheme;
pub use crate::{
    errors::{LegoArtError, Result},
};
pub use image::{Luma, Rgb};

#[derive(Debug,Copy, Clone)]
pub enum ColorAverage {
    RGBSpace,

}

#[derive(Debug, Copy, Clone)]
pub enum ColorMetrics {
    Manhattan,
    Euclid,
}

#[derive(Debug, Clone)]
pub struct LegoArt {
    pub color_average: ColorAverage,
    pub color_metrics: ColorMetrics,
    pub theme: LegoTheme,
}

impl Default for LegoArt {
    fn default() -> Self {
        Self {
            color_average: ColorAverage::RGBSpace,
            color_metrics: ColorMetrics::Manhattan,
            theme: LegoTheme {}
        }
    }
}
