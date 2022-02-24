use crate::RichString;
use crate::{Color, Decoration, Feature};
use std::ops::BitAnd;

/// A style to be applied on text
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Style {
    /// Text color
    pub foreground: Color,
    /// Background color
    pub background: Color,
    /// Bold
    pub bold: Feature,
    /// Italic
    pub italic: Feature,
    /// Underline
    pub decoration: Decoration,
}

impl Style {
    /// Returns a style that inherits all its properties
    pub fn plain() -> Self {
        Self {
            foreground: Color::INHERIT,
            background: Color::INHERIT,
            bold: Feature::INHERIT,
            italic: Feature::INHERIT,
            decoration: Decoration::INHERIT,
        }
    }

    /// Creates
    pub fn apply_to(&self, string: RichString) -> RichString {
        string
    }
}

impl BitAnd for Style {
    type Output = Self;

    fn bitand(self, _rhs: Self) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct StyleSpan {
    pub start: usize,
    pub end: usize,
    pub style: Style,
}
