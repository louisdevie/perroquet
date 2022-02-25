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
}

impl BitAnd for Style {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            foreground: if self.foreground == Color::INHERIT {
                rhs.foreground
            } else {
                self.foreground
            },
            background: if self.background == Color::INHERIT {
                rhs.background
            } else {
                self.background
            },
            bold: if self.bold == Feature::INHERIT {
                rhs.bold
            } else {
                self.bold
            },
            italic: if self.italic == Feature::INHERIT {
                rhs.italic
            } else {
                self.italic
            },
            decoration: if self.decoration == Decoration::INHERIT {
                rhs.decoration
            } else {
                self.decoration
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleSpan {
    pub start: usize,
    pub end: usize,
    pub style: Style,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain() {
        assert_eq!(
            Style::plain(),
            Style {
                foreground: Color::INHERIT,
                background: Color::INHERIT,
                bold: Feature::INHERIT,
                italic: Feature::INHERIT,
                decoration: Decoration::INHERIT,
            }
        );
    }

    #[test]
    fn test_bitand() {
        assert_eq!(
            Style {
                foreground: Color::INHERIT,
                background: Color::INHERIT,
                bold: Feature::INHERIT,
                italic: Feature::INHERIT,
                decoration: Decoration::INHERIT,
            } & Style {
                foreground: Color::RED,
                background: Color::GREEN,
                bold: Feature::ENABLED,
                italic: Feature::DISABLED,
                decoration: Decoration::UNDERLINE,
            },
            Style {
                foreground: Color::RED,
                background: Color::GREEN,
                bold: Feature::ENABLED,
                italic: Feature::DISABLED,
                decoration: Decoration::UNDERLINE,
            }
        );

        assert_eq!(
            Style {
                foreground: Color::BLUE,
                background: Color::PURPLE,
                bold: Feature::DISABLED,
                italic: Feature::ENABLED,
                decoration: Decoration::NONE,
            } & Style {
                foreground: Color::RED,
                background: Color::GREEN,
                bold: Feature::ENABLED,
                italic: Feature::DISABLED,
                decoration: Decoration::UNDERLINE,
            },
            Style {
                foreground: Color::BLUE,
                background: Color::PURPLE,
                bold: Feature::DISABLED,
                italic: Feature::ENABLED,
                decoration: Decoration::NONE,
            }
        );
    }
}
