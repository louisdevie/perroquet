use crate::style::{Style, StyleSpan};
use crate::substring::Substring;
use std::ops::Add;

#[derive(Debug, PartialEq)]
/// String with syle information
pub struct RichString {
    text: String,
    style: Vec<StyleSpan>,
}

impl From<&str> for RichString {
    fn from(value: &str) -> Self {
        Self::from(value, Style::plain())
    }
}

impl RichString {
    /// Creates a new empty string
    pub fn new() -> Self {
        Self {
            text: String::new(),
            style: vec![],
        }
        .into_normalised()
    }

    /// Creates a new string from text and a style
    ///
    /// The style will be applied to the entire string.
    pub fn from(text: &str, style: Style) -> Self {
        Self {
            text: String::from(text),
            style: vec![StyleSpan {
                style,
                start: 0,
                end: text.chars().count(),
            }],
        }
        .into_normalised()
    }

    /// Returns the size of the string *in characters*
    pub fn len(&self) -> usize {
        self.text.chars().count()
    }

    /// Returns a reference to the inner text
    pub fn raw(&self) -> &str {
        &self.text
    }

    /// Returns a slice of the string
    ///
    /// Starts from index `start` to index `end` *in characters*
    /// and keep the style as is.
    pub fn substring(&self, start: usize, end: usize) -> Self {
        let mut style = Vec::new();

        for span in &self.style {
            if start >= span.start {
                if end <= span.end {
                    style.push(StyleSpan {
                        style: span.style,
                        start: 0,
                        end: end - start,
                    });

                    break;
                } else {
                    style.push(StyleSpan {
                        style: span.style,
                        start: 0,
                        end: span.end - start,
                    });
                }
            } else {
                if end <= span.end {
                    style.push(StyleSpan {
                        style: span.style,
                        start: span.start - start,
                        end: end - start,
                    });
                    break;
                } else {
                    style.push(StyleSpan {
                        style: span.style,
                        start: span.start - start,
                        end: span.end - start,
                    });
                }
            }
        }

        Self {
            text: String::from(self.text.substring(start, end)),
            style,
        }
        .into_normalised()
    }

    /// Returns the style for a character
    pub fn style_at(&self, index: usize) -> Style {
        for span in &self.style {
            if index < span.end {
                return span.style;
            }
        }
        return Style::plain();
    }

    /// Append another string
    pub fn push(&mut self, other: &Self) {
        if other.style.len() > 0 {
            if self.style.len() == 0 {
                *self = other.clone();
            } else {
                let last = self.style.len() - 1;

                for span in &other.style {
                    self.style.push(StyleSpan {
                        style: span.style,
                        start: span.start + self.style[last].end,
                        end: span.end + self.style[last].end,
                    })
                }

                self.text.push_str(&other.text);
            }
            self.normalise();
        }
    }

    /// Creates a new string without style and append it
    pub fn push_plain(&mut self, other: &str) {
        self.push(&Self::from(other, Style::plain()));
    }

    /// Creates a new string with the style of the last character and append it
    pub fn push_extend(&mut self, other: &str) {
        if self.style.len() == 0 {
            self.push(&Self::from(other, Style::plain()));
        } else {
            self.push(&Self::from(other, self.style[self.style.len() - 1].style));
        }
    }

    /// Insert a string at the given index
    pub fn insert(&mut self, index: usize, other: &Self) {
        if other.style.len() > 0 {
            if self.style.len() == 0 {
                *self = other.clone();
            } else {
                let before = self.substring(0, index);
                let after = self.substring(index, self.len());

                *self = before;
                self.push(other);
                self.push(&after);
            }
            self.normalise();
        }
    }

    /// Creates a new string without style and insert it
    pub fn insert_plain(&mut self, index: usize, other: &str) {
        self.insert(index, &Self::from(other, Style::plain()));
    }

