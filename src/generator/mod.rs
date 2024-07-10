use std::{fs::File, io::Read, path::Path};

use docx_rs::{Document, Docx};
use parser::{data_item::SingleValue, data_source::DataSource, render_format::{self, ImageFormat, NumberFormat, RenderFormat, RenderMode}, result_item::ResultItem, GenerationRequest};
use renderer::{document::replace_placeholder_in_document, replacement::{self, Replacement, StringReplacement}};

use crate::error::GeneratorError;

pub mod parser;
pub mod renderer;
pub mod utils;

pub fn generate_new_document(input_param: &String) -> Result<bool, GeneratorError>{
    let request: GenerationRequest = serde_json::from_str(&input_param)?;
    let data_source = DataSource::new(request.data_item, request.result_item.clone())?;

    // test input file
    let mut file = File::open("./hello.docx")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let document = docx_rs::read_docx(&buf)?;

    let new_document = process_result_items(&document.document, &data_source, &request.result_item)?;
    // test output file
    let path = Path::new("output.docx");
    let file = File::create(path)?;
    let output_document = Docx::new();
    let output_document = output_document.document(new_document);
    output_document.build().pack(file).map_err(|_| GeneratorError::SystemError("pack up docx file failed".to_string()))?;

    Ok(true)
}

fn process_result_items(document: &Document, data_source: &DataSource, result_items: &Vec<ResultItem>) -> Result<Document, GeneratorError> {
    let mut new_document = document.clone();
    for result_item in result_items {
        let replacement=  match result_item.render_mode {
            RenderMode::String => {
                let data = data_source.get_value(result_item.key_name.as_str());
                let data = match data {
                    Some(data) => data,
                    None => continue,
                };
                let data = parse_single_value_into_string(data);
                Replacement::STRING(StringReplacement { value: data })
            }
            RenderMode::Number => {
                let data = data_source.get_value(result_item.key_name.as_str());
                let data = match data {
                    Some(data) => data,
                    None => continue,
                };
                let data = parse_single_value_into_number(data);
                let render_format = parse_render_format_into_number_format(&result_item.render_format)?;
                Replacement::NUMBER(replacement::NumberReplacement { value: data, precision: render_format.precision })
            }
            RenderMode::Image => {
                let data = data_source.get_value(result_item.key_name.as_str());
                let data = match data {
                    Some(data) => data,
                    None => continue,
                };
                let data = match data {
                    SingleValue::String(string) => string,
                    _ => continue,
                };
                let image_data = data_source.get_file(data.as_str());
                let image_data = match image_data {
                    Some(data) => data,
                    None => continue,
                };
                // let render_format = parse_render_format_into_image_format(&result_item.render_format)?;
                Replacement::IMAGE(replacement::ImageReplacement { value: image_data.clone() })
            }
            _ => {
                // todo: implement other render mode
                continue;
            }
        };
        let placeholder = get_placeholder(result_item);
        new_document = replace_placeholder_in_document(new_document, &placeholder, &replacement)
    }
    Ok(new_document)
}

fn get_placeholder(result_item: &ResultItem) -> String {
    "{{".to_string() + result_item.key_name.as_str() + "}}"
}

fn parse_single_value_into_string(single_value: &SingleValue) -> String {
    match single_value {
        SingleValue::Number(number) => number.to_string(),
        SingleValue::String(string) => string.clone(),
        SingleValue::Boolean(boolean) => boolean.to_string(),
        SingleValue::NumberArray(array) => array.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","),
        SingleValue::StringArray(array) => array.join(","),
    }
}

fn parse_single_value_into_number(single_value: &SingleValue) -> f64 {
    match single_value {
        SingleValue::Number(number) => *number,
        SingleValue::String(string) => string.parse::<f64>().unwrap(),
        SingleValue::Boolean(boolean) => {
            if *boolean {
                1.0
            } else {
                0.0
            }
        },
        SingleValue::NumberArray(array) => array.iter().sum(),
        SingleValue::StringArray(array) => array.iter().map(|x| x.parse::<f64>().unwrap()).sum(),
    }
}

fn parse_render_format_into_number_format(render_format: &RenderFormat) -> Result<NumberFormat, GeneratorError> {
    match render_format {
        RenderFormat::NumberFormat(number_format) => {
            Ok(NumberFormat {
                precision: number_format.precision
            })
        }
        _ => {
            Err(GeneratorError::SystemError("render format is not number format".to_string()))
        }
    }
}

fn parse_render_format_into_image_format(render_format: &RenderFormat) -> Result<ImageFormat, GeneratorError> {
    match render_format {
        RenderFormat::ImageFormat(image_format) => {
            Ok(ImageFormat {
                width: image_format.width,
                height: image_format.height,
                mode: image_format.mode.clone(),
            })
        }
        _ => {
            Err(GeneratorError::SystemError("render format is not image format".to_string()))
        }
    }
}