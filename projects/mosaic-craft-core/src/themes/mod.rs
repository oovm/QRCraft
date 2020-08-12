mod builder;
use builder::{repack_all_theme,repack_directory};
use serde::{Serialize,Deserialize};
use image::{Rgb, DynamicImage};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct LegoThemeConfig {
    name: String,
    designer:String,
    designer_url:String,
    path: Vec<String>,
    pack: Option<String>,
    preview: Option<String>
}


#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct LegoTheme {
    name: String,
    designer:String,
    designer_url:String,
    images: Vec<(Rgb<u8>, Rc<DynamicImage>)>
}