    /// Creates a new string with the style of the character before the split and insert it
    ///
    /// If inserted at index 0, the style of the first character will be used.
    pub fn insert_extend(&mut self, index: usize, other: &str) {
        self.insert(index, &Self::from(other, self.style_at(index)));
    }

    /// Split the string at `separator`
    pub fn split(&self, separator: &str) -> Vec<Self> {
        let sepsize = separator.len();
        let mut pieces = Vec::new();

        let mut last = 0;
        for i in 0..(self.len() - sepsize + 1) {
            if self.text.substring(i, i + sepsize) == separator {
                pieces.push(self.substring(last, i));
                last = i + sepsize;
            }
        }
        pieces.push(self.substring(last, self.len()));

        return pieces;
    }

    fn into_normalised(mut self) -> Self {
        self.normalise();
        self
    }

    fn normalise(&mut self) {
        // remove zero-sized spans
        self.style.retain(|span| span.start < span.end);
    }

    /// See [Self::complement()].
    pub fn into_complemented(mut self, style: Style) -> Self {
        self.complement(style);
        self.into_normalised()
    }

    /// Add a style to the entire string without overriding already present styles
    pub fn complement(&mut self, style: Style) {
        for span in self.style.iter_mut() {
            span.style = span.style & style;
        }
        self.normalise();
    }

    /// See [Self::overwrite()].
    pub fn into_overwritten(mut self, style: Style) -> Self {
        self.overwrite(style);
        self
    }

    /// Add a style to the entire string, overriding already present styles
    pub fn overwrite(&mut self, style: Style) {
        for span in self.style.iter_mut() {
            span.style = style & span.style;
        }
        self.normalise()
    }
}

impl Add<RichString> for &str {
    type Output = RichString;

    fn add(self, mut rhs: RichString) -> RichString {
        rhs.insert_plain(0, self);
        rhs
    }
}

impl Add<&str> for RichString {
    type Output = RichString;

    fn add(mut self, rhs: &str) -> RichString {
        self.push_plain(rhs);
        self
    }
}

impl Add<RichString> for RichString {
    type Output = RichString;

    fn add(mut self, rhs: RichString) -> RichString {
        self.push(&rhs);
        self
    }
}

impl Clone for RichString {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            style: self.style.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shortcuts::*;

    #[test]
    fn test_into() {
        let string: RichString = "colorful".into();

        assert_eq!(string, RichString::from("colorful", Style::plain()));
    }

    #[test]
    fn test_new() {
        assert_eq!(RichString::new(), RichString::from("", Style::plain()));
    }

    #[test]
    fn test_from() {
        let string = RichString::from("colorful", YELLOW);

        assert_eq!(string.raw(), "colorful");

        for i in 0..7 {
            assert_eq!(string.style_at(i), YELLOW);
        }
    }

    #[test]
    fn test_len() {
        let string = RichString::from("çōłöŕfûl", YELLOW);

        assert_eq!(string.len(), 8);
    }

    #[test]
    fn test_raw() {
        let string = RichString::from("colorful", Style::plain());

        assert_eq!(string.raw(), "colorful");
    }

    #[test]
    fn test_substring() {
        let string = RichString::from("col", YELLOW)
            + RichString::from("or", BOLD & RED)
            + RichString::from("ful", ITALIC & ON_PURPLE);

        let sub1 = RichString::from("ol", YELLOW);
        assert_eq!(string.substring(1, 3), sub1);

        let sub1 = RichString::from("l", YELLOW)
            + RichString::from("or", BOLD & RED)
            + RichString::from("f", ITALIC & ON_PURPLE);
        assert_eq!(string.substring(2, 6), sub1);
    }

    #[test]
    fn test_style_at() {
        let string = RichString::from("col", YELLOW)
            + RichString::from("or", BOLD & RED)
            + RichString::from("ful", ITALIC & ON_PURPLE);

        assert_eq!(string.style_at(0), YELLOW);
        assert_eq!(string.style_at(3), BOLD & RED);
        assert_eq!(string.style_at(5), ITALIC & ON_PURPLE);
        assert_eq!(string.style_at(8), Style::plain());
    }

