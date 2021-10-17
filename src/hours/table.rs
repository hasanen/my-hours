//! Print time entries to terminal in table
use crate::hours::types;
use prettytable::{format, Attr, Cell, Row, Table};

/// Prints given entries to terminal
pub fn print(time_entries: &types::TimeEntries) {
    println!("");
    print_hours_table(time_entries);
    println!("");
    println!("");
    print_common_table();
}

fn print_hours_table(time_entries: &types::TimeEntries) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(vec![
        header_cell(&"Project"),
        header_cell(&"Today"),
        header_cell(&"Daily AVG"),
        header_cell(&"This month"),
        header_cell(&"Target"),
        header_cell(&"Billing"),
    ]));
    for project in time_entries.uniq_projects() {
        table.add_row(Row::new(vec![
            Cell::new(&project),
            Cell::new(""),
            Cell::new(""),
            Cell::new(" "),
            Cell::new(""),
            Cell::new(""),
        ]));
    }
    table.add_row(Row::new(vec![
        Cell::new("Total").style_spec("i"),
        Cell::new(""),
        Cell::new(""),
        Cell::new(""),
        Cell::new(""),
        Cell::new(""),
    ]));
    table.printstd();
}
fn print_common_table() {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .padding(0, 2)
        .build();
    table.set_format(format);
    table.add_row(Row::new(vec![
        header_cell(&"Work days left"),
        Cell::new("12").style_spec("r"),
    ]));
    table.add_row(Row::new(vec![
        header_cell(&"Target AVG / day"),
        Cell::new("2").style_spec("r"),
    ]));
    table.add_row(Row::new(vec![
        header_cell(&"Hours left"),
        Cell::new("2").style_spec("r"),
    ]));
    table.printstd();
}
fn header_cell(title: &str) -> Cell {
    return Cell::new(title).with_style(Attr::Bold);
}
