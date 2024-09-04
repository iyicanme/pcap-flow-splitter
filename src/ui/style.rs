use ratatui::style::{Style, Stylize};

use crate::ui::color;

pub const HEADER: Style = Style::new().fg(color::HEADER.fg()).bg(color::HEADER.bg());

pub const FOOTER: Style = Style::new().fg(color::FOOTER.fg()).bg(color::FOOTER.bg());

pub const TABLE: Style = Style::new().fg(color::TABLE.fg()).bg(color::TABLE.bg());

pub mod table {
    use ratatui::prelude::Style;
    use ratatui::style::Stylize;

    use crate::ui::style::color;

    pub const EVEN: Style = Style::new()
        .fg(color::table::EVEN.fg())
        .bg(color::table::EVEN.bg());
    pub const ODD: Style = Style::new()
        .fg(color::table::ODD.fg())
        .bg(color::table::ODD.bg());

    pub fn header() -> Style { ODD }

    pub mod selected {
        use ratatui::prelude::Style;

        use crate::ui::style::color;

        pub const EVEN: Style = Style::new()
            .fg(color::table::selected::EVEN.fg())
            .bg(color::table::selected::EVEN.bg());
        pub const ODD: Style = Style::new()
            .fg(color::table::selected::ODD.fg())
            .bg(color::table::selected::ODD.bg());
    }

    pub const fn get_row_style_by_index(index: usize, selected: bool) -> Style {
        match (index % 2, selected) {
            (0, false) => EVEN,
            (_, false) => ODD,
            (0, true) => selected::EVEN,
            (_, true) => selected::ODD,
        }
    }
}

pub mod tabs {
    use ratatui::style::Style;

    use crate::ui::color;

    pub const SELECTED: Style = Style::new().fg(color::tabs::SELECTED.fg()).bg(color::tabs::SELECTED.bg());
    pub const UNSELECTED: Style = Style::new().fg(color::tabs::UNSELECTED.fg()).bg(color::tabs::UNSELECTED.bg());
}

pub fn file() -> Style { Style::new() }

pub fn directory() -> Style { Style::new().bold() }

pub fn symbolic_link() -> Style { Style::new().underlined() } 
