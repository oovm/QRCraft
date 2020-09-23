use crate::{MosaicCraft, MosaicCraftThemeItem, Rgb};
use image::DynamicImage;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MosaicCraftCanvas {
    pub data: Vec<MosaicCraftCanvasItem>,
}

#[derive(Debug, Clone)]
pub struct MosaicCraftCanvasItem {
    pub x1: u32,
    pub y1: u32,
    pub data: Rc<DynamicImage>,
}
