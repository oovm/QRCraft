use crate::{MosaicCraftTheme, Result, MOSAIC_CRAFT_MAX_BLOCK_SIZE};
use image::{imageops::FilterType, DynamicImage, GenericImageView, RgbImage};
use std::path::Path;
use walkdir::WalkDir;

pub fn repack_directory(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<MosaicCraftTheme> {
    for entry in WalkDir::new(from).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".png") {
            println!("{}", f_name);
        }
    }
    unimplemented!()
}

pub fn repack_all_theme(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    unimplemented!()
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
    let pack = repack_directory("../mosaic-craft-themes", "../mosaic-craft-themes").unwrap();
    println!("{:?}", pack)
}
