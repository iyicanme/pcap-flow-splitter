use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Style, Text};
use ratatui::widgets::{Cell, HighlightSpacing, Row, Table, TableState};

use crate::ui::style;

pub fn draw<'a>(
    frame: &mut Frame,
    rect: Rect,
    header: impl Iterator<Item=&'a str>,
    rows: impl Iterator<Item=(Style, impl Iterator<Item=&'a str>)>,
    selected: usize,
    table_state: &mut TableState,
) {
    let header = header
        .map(Cell::from)
        .collect::<Row>()
        .style(style::table::HEADER)
        .height(1);

    let selected_style = get_row_style_by_index(selected, true);
    let rows = rows.enumerate().map(|(index, (_, items))| row_from_index_and_items(index, items));

    let select_bar = " █ ";
    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Min(15),
            Constraint::Min(15),
        ],
    )
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            select_bar.into(),
            select_bar.into(),
            "".into(),
        ]))
        .style(style::TABLE)
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(table, rect, table_state);
}

fn row_from_index_and_items<'a>(index: usize, items: impl Iterator<Item=&'a str>) -> Row<'a> {
    items
        .map(Cell::new)
        .collect::<Row>()
        .style(get_row_style_by_index(index, false))
}

const fn get_row_style_by_index(index: usize, selected: bool) -> Style {
    match (index % 2, selected) {
        (0, false) => style::table::EVEN,
        (0, true) => style::table::selected::EVEN,
        (_, false) => style::table::ODD,
        (_, true) => style::table::selected::ODD,
    }
}
