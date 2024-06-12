mod parser;

use parser::GenerationRequest;
use serde_json::{Value, Error};
use std::collections::HashMap;

fn main() {
    let json_str = r#"
    {
        "templateFile": "template.docx",
        "dataItem": {
            "name": "John Doe",
            "age": 30,
            "address": {
                "street": "123 Main St",
                "city": "Anytown"
            },
            "scores": [85, 92, 78]
        },
        "resultItem": [
            {
                "name": "Name",
                "dataType": 1,
                "keyName": "name",
                "renderMode": 1,
                "properties": [
                    {
                        "name": "Name",
                        "dataType": 1,
                        "keyName": "name",
                        "renderMode": 1,
                        "properties": [
                            {
                                "name": "Name",
                                "dataType": 1,
                                "keyName": "name",
                                "renderMode": 1
                            }
                        ]
                    }
                ]
            },
            {
                "name": "Age",
                "dataType": 2,
                "keyName": "age",
                "renderMode": 2
            }
        ]
    }
    "#;
    let request: GenerationRequest = serde_json::from_str(&json_str).unwrap();
    println!("{:#?}", request);
}
