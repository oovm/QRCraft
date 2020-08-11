use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct LegoThemeBuilder {
    size: u32
}

