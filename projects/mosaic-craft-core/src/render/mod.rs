use image::{DynamicImage, GenericImageView, Rgb};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ColorAverage {
    RGBSpace,
}

#[derive(Debug, Copy, Clone)]
pub enum ColorMetrics {
    Manhattan,
    Euclid,
}

impl ColorAverage {
    pub fn mean(&self, img: &DynamicImage) -> Rgb<u8> {
        match self {
            ColorAverage::RGBSpace => {
                let all = img.width() as f32 * img.height() as f32;
                let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
                for c in img.to_rgb().pixels() {
                    unsafe {
                        r += *c.0.get_unchecked(0) as f32;
                        g += *c.0.get_unchecked(1) as f32;
                        b += *c.0.get_unchecked(2) as f32;
                    }
                }
                Rgb([(r / all) as u8, (g / all) as u8, (b / all) as u8])
            }
        }
    }
}
