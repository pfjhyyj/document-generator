use docx_rs::{Paragraph, ParagraphChild, Run, RunChild, RunProperty, Text};
use regex::Regex;

// refactor paragraph to make sure that the placeholder is in the same run
pub fn refactor_paragraph(paragraph: &Paragraph, placeholder: &String) -> Paragraph {
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

            for run_child in run.children.iter() {
                if let RunChild::Text(text) = run_child {
                    let mut text_length = text.text.len();

                    loop {
                        if char_index <= start && char_index + text_length > start {
                            // Placeholder starts in this run
                            in_placeholder = true;
                            combined_text.push_str(&text.text[start - char_index..]);
                            if current_run_property.is_none() {
                                current_run_property = Some(run.run_property.clone());
                            }
                            char_index += text_length;
                            break;
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
                                run_property: current_run_property.take().unwrap(),
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

    if in_placeholder {
        // Handle the case where the placeholder is at the end of the paragraph
        let new_combined_run = Run {
            run_property: current_run_property.take().unwrap(),
            children: vec![RunChild::Text(Text::new(&combined_text))],
        };
        new_paragraph_children.push(ParagraphChild::Run(Box::new(new_combined_run)));
    }

    new_paragraph_children
}
