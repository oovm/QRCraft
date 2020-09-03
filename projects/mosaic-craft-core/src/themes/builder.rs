use crate::{
    themes::MosaicCraftThemeConfig, MosaicCraftTheme, Result, MOSAIC_CRAFT_MAX_BLOCK_SIZE, MOSAIC_CRAFT_THEME_CONFIG_NAME,
};
use image::{imageops::FilterType, DynamicImage, GenericImageView, RgbImage};
use serde_json::Error;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

pub fn repack_directory(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<MosaicCraftTheme> {
    let config_file = from.as_ref().join(MOSAIC_CRAFT_THEME_CONFIG_NAME);
    let mut config = MosaicCraftThemeConfig::try_parse_config(config_file);
    let mut theme = MosaicCraftTheme::from(config);
    let mut pack = vec![];
    let out = MosaicCraftTheme { name: "".to_string(), designer: "".to_string(), designer_url: "".to_string(), images: vec![] };
    for entry in std::fs::read_dir(from)?.filter_map(|e| e.ok()) {
        match MosaicCraftThemeConfig::try_parse_png(entry) {
            None => (),
            Some((img, name)) => {
                config.images_path.push(name),
                theme.push_image(img)
            }
        }

    }
    unimplemented!()
}

pub fn repack_all_theme(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    for entry in WalkDir::new(from).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(MOSAIC_CRAFT_THEME_CONFIG_NAME) {
            println!("{}", f_name);
        }
    }
    unimplemented!()
}

impl MosaicCraftTheme {
    fn push_image(&mut self, img: DynamicImage){
        let name = file.parent().and_then(|e| e.file_name()).unwrap().to_string_lossy().to_string();
        let raw = read_to_string(file).unwrap_or(String::from("|"));
        match serde_json::from_str::<Self>(&raw) {
            Ok(o) => o,
            Err(_) => Self { name, ..Default::default() },
        }
    }
}

impl MosaicCraftThemeConfig {
    fn try_parse_config(file: PathBuf) -> Self {
        let name = file.parent().and_then(|e| e.file_name()).unwrap().to_string_lossy().to_string();
        let raw = read_to_string(file).unwrap_or(String::from("|"));
        match serde_json::from_str::<Self>(&raw) {
            Ok(o) => o,
            Err(_) => Self { name, ..Default::default() },
        }
    }
    fn try_parse_png(file: DirEntry)->Option<(DynamicImage, String)> {
        let f_name = file.file_name().to_string_lossy().to_string(); // FIXME: why use to_string
        if f_name.ends_with(".png") {
            print!("{}", f_name);
        }
        return None
    }
}

#[rustfmt::skip]
fn check_image_size(image: DynamicImage) -> DynamicImage {
    if image.width() > MOSAIC_CRAFT_MAX_BLOCK_SIZE
    || image.height() > MOSAIC_CRAFT_MAX_BLOCK_SIZE
    || image.width() != image.height()
    {
        let size = MOSAIC_CRAFT_MAX_BLOCK_SIZE.min(image.width()).min(image.height());
        image.resize_exact(size, size, FilterType::Nearest)
    }
    else {
        return image;
    }
}

#[test]
fn test() {
    let pack = repack_directory("../mosaic-craft-themes/minecraft3d", "../mosaic-craft-themes").unwrap();
    println!("{:?}", pack)
}
