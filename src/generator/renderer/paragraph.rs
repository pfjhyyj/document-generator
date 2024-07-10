use docx_rs::{Paragraph, ParagraphChild, Pic, Run, RunChild, RunProperty, Text};
use regex::Regex;

use super::replacement::Replacement;

pub fn replace_placeholder_in_paragraph(paragraph: &Paragraph, placeholder: &String, replacement: &Replacement) -> Paragraph {
    let refactored_paragraph = refactor_paragraph(paragraph, placeholder);
    let mut new_paragraph = refactored_paragraph.clone();
    new_paragraph.children = Vec::new();
    for paragraph_child in refactored_paragraph.children.iter() {
        match paragraph_child {
            ParagraphChild::Run(run) => {
                let mut replace_by_replacement = false;
                for run_child in run.children.iter() {
                    if let RunChild::Text(text) = run_child {
                        if text.text.contains(placeholder) {
                            let new_run = generate_new_run_by_replacement(run, replacement);
                            new_paragraph.children.push(ParagraphChild::Run(Box::new(new_run)));
                            replace_by_replacement = true;
                            break;
                        }
                    }
                }
                if !replace_by_replacement {
                    new_paragraph.children.push(paragraph_child.clone());
                }
            }
            _ => {
                new_paragraph.children.push(paragraph_child.clone());
            }
        }
    }
    new_paragraph
}

fn generate_new_run_by_replacement(run: &Run, replacement: &Replacement) -> Run {
    let mut new_run = run.clone();
    new_run.children = Vec::new();
    match replacement {
        Replacement::STRING(value) => {
            let new_text = value.value.clone();
            new_run = new_run.add_text(new_text);
        }
        Replacement::NUMBER(value) => {
            let new_text = format!("{:.1$}", value.value, value.precision as usize);
            new_run = new_run.add_text(new_text);
        }
        Replacement::CHECKBOX(value) => {
            let new_text = if value.value { "☑" } else { "☐" };
            new_run = new_run.add_text(new_text);
        }
        Replacement::IMAGE(value) => {
            let pic = Pic::new(&value.value).size(200000, 200000);
            new_run = new_run.add_image(pic);
        }
        Replacement::EMPTY => {
            // Do nothing
        }
        _ => {
            // TODO
        }
    }
    new_run
}



// refactor paragraph to make sure that the placeholder is in the same run
fn refactor_paragraph(paragraph: &Paragraph, placeholder: &String) -> Paragraph {
    let mut new_paragraph = Paragraph::new();
    let paragraph_children = paragraph.children.clone();
    let searching_paragraph_children = refactor_paragraph_children_by_placeholder(&paragraph_children, placeholder);
    for refactored_paragraph_child in searching_paragraph_children {
        new_paragraph.children.push(refactored_paragraph_child);
    }
    new_paragraph
}

fn escape_braces(s: &str) -> String {
    s.replace("{", "\\{").replace("}", "\\}")
}

fn refactor_paragraph_children_by_placeholder(paragraph_children: &Vec<ParagraphChild>, placeholder: &String) -> Vec<ParagraphChild> {
    let mut paragraph_text = String::new();

    // Create the full paragraph text and track the positions of each RunChild
    for paragraph_child in paragraph_children.iter() {
        if let ParagraphChild::Run(run) = paragraph_child {
            for run_child in run.children.iter() {
                if let RunChild::Text(text) = run_child {
                    paragraph_text.push_str(&text.text);
                }
            }
        }
    }

    // Find the placeholder in the combined text
    let escaped_placeholder = escape_braces(placeholder);
    let re = Regex::new(&format!("{}", escaped_placeholder)).unwrap();
    let matches: Vec<(usize, usize)> = re.find_iter(&paragraph_text)
        .map(|mat| (mat.start(), mat.end()))
        .collect();

    if matches.is_empty() {
        // No placeholder found
        return paragraph_children.clone();
    }

    let mut placeholder_index = 0;
    let mut start = matches.get(placeholder_index).unwrap().0;
    let mut end = matches.get(placeholder_index).unwrap().1;

    let mut new_paragraph_children: Vec<ParagraphChild> = Vec::new();
    let mut combined_text = String::new();
    let mut current_run_property: Option<RunProperty> = None;
    let mut in_placeholder = false;

    let mut char_index = 0;

    for paragraph_child in paragraph_children.iter() {
        if let ParagraphChild::Run(run) = paragraph_child {
            let mut new_run_children: Vec<RunChild> = Vec::new();
            current_run_property = Some(run.run_property.clone());

            for run_child in run.children.iter() {
                if let RunChild::Text(text) = run_child {
                    let mut text_length = text.text.len();

                    loop {
                        if text_length == 0 {
                            break;
                        } else if char_index <= start && char_index + text_length > start {
                            // Placeholder start or in this run
                            if char_index + text_length >= end {
                                // Placeholder is in this run
                                combined_text.push_str(&text.text[start - char_index..end - char_index]);
                                let new_combined_run = Run {
                                    run_property: current_run_property.clone().unwrap(),
                                    children: vec![RunChild::Text(Text::new(&combined_text))],
                                };
                                new_paragraph_children.push(ParagraphChild::Run(Box::new(new_combined_run)));
                                text_length = text_length - (end - start);
                                char_index += end - start;
                                combined_text.clear();

                                placeholder_index += 1;
                                if placeholder_index < matches.len() {
                                    start = matches.get(placeholder_index).unwrap().0;
                                    end = matches.get(placeholder_index).unwrap().1;
                                }
                                continue;
                            } else {
                                // Placeholder is starting in this run
                                in_placeholder = true;
                                combined_text.push_str(&text.text[start - char_index..]);
                                char_index += text_length;
                                break;
                            }
                        } else if in_placeholder && char_index + text_length < end {
                            // Placeholder is continuing in this run
                            combined_text.push_str(&text.text);
                            char_index += text_length;
                            break;
                        } else if in_placeholder && char_index + text_length >= end {
                            // Placeholder is ended in this run
                            in_placeholder = false;
                            combined_text.push_str(&text.text[..end - char_index]);
                            let new_combined_run = Run {
                                run_property: current_run_property.clone().unwrap(),
                                children: vec![RunChild::Text(Text::new(&combined_text))],
                            };
                            new_paragraph_children.push(ParagraphChild::Run(Box::new(new_combined_run)));
                            text_length = text_length - (end - char_index);
                            char_index += end - char_index;
                            combined_text.clear();

                            placeholder_index += 1;
                            if placeholder_index < matches.len() {
                                start = matches.get(placeholder_index).unwrap().0;
                                end = matches.get(placeholder_index).unwrap().1;
                            }
                            continue;
                        } else {
                            // Regular text before or after placeholder
                            if text_length > 0 {
                                let text = Text::new(&text.text[0..text_length]);
                                new_run_children.push(RunChild::Text(text.clone()));
                                char_index += text_length;
                            }
                            break;
                        }
                    }
                }
            }

            if !new_run_children.is_empty() {
                let new_run = Run {
                    run_property: run.run_property.clone(),
                    children: new_run_children,
                };
                new_paragraph_children.push(ParagraphChild::Run(Box::new(new_run)));
            }
        }
    }

    new_paragraph_children
}
