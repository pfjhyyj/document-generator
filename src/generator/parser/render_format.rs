use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum RenderMode {
    String = 1,
    Number = 2,
    Image = 3,
    Table = 4,
    Checkbox = 5,
    Embedded = 98,
    Template = 99,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RenderFormat {
    None,
    ImageFormat(ImageFormat),
    NumberFormat(NumberFormat),
    TableFormat(TableFormat),
    TemplateFormat(TemplateFormat),
}

impl Default for RenderFormat {
    fn default() -> Self {
        RenderFormat::None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum ImageRenderMode {
    AutoSize = 1,
    FixSize = 2,
}

impl Default for ImageRenderMode {
    fn default() -> Self {
        ImageRenderMode::AutoSize
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageFormat {
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    #[serde(default)]
    pub mode: ImageRenderMode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NumberFormat {
    pub precision: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableFormat {
    pub repeat_header: bool,
    pub header_row_length: u8,
    pub merge_column: bool,
    pub merge_column_length: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateFormat {
    pub template: String,
}
