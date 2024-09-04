use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Tabs, Widget};

use crate::ui::style;

pub fn draw(frame: &mut Frame, area: Rect, index: usize, tab_text: impl Iterator<Item=String>) {
    Tabs::new(tab_text)
        .style(style::tabs::UNSELECTED)
        .highlight_style(style::tabs::SELECTED)
        .select(index)
        .padding("", "")
        .divider(" ")
        .render(area, frame.buffer_mut(),
        );
}