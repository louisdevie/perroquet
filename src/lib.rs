#[deny(missing_docs)]
extern crate substring;

pub mod style;

mod string;
pub use string::RichString;

mod attributes;
pub use attributes::{Color, Decoration, Feature};

#[macro_export]
macro_rules! style {
    ($string: literal) => {
        perroquet::RichString::from($string, perroquet::style::Style::plain())
    };
}
