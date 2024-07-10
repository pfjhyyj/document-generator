mod generator;
mod error;

fn main() {
    let json_str = r#"
    {
        "templateFile": "template.docx",
        "dataItem": {
            "name": "John Doe",
            "age": 30,
            "image": "http://i0.hdslb.com/bfs/new_dyn/f3e8656b36556c95941b703f5b937e4343536.png"
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
            },
            {
                "name": "Image",
                "dataType": 1,
                "keyName": "image",
                "renderMode": 3
            },
            {
                "name": "Age",
                "dataType": 1,
                "keyName": "age",
                "renderMode": 1,
                "properties": [
                    {
                        "name": "Age",
                        "dataType": 1,
                        "keyName": "age",
                        "renderMode": 1
                    }
                ]
            }
        ]
    }
    "#;
    
    let result = generator::generate_new_document(&json_str.to_string());
    match result {
        Ok(_) => println!("Document generated successfully"),
        Err(e) => eprintln!("Error: {:?}", e)
    }
}
