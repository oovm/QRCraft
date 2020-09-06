mod builder;
use crate::ColorAverage;
pub use builder::{repack_all_theme, repack_directory};
use image::{DynamicImage, Rgb, ImageOutputFormat};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};
use serde::ser::{Serializer, SerializeStruct};
use serde::de::{self, Deserializer, Visitor, SeqAccess, MapAccess};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MosaicCraftThemeConfig {
    name: String,
    authors: Vec<String>,
    images_path: Vec<String>,
    images_pack: Option<String>,
    color_average: ColorAverage,
    preview: Option<String>,
}

#[derive(Clone,Serialize, Deserialize, )]
pub struct MosaicCraftTheme {
    name: String,
    authors: Vec<String>,
    color_average: ColorAverage,
    images: Vec<MosaicCraftThemeItem>,
}

#[derive(Clone, Debug)]
pub struct MosaicCraftThemeItem {
    color: Rgb<u8>,
    image: DynamicImage,
}

impl Serialize for MosaicCraftThemeItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("MosaicCraftThemeItem", 3)?;
        let mut buf = vec![];
        self.image.write_to(&mut buf, ImageOutputFormat::Png);
        unsafe {
            state.serialize_field("r", self.color.0.get_unchecked(0))?;
            state.serialize_field("g", self.color.0.get_unchecked(1))?;
            state.serialize_field("b", self.color.0.get_unchecked(2))?;
            state.serialize_field("image", &buf)?;
        }
        state.end()
    }
}



impl<'de> Deserialize<'de> for MosaicCraftThemeItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        enum Field { Secs, Nanos }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`r` | `g` | `b` | `image`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                    {
                        match value {
                            "secs" => Ok(Field::Secs),
                            "nanos" => Ok(Field::Nanos),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        const FIELDS: &'static [&'static str] = &["r", "g", "b", "image"];
        deserializer.deserialize_struct("Duration", FIELDS, FieldVisitor)
    }
}


impl Debug for MosaicCraftTheme {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("MosaicCraftTheme")
            .field("name", &self.name)
            .field("authors", &self.authors)
            .field("images", &self.images.len())
            .finish()
    }
}

impl Default for MosaicCraftThemeConfig {
    fn default() -> Self {
        Self {
            name: String::from("anonymous"),
            authors: vec![],
            images_path: vec![],
            images_pack: None,
            color_average: Default::default(),
            preview: None,
        }
    }
}

impl From<MosaicCraftThemeConfig> for MosaicCraftTheme {
    fn from(cfg: MosaicCraftThemeConfig) -> Self {
        Self { name: cfg.name, authors: cfg.authors, color_average: cfg.color_average, images: vec![] }
    }
}

impl MosaicCraftTheme {
    pub fn load_buildin() -> Self {
        unimplemented!()
    }
}

