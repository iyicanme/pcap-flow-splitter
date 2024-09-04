use ratatui::style::Color;

use crate::ui::color_set::ColorSet;

pub mod table {
    use ratatui::style::Color;

    use crate::ui::color_set::ColorSet;

    pub const EVEN: ColorSet = ColorSet::new(Color::White, Color::Black);
    pub const ODD: ColorSet = ColorSet::new(Color::LightBlue, Color::Black);

    pub mod selected {
        use ratatui::prelude::Color;

        use crate::ui::color_set::ColorSet;

        pub const EVEN: ColorSet = ColorSet::new(Color::White, Color::Gray);
        pub const ODD: ColorSet = ColorSet::new(Color::LightBlue, Color::Gray);
    }
}

pub mod tabs {
    use ratatui::prelude::Color;

    use crate::ui::color_set::ColorSet;

    pub const SELECTED: ColorSet = ColorSet::new(Color::White, Color::Rgb(0x61, 0x61, 0x61));
    pub const UNSELECTED: ColorSet = ColorSet::new(Color::Rgb(0xBC, 0xBE, 0xE4), Color::Black);
}

pub const HEADER: ColorSet = ColorSet::new(Color::White, Color::Black);
pub const FOOTER: ColorSet = ColorSet::new(Color::White, Color::Black);
pub const TABLE: ColorSet = ColorSet::new(Color::White, Color::Black);
