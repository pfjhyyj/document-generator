use docx_rs::{Paragraph, ParagraphChild, Run, RunChild, RunProperty, Text};
use regex::Regex;


fn generate_new_run(run: &Run, run_child: &Vec<RunChild>) -> Box<Run> {
    let mut new_run = run.clone();
    new_run.children = Vec::new();
    for child in run_child {
        new_run.children.push(child.clone());
    }
    Box::new(new_run)
}

fn generate_new_paragraph(paragraph: &mut Paragraph, paragraph_child: &Vec<ParagraphChild>) -> Paragraph {
    let mut new_paragraph = paragraph.clone();
    new_paragraph.children = Vec::new();
    for child in paragraph_child {
        new_paragraph.children.push(child.clone());
    }
    new_paragraph
}

// refactor paragraph to make sure that the placeholder is in the same run
pub fn refactor_paragraph(paragraph: &Paragraph, placeholder: &String) -> Paragraph {
    let mut new_paragraph = Paragraph::new();
    let paragraph_children = paragraph.children.clone();
    let mut i = 0;
    // ensure searching continuously for the placeholder
    let mut searching_paragraph_children: Vec<ParagraphChild> = Vec::new();
    let mut searching_text = String::new();
    while i < paragraph_children.len() {
        let current_child = paragraph_children[i].clone();
        match current_child.clone() {
            ParagraphChild::Run(run) => {
                let run_children = run.children.clone();
                let mut is_text_run = true;
                for run_child in run_children.clone() {
                    match run_child {
                        RunChild::Text(text) => {
                            searching_text.push_str(&text.text);
                        }
                        _ => {
                            is_text_run = false;
                            break;
                        }
                    }
                }
                if !is_text_run {
                    // meet with non-run element, reset searching and push to new paragraph
                    searching_text = String::new();
                    for refactored_paragraph_child in searching_paragraph_children {
                        new_paragraph.children.push(refactored_paragraph_child);
                    }
                    searching_paragraph_children = Vec::new();
                } else {
                    searching_paragraph_children.push(current_child.clone());
                }
                
                if searching_text.contains(placeholder) {
                    // found the placeholder, generate new run and push to new paragraph
                    let refactored_paragraph_child = refactor_paragraph_children_by_placeholder(&searching_paragraph_children, &placeholder);
                    for refactored_paragraph_child in refactored_paragraph_child {
                        new_paragraph.children.push(refactored_paragraph_child);
                    }
                    searching_paragraph_children = Vec::new();
                }
            }
            // ParagraphChild::CommentStart(_) => {
            //     // meet with non-run element, reset searching and push to new paragraph
            //     searching_text = String::new();
            //     for refactored_paragraph_child in searching_paragraph_children {
            //         new_paragraph.children.push(refactored_paragraph_child);
            //     }
            //     searching_paragraph_children = Vec::new();
            // }
            // ParagraphChild::CommentEnd(_) => {
            //     // meet with non-run element, reset searching and push to new paragraph
            //     searching_text = String::new();
            //     for refactored_paragraph_child in searching_paragraph_children {
            //         new_paragraph.children.push(refactored_paragraph_child);
            //     }
            //     searching_paragraph_children = Vec::new();
            // }
            // ParagraphChild::Delete(_) => {
            //     // meet with non-run element, reset searching and push to new paragraph
            //     searching_text = String::new();
            //     for refactored_paragraph_child in searching_paragraph_children {
            //         new_paragraph.children.push(refactored_paragraph_child);
            //     }
            //     searching_paragraph_children = Vec::new();
            // }
            // ParagraphChild::Insert(_) => {
            //     // meet with non-run element, reset searching and push to new paragraph
            //     searching_text = String::new();
            //     for refactored_paragraph_child in searching_paragraph_children {
            //         new_paragraph.children.push(refactored_paragraph_child);
            //     }
            //     searching_paragraph_children = Vec::new();
            // }
            // ParagraphChild::Hyperlink(_) => {
            //     // meet with non-run element, reset searching and push to new paragraph
            //     searching_text = String::new();
            //     for refactored_paragraph_child in searching_paragraph_children {
            //         new_paragraph.children.push(refactored_paragraph_child);
            //     }
            //     searching_paragraph_children = Vec::new();
            // }
            // ParagraphChild::StructuredDataTag(_) => {
            //     // meet with non-run element, reset searching and push to new paragraph
            //     searching_text = String::new();
            //     for refactored_paragraph_child in searching_paragraph_children {
            //         new_paragraph.children.push(refactored_paragraph_child);
            //     }
            //     searching_paragraph_children = Vec::new();
            // }
            // ParagraphChild::BookmarkStart(_) => {
            //     // meet with non-run element, reset searching and push to new paragraph
            //     searching_text = String::new();
            //     for refactored_paragraph_child in searching_paragraph_children {
            //         new_paragraph.children.push(refactored_paragraph_child);
            //     }
            //     searching_paragraph_children = Vec::new();
            // }
            // ParagraphChild::BookmarkEnd(_) => {
            //     // meet with non-run element, reset searching and push to new paragraph
            //     searching_text = String::new();
            //     for refactored_paragraph_child in searching_paragraph_children {
            //         new_paragraph.children.push(refactored_paragraph_child);
            //     }
            //     searching_paragraph_children = Vec::new();
            // }
            _ => {
                // // meet with non-run element, reset searching and push to new paragraph
                // searching_text = String::new();
                // for refactored_paragraph_child in searching_paragraph_children {
                //     new_paragraph.children.push(refactored_paragraph_child);
                // }
                // searching_paragraph_children = Vec::new();
            }
        }
        i += 1;
    }
    // when the paragraph is end, check if the placeholder is in the last run
    // if !searching_text.is_empty() && searching_text.contains(placeholder) {
    //     let refactored_paragraph_child = refactor_paragraph_children_by_placeholder(&searching_paragraph_children, &placeholder);
    //     for refactored_paragraph_child in refactored_paragraph_child {
    //         new_paragraph.children.push(refactored_paragraph_child);
    //     }
    // }
    // push the remaining paragraph children to the new paragraph
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
    let mut run_indices: Vec<(usize, usize)> = Vec::new(); // (run index, run_child index)

    // Create the full paragraph text and track the positions of each RunChild
    for (i, paragraph_child) in paragraph_children.iter().enumerate() {
        if let ParagraphChild::Run(run) = paragraph_child {
            for (j, run_child) in run.children.iter().enumerate() {
                if let RunChild::Text(text) = run_child {
                    run_indices.push((i, j));
                    paragraph_text.push_str(&text.text);
                }
            }
        }
    }

    // Find the placeholder in the combined text
    let escaped_placeholder = escape_braces(placeholder);
    let re = Regex::new(&format!("{}", escaped_placeholder)).unwrap();
    let caps = re.captures(&paragraph_text);

    if caps.is_none() {
        // No placeholder found
        return paragraph_children.clone();
    }

    let caps = caps.unwrap();
    let full_match = caps.get(0).unwrap();
    let start = full_match.start();
    let end = full_match.end();

    let mut new_paragraph_children: Vec<ParagraphChild> = Vec::new();
    let mut combined_text = String::new();
    let mut current_run_property: Option<RunProperty> = None;
    let mut in_placeholder = false;

    let mut char_index = 0;

    for (i, paragraph_child) in paragraph_children.iter().enumerate() {
        if let ParagraphChild::Run(run) = paragraph_child {
            let mut new_run_children: Vec<RunChild> = Vec::new();

            for (j, run_child) in run.children.iter().enumerate() {
                if let RunChild::Text(text) = run_child {
                    let text_length = text.text.len();
                    
                    if char_index <= start && char_index + text_length > start {
                        // Placeholder starts in this run
                        in_placeholder = true;
                        combined_text.push_str(&text.text[start - char_index..]);
                        if current_run_property.is_none() {
                            current_run_property = Some(run.run_property.clone());
                        }
                    } else if in_placeholder && char_index < end {
                        // Placeholder is continuing in this run
                        combined_text.push_str(&text.text);
                    } else if char_index >= end {
                        // Placeholder has ended
                        if in_placeholder {
                            in_placeholder = false;
                            let new_combined_run = Run {
                                run_property: current_run_property.take().unwrap(),
                                children: vec![RunChild::Text(Text::new(&combined_text))],
                            };
                            new_paragraph_children.push(ParagraphChild::Run(Box::new(new_combined_run)));
                            combined_text.clear();
                        }
                        new_run_children.push(RunChild::Text(text.clone()));
                    } else {
                        // Regular text before or after placeholder
                        new_run_children.push(RunChild::Text(text.clone()));
                    }

                    char_index += text_length;
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
