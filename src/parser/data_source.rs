use std::collections::{HashMap, VecDeque};

use serde::de::value;
use serde_json::{Error, Value};

use super::result_item::{DataItem, SingleValue};

pub struct DataSource {
    data_map: HashMap<String, SingleValue>,
}

impl DataSource {
    pub fn new(data: DataItem) -> Result<DataSource, Error> {
        let mut map = HashMap::new();
        let mut stack: VecDeque<DataItem> = VecDeque::new();
        let mut prefix = "";
        stack.push_back(data);

        while let Some(current) = stack.pop_back() {
            match current {
                DataItem::SingleValue(value) => {
                    map.insert(prefix.clone(), value);
                }
                DataItem::Array(array) => {
                    for value in array {
                        stack.push_back(value);
                    }
                }
                DataItem::Object(object) => {
                    for (key, value) in object {
                        stack.push_back(value)
                    }
                }
            }
        }
        Ok(DataSource {data_map: map})
    }
    // pub fn new(source_json: &String) -> Result<DataSource, Error> {
    //     let json: Value = serde_json::from_str(source_json)?;
    //     let mut map = HashMap::new();
    //     flatten_json(&mut map, json, "".to_string());
    //     Ok(DataSource { data_map: map })
    // }

    // pub fn get_data(&self, key: &str) -> Option<&Value> {
    //     self.data_map.get(key)
    // }

    // pub fn validate(&self) -> bool {
    //     return true;
    // }
}

fn flatten_json(map: &mut HashMap<String, Value>, value: Value, prefix: String) {
    match value {
        Value::Object(obj) => {
            for (k, v) in obj {
                let new_prefix = if prefix.is_empty() {
                    k
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_json(map, v, new_prefix);
            }
        }
        Value::Array(arr) => {
            map.insert(prefix.clone(), Value::Array(arr.clone()));
            for (i, v) in arr.into_iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, i);
                flatten_json(map, v, new_prefix)
            }
        }
        _ => {
            map.insert(prefix, value);
        }
    }
}
