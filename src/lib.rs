#[deny(missing_docs)]
extern crate substring;

mod style;

mod string;
pub use string::RichString;

mod attributes;
pub use attributes::Color;
