mod generator;
mod error;

use std::{fs::File, io::Read, path::Path};

use docx_rs::{read_docx, DocumentChild, Docx};
use generator::parser::{data_source::DataSource, GenerationRequest};

fn main() {
    let json_str = r#"
    {
        "templateFile": "template.docx",
        "dataItem": {
            "name": "John Doe",
            "age": 30
        },
        "resultItem": [
            {
                "name": "Name",
                "dataType": 1,
                "keyName": "name",
                "renderMode": 1
            },
            {
                "name": "Age",
                "dataType": 1,
                "keyName": "age",
                "renderMode": 1
            }
        ]
    }
    "#;
    
    let result = generator::generate_new_document(&json_str.to_string());
    match result {
        Ok(_) => println!("Document generated successfully"),
        Err(e) => println!("Error: {:?}", e)
    }
}
