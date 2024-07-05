use docx_rs::{Document, DocumentChild, ParagraphChild, RunChild};


pub fn print_document(document: &Document) {
    for element in document.children.clone() {
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
            }
            _ => {}
        }
    }
}