mod parser;
mod reader;
mod renderer;

use std::{fs::File, io::Read};

use docx_rs::{read_docx, DocumentChild, ParagraphChild, RunChild};
use parser::{data_source::DataSource, GenerationRequest};

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
            "name": [
                {
                    "street": "123 Main St",
                    "city": "Anytown"
                }
            ],
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
    let data_source = DataSource::new(request.data_item).unwrap();

    let mut file = File::open("./hello.docx").unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    let document = read_docx(&buf).unwrap();
    
    for element in document.document.children {
        match element {
            DocumentChild::Paragraph(paragraph) => {
                for para in paragraph.children.clone() {
                    match para {
                        ParagraphChild::Run(run) => {
                            for run_child in run.children {
                                match run_child {
                                    RunChild::Text(text) => {
                                        println!("{}", text.text);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                println!("=====================");
                let new_paragraph = renderer::document::refactor_paragraph(&paragraph, &"{{abc}}".to_string());
                for para in new_paragraph.children {
                    match para {
                        ParagraphChild::Run(run) => {
                            for run_child in run.children {
                                match run_child {
                                    RunChild::Text(text) => {
                                        println!("{}", text.text);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
