#![deny(missing_docs)]
#![deny(warnings)]
extern crate substring;

mod style;
pub use style::Style;

mod string;
pub use string::RichString;

mod attributes;
pub use attributes::{Color, Decoration, Feature};

const YELLOW: Style = Style {
    foreground: Color::YELLOW,
    background: Color::INHERIT,
    bold: Feature::INHERIT,
    italic: Feature::INHERIT,
    decoration: Decoration::INHERIT,
};

const BOLD: Style = Style {
    foreground: Color::INHERIT,
    background: Color::INHERIT,
    bold: Feature::ENABLED,
    italic: Feature::INHERIT,
    decoration: Decoration::INHERIT,
};

#[macro_export]
macro_rules! style {
    ($string: literal) => {
        RichString::from($string, perroquet::style::Style::plain())
    };
    ($string: literal, $style: expr) => {
        RichString::from($string, $style)
    };
    ($string: expr, $style: expr) => {
        ($style).apply_to($string)
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
