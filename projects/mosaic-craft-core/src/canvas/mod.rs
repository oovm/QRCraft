use crate::{LegoData, Rgb};
use std::rc::Rc;

mod draw;
mod save;

#[derive(Debug, Clone)]
pub struct LegoCanvas {
    pub data: Vec<LegoCanvasItem>,
    pub font_size: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct LegoCanvasItem {
    pub x: f32,
    pub y: f32,
    pub color: Rgb<u8>,
    pub data: Rc<LegoData>,
}
