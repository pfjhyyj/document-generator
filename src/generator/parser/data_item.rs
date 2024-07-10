use std::collections::HashMap;

use serde::{Deserialize, Serialize};



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