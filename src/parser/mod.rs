use result_item::{DataItem, ResultItem};
use serde::{Deserialize, Serialize};

pub mod result_item;
pub mod data_source;
pub mod render_format;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerationRequest {
    pub template_file: String,
    pub data_item: DataItem,
    #[serde(default)]
    pub result_item: Vec<ResultItem>
}