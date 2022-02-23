use crate::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Style {
    pub foreground: Color,
}

impl Style {
    pub fn plain() -> Self {
        Self {
            foreground: Color::INHERIT,
        }
    }
}

pub struct StyleSpan {
    pub start: usize,
    pub end: usize,
    pub style: Style,
}
