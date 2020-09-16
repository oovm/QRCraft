use crate::{MosaicCraft, MosaicCraftThemeItem, Rgb};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MosaicCraftCanvas {
    pub data: Vec<MosaicCraftCanvasItem>,
    pub font_size: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct MosaicCraftCanvasItem {
    pub x: f32,
    pub y: f32,
    pub color: Rgb<u8>,
    pub data: Rc<MosaicCraftThemeItem>,
}
