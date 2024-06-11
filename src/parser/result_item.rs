use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::render_format::{RenderFormat, RenderMode};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub enum DataItem {
    SingleValue(SingleValue),
    Object(HashMap<String, DataItem>),
    Array(Vec<DataItem>)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SingleValue {
    Number(f64),
    String(String),
    Boolean(bool),
    NumberArray(Vec<f64>),
    StringArray(Vec<String>),
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum DataType {
  String = 1,
  Number = 2,
  Boolean = 3,
  Object = 4,
  Array = 5
}

#[derive(Serialize, Deserialize, Debug)]
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

