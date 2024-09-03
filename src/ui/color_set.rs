use ratatui::prelude::Color;

pub struct ColorSet(Color, Color);

impl ColorSet {
    pub const fn new(fg: Color, bg: Color) -> Self {
        Self(fg, bg)
    }
    pub const fn fg(&self) -> Color {
        self.0
    }

    pub const fn bg(&self) -> Color {
        self.1
    }
}
