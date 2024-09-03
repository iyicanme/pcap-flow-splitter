use ratatui::style::{Style, Stylize};

use crate::ui::color;

pub const HEADER: Style = Style::new().fg(color::HEADER.fg()).bg(color::HEADER.bg());

pub const FOOTER: Style = Style::new().fg(color::FOOTER.fg()).bg(color::FOOTER.bg());

pub const TABLE: Style = Style::new().fg(color::TABLE.fg()).bg(color::TABLE.bg());

pub mod table {
    use ratatui::prelude::Style;

    use crate::ui::style::color;

    pub const EVEN: Style = Style::new()
        .fg(color::table::EVEN.fg())
        .bg(color::table::EVEN.bg());
    pub const ODD: Style = Style::new()
        .fg(color::table::ODD.fg())
        .bg(color::table::ODD.bg());
    pub const HEADER: Style = ODD;

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
}

pub fn file() -> Style { Style::new() }

pub fn directory() -> Style { Style::new().bold() }

pub fn symbolic_link() -> Style { Style::new().underlined() } 
