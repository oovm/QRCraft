use crate::{
    themes::MosaicCraftThemeConfig, MosaicCraftTheme, Result, MOSAIC_CRAFT_MAX_BLOCK_SIZE, MOSAIC_CRAFT_THEME_CONFIG_NAME,
};
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

pub fn repack_directory(config_from: impl AsRef<Path>, pack_to: impl AsRef<Path>) -> Result<MosaicCraftTheme> {
    let config_file = config_from.as_ref().join(MOSAIC_CRAFT_THEME_CONFIG_NAME);
    let mut config = MosaicCraftThemeConfig::try_parse_config(config_file);
    let mut theme = MosaicCraftTheme::from(config.clone());
    for entry in std::fs::read_dir(config_from)?.filter_map(|e| e.ok()) {
        if let Some((img, name)) = MosaicCraftThemeConfig::try_parse_png(entry) {
            config.images_path.push(name);
            theme.push_image(img);
        }
    }
    fs::write(&config_file,serde_json::to_string(&address)?.as_bytes())?;
    fs::write(pack_to,bincode::serialize(&world)?)?;
    return Ok(theme);
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
    fn push_image(&mut self, img: DynamicImage) {
        let checked = check_image_size(img);
        let mean = self.color_average.mean(&checked);
        self.images.push((mean, checked));
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
    fn try_parse_png(file: std::fs::DirEntry) -> Option<(DynamicImage, String)> {
        match image::open(file.path()) {
            Ok(o) => {
                let name = file.file_name().to_string_lossy().to_string();
                Some((o, name))
            }
            Err(_) => None,
        }
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
