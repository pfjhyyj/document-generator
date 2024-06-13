use std::{collections::{HashMap, VecDeque}, fmt::Error};

use super::result_item::{ DataItem, SingleValue};

#[derive(Debug)]
pub struct DataSource {
    pub data_map: HashMap<String, SingleValue>,
}

pub struct TraverseDataItem {
    data_item: DataItem,
    prefix: String,
}

impl DataSource {
    pub fn new(data: DataItem) -> Result<DataSource, Error> {
        let mut map = HashMap::new();
        let mut stack: VecDeque<TraverseDataItem> = VecDeque::new();
        stack.push_back(TraverseDataItem { 
            data_item: data,
            prefix: "".to_string()
        });

        while let Some(current) = stack.pop_back() {
            match current.data_item {
                DataItem::SingleValue(value) => {
                    map.insert(current.prefix, value);
                }
                DataItem::Array(array) => {
                    for (index, value) in array.into_iter().enumerate() {
                        stack.push_back(TraverseDataItem {
                            data_item:  value,
                            prefix: format!("{}[{}]", current.prefix, index)
                        });
                    }
                }
                DataItem::Object(object) => {
                    for (key, value) in object {
                        let prefix = if current.prefix.is_empty() {
                            key
                        } else {
                            format!("{}.{}", current.prefix, key)
                        };
                        stack.push_back(TraverseDataItem {
                            data_item: value,
                            prefix: prefix
                        })
                    }
                }
            }
        }
        Ok(DataSource {data_map: map})
    }

    // pub fn get_data(&self, key: &str) -> Option<&SingleValue> {
    //     self.data_map.get(key)
    // }
}