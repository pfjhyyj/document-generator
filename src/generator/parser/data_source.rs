use std::{collections::{HashMap, VecDeque}, sync::{Arc, Mutex}};

use log::warn;
use tokio::runtime::Builder;
use url::Url;

use crate::{error::GeneratorError, generator::utils::http::get_file_by_url};

use super::{data_item::{DataItem, SingleValue}, render_format::RenderMode, result_item::{ self, ResultItem}};

#[derive(Debug)]
pub struct DataSource {
    pub data_item_map: HashMap<String, SingleValue>,
    pub file_map: HashMap<String, Vec<u8>>,
}

impl DataSource {
    pub fn new(data: DataItem, result_items: Vec<ResultItem>) -> Result<DataSource, GeneratorError> {
    
        let data_item_map = generate_data_item_map(data);

        let result_item_map = generate_result_item_map(result_items);

        let file_map = generate_file_cache_map(&data_item_map, &result_item_map);

        Ok(DataSource {data_item_map: data_item_map, file_map: file_map})
    }
    
    pub fn get_value(&self, key: &str) -> Option<&SingleValue> {
        self.data_item_map.get(key)
    }

    pub fn get_file(&self, url: &str) -> Option<&Vec<u8>> {
        self.file_map.get(url)
    }
}

fn generate_data_item_map(data: DataItem) -> HashMap<String, SingleValue> {
    let mut map = HashMap::new();
    let mut stack: VecDeque<TraverseDataItem> = VecDeque::new();
    
    struct TraverseDataItem {
        data_item: DataItem,
        prefix: String,
    }

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
    map
}

fn generate_result_item_map(result_items: Vec<ResultItem>) -> HashMap<String, ResultItem> {
    struct TraverseResultItem {
        result_item: ResultItem,
        prefix: String,
    }

    let mut result_item_stack: VecDeque<TraverseResultItem> = VecDeque::new();
    let mut result_item_map = HashMap::new();
    for result_item in result_items {
        result_item_stack.push_back(TraverseResultItem {
            result_item: result_item,
            prefix: "".to_string()
        });

        while let Some(current) = result_item_stack.pop_back() {
            let current_item = current.result_item;
            let prefix = current.prefix;
            match current_item.data_type {
                result_item::DataType::String | result_item::DataType::Number | result_item::DataType::Boolean => {
                    let key = if prefix.is_empty() {
                        current_item.key_name.clone()
                    } else {
                        format!("{}.{}", prefix, current_item.key_name)
                    };
                    result_item_map.insert(key, current_item);
                }
                result_item::DataType::Object => {
                    for property in current_item.properties {
                        let new_prefix = if prefix.is_empty() {
                            property.name.clone()
                        } else {
                            format!("{}.{}", prefix, property.name)
                        };
                        result_item_stack.push_back(TraverseResultItem {
                            result_item: property,
                            prefix: new_prefix
                        });
                    }
                }
                result_item::DataType::Array => {
                    let new_prefix = if prefix.is_empty() {
                        current_item.name.clone()
                    } else {
                        format!("{}.{}", prefix, current_item.name)
                    };
                    for (index, property) in current_item.properties.into_iter().enumerate() {
                        let new_prefix = format!("{}[{}]", new_prefix, index);
                        result_item_stack.push_back(TraverseResultItem {
                            result_item: property,
                            prefix: new_prefix
                        });
                    }
                }
                
            }
        }
        
    }
    result_item_map
}


fn generate_file_cache_map(data_item_map: &HashMap<String, SingleValue>, result_item_map: &HashMap<String, ResultItem>) -> HashMap<String, Vec<u8>> {
    let mut file_map: HashMap<String, Vec<u8>> = HashMap::new();
    for (key, result_item) in result_item_map {
        if result_item.render_mode == RenderMode::Image || result_item.render_mode == RenderMode::Template {
            let url = data_item_map.get(key);
            if let Some(SingleValue::String(url)) = url {
                if Url::parse(&url).is_err() {
                    // ignore invalid url
                    warn!("Invalid url: {}", url);
                    continue;
                }
                let content: Vec<u8> = Vec::new();
                file_map.insert(url.clone(), content);
            }
        }
    }

    download_all_files(&mut file_map);

    file_map
}

fn download_all_files(file_map: &mut HashMap<String, Vec<u8>>) {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let file_map_arc = Arc::new(Mutex::new(file_map.clone()));
    let mut handles = Vec::new();

    for url in file_map.keys() {
        let file_map = Arc::clone(&file_map_arc);
        handles.push(runtime.spawn(download_file_and_save(url.clone(), file_map)));
    }

    for handle in handles {
        runtime.block_on(handle).unwrap();
    }

    let file_map_lock = Arc::try_unwrap(file_map_arc).unwrap();
    let final_file_map = file_map_lock.into_inner().unwrap();
    file_map.clear();
    file_map.extend(final_file_map);
}

async fn download_file_and_save(url: String, file_map: Arc<Mutex<HashMap<String, Vec<u8>>>>) {
    let result = get_file_by_url(url.clone()).await;
    if let Ok(content) = result {
        let mut file_map = file_map.lock().unwrap();
        file_map.insert(url, content);
    } else {
        warn!("Failed to download file: {:?}", result);
    }
}