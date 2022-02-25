#![deny(missing_docs)]
#![deny(warnings)]

//! Provides styling for terminal output
//!
//! Made to work with [textflow](https://docs.rs/textflow)

extern crate substring;

mod style;
pub use style::Style;

mod string;
pub use string::RichString;

mod attributes;
pub use attributes::{Color, Decoration, Feature};

mod shortcuts;
pub use shortcuts::{
    BLACK, BLUE, BOLD, CYAN, GREEN, ITALIC, LIGHTGREY, ON_BLACK, ON_BLUE, ON_CYAN, ON_GREEN,
    ON_LIGHTGREY, ON_PURPLE, ON_RED, ON_YELLOW, PURPLE, RED, UNDERLINE, YELLOW,
};

/// Apply a style to a string literal
#[macro_export]
macro_rules! style {
    ($string: literal) => {
        RichString::from($string, Style::plain())
    };
    ($string: literal, $style: expr) => {
        RichString::from($string, $style)
    };
    ($string: expr, $style: expr) => {
        ($string).into_complemented($style)
    };
}

#[test]
fn test_macro() {
    let mut expected = RichString::from("A very ", YELLOW);
    expected.push(&RichString::from("important", YELLOW & BOLD));
    expected.push(&RichString::from(" issue", YELLOW));

    assert_eq!(
        style!("A very " + style!("important", BOLD) + " issue", YELLOW),
        expected
    );
}
