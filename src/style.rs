use crate::{Color, Decoration, Feature};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Style {
    pub foreground: Color,
    pub background: Color,
    pub bold: Feature,
    pub italic: Feature,
    pub dim: Feature,
    pub bright: Feature,
    pub decoration: Decoration,
    pub inverted: Feature,
}

impl Style {
    pub fn plain() -> Self {
        Self {
            foreground: Color::INHERIT,
            background: Color::INHERIT,
            bold: Feature::INHERIT,
            italic: Feature::INHERIT,
            dim: Feature::INHERIT,
            bright: Feature::INHERIT,
            decoration: Decoration::INHERIT,
            inverted: Feature::INHERIT,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StyleSpan {
    pub start: usize,
    pub end: usize,
    pub style: Style,
}
