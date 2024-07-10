use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::render_format::{RenderFormat, RenderMode};


#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum DataType {
  String = 1,
  Number = 2,
  Boolean = 3,
  Object = 4,
  Array = 5
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultItem {
  pub name: String,
  pub key_name: String,
  pub data_type: DataType,  
  pub render_mode: RenderMode,
  #[serde(default)]
  pub render_format: RenderFormat,
  #[serde(default)]
  pub properties: Vec<ResultItem>
}

