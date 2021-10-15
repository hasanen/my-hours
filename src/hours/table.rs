//! Print time entries to terminal in table
use crate::hours::types;
use prettytable::{Attr, Cell, Row, Table};

/// Prints given entries to terminal
pub fn print(time_entries: &types::TimeEntries) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
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
    table.printstd();
}

fn header_cell(title: &str) -> Cell {
    return Cell::new(title).with_style(Attr::Bold);
}
