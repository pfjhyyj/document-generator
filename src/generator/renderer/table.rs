use docx_rs::{Table, TableCellContent, TableChild, TableRowChild};

use super::{paragraph::replace_placeholder_in_paragraph, replacement::Replacement};


pub fn replace_placeholder_in_table(table: &Table, placeholder: &String, replacement: &Replacement) -> Table {
    let mut new_table = table.clone();
    new_table.rows = Vec::new();
    for row in table.rows.iter() {
        let new_table_child = row.clone();
        let TableChild::TableRow(mut old_rows) = new_table_child.clone();
        let TableChild::TableRow(mut new_rows) = new_table_child;
        new_rows.cells = Vec::new();
        for cell in old_rows.cells.iter_mut() {
            let mut new_cell = cell.clone();
            new_cell = replace_placeholder_in_cell(&new_cell, placeholder, replacement);
            new_rows.cells.push(new_cell);
        }
        new_table.rows.push(TableChild::TableRow(new_rows));
    }
    new_table
}

pub fn replace_placeholder_in_cell(row_child: &TableRowChild, placeholder: &String, replacement: &Replacement) -> TableRowChild {
    match row_child {
        TableRowChild::TableCell(cell) => {
            let mut new_cell = cell.clone();
            new_cell.children = Vec::new();
            for cell_child in cell.children.iter() {
                match cell_child {
                    TableCellContent::Paragraph(paragraph) => {
                        let new_paragraph = replace_placeholder_in_paragraph(paragraph, placeholder, replacement);
                        new_cell.children.push(TableCellContent::Paragraph(new_paragraph));
                    }
                    _ => {
                        new_cell.children.push(cell_child.clone());
                    }
                }
            }
            TableRowChild::TableCell(new_cell)
        }
    }
}