use docx_rs::{Document, DocumentChild};

use super::{paragraph::replace_placeholder_in_paragraph, replacement::Replacement, table::replace_placeholder_in_table};

pub fn replace_placeholder_in_document(document: Document, placeholder: &String, replacement: &Replacement) -> Document {
    let mut new_document = Document::new();
    for document_child in document.children.iter() {
        match document_child {
            DocumentChild::Paragraph(paragraph) => {
                let new_paragraph = replace_placeholder_in_paragraph(paragraph, placeholder, replacement);
                new_document.children.push(DocumentChild::Paragraph(Box::new(new_paragraph)));
            }
            DocumentChild::Table(table) => {
                let new_table = replace_placeholder_in_table(table, placeholder, replacement);
                new_document.children.push(DocumentChild::Table(Box::new(new_table)));
            }
            _ => {
                new_document.children.push(document_child.clone());
            }
        }
    }
    new_document
}