    #[test]
    fn test_push() {
        let mut string = RichString::from("color", GREEN);
        string.push(&RichString::from("ful", BOLD));

        for i in 0..5 {
            assert_eq!(string.style_at(i), GREEN);
        }
        for i in 5..8 {
            assert_eq!(string.style_at(i), BOLD);
        }
    }

    #[test]
    fn test_push_plain() {
        let mut string = RichString::from("color", GREEN);
        string.push_plain("ful");

        for i in 0..5 {
            assert_eq!(string.style_at(i), GREEN);
        }
        for i in 5..8 {
            assert_eq!(string.style_at(i), Style::plain());
        }
    }

    #[test]
    fn test_push_extend() {
        let mut string = RichString::from("color", GREEN);
        string.push_extend("ful");

        for i in 0..8 {
            assert_eq!(string.style_at(i), GREEN);
        }
    }

    #[test]
    fn test_insert() {
        let mut string = RichString::from("coful", GREEN);
        string.insert(2, &RichString::from("lor", BOLD));

        assert_eq!(string.style_at(0), GREEN);
        assert_eq!(string.style_at(1), GREEN);
        assert_eq!(string.style_at(2), BOLD);
        assert_eq!(string.style_at(3), BOLD);
        assert_eq!(string.style_at(4), BOLD);
        assert_eq!(string.style_at(5), GREEN);
        assert_eq!(string.style_at(6), GREEN);
        assert_eq!(string.style_at(7), GREEN);
    }

    #[test]
    fn test_insert_plain() {
        let mut string = RichString::from("coful", GREEN);
        string.insert_plain(2, "lor");

        assert_eq!(string.style_at(0), GREEN);
        assert_eq!(string.style_at(1), GREEN);
        assert_eq!(string.style_at(2), Style::plain());
        assert_eq!(string.style_at(3), Style::plain());
        assert_eq!(string.style_at(4), Style::plain());
        assert_eq!(string.style_at(5), GREEN);
        assert_eq!(string.style_at(6), GREEN);
        assert_eq!(string.style_at(7), GREEN);
    }

    #[test]
    fn test_insert_extend() {
        let mut string1 = RichString::from("coful", GREEN);
        string1.insert_extend(2, "lor");

        for i in 0..8 {
            assert_eq!(string1.style_at(i), GREEN);
        }

        let mut string2 = RichString::from("orful", GREEN);
        string2.insert_extend(0, "col");

        for i in 0..8 {
            assert_eq!(string2.style_at(i), GREEN);
        }
    }

    #[test]
    fn test_split() {
        let string = RichString::from("colorful", CYAN);

        assert_eq!(
            string.split("o"),
            vec![
                RichString::from("c", CYAN),
                RichString::from("l", CYAN),
                RichString::from("rful", CYAN),
            ]
        );

        assert_eq!(
            string.split("or"),
            vec![RichString::from("col", CYAN), RichString::from("ful", CYAN),]
        );

        assert_eq!(
            string.split("col"),
            vec![RichString::from("", CYAN), RichString::from("orful", CYAN),]
        );

        assert_eq!(
            string.split("ful"),
            vec![RichString::from("color", CYAN), RichString::from("", CYAN),]
        );
    }

    #[test]
    fn test_complement() {
        let string = "col" + RichString::from("or", PURPLE & BOLD) + "ful";
        let expected = RichString::from("col", YELLOW)
            + RichString::from("or", PURPLE & BOLD)
            + RichString::from("ful", YELLOW);

        assert_eq!(string.into_complemented(YELLOW), expected);
    }

    #[test]
    fn test_overwrite() {
        let string = "col" + RichString::from("or", PURPLE & BOLD) + "ful";
        let expected = RichString::from("col", YELLOW)
            + RichString::from("or", YELLOW & BOLD)
            + RichString::from("ful", YELLOW);

        assert_eq!(string.into_overwritten(YELLOW), expected);
    }
}
