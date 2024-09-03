use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Style, Text};
use ratatui::widgets::{Cell, HighlightSpacing, Row, Table, TableState};

use crate::ui::style;
use crate::ui::style::table::get_row_style_by_index;

pub fn draw<'a>(
    frame: &mut Frame,
    rect: Rect,
    header: impl Iterator<Item=&'a str>,
    rows: impl Iterator<Item=Row<'a>>,
    selected: usize,
    table_state: &mut TableState,
) {
    let header = header
        .map(Cell::from)
        .collect::<Row>()
        .style(style::table::header())
        .height(1);

    let selected_style = get_row_style_by_index(selected, true);
    
    let select_bar = " █ ";
    let highlight = Text::from(vec!["".into(), select_bar.into(), select_bar.into(), "".into()]);
    
    let table = Table::new(rows, [Constraint::Min(1)])
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(highlight)
        .style(style::TABLE)
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(table, rect, table_state);
}
