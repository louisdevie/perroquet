use crate::style::{Style, StyleSpan};
use crate::substring::Substring;
use std::ops::Add;

#[derive(Debug, PartialEq)]
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
    pub fn new() -> Self {
        Self {
            text: String::new(),
            style: vec![],
        }
    }

    pub fn from(text: &str, style: Style) -> Self {
        if text.len() == 0 {
            Self::new()
        } else {
            Self {
                text: String::from(text),
                style: vec![StyleSpan {
                    style,
                    start: 0,
                    end: text.len(),
                }],
            }
        }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn raw(&self) -> &str {
        &self.text
    }

    pub fn substring(&self, start: usize, end: usize) -> Self {
        let mut style = Vec::new();

        for span in &self.style {
            if start >= span.start {
                if end <= span.end {
                    style.push(StyleSpan {
                        style: span.style,
                        start,
                        end,
                    });
                    break;
                } else {
                    style.push(StyleSpan {
                        style: span.style,
                        start,
                        end: span.end,
                    });
                }
            } else {
                if end <= span.end {
                    style.push(StyleSpan {
                        style: span.style,
                        start: span.start,
                        end,
                    });
                    break;
                } else {
                    style.push(StyleSpan {
                        style: span.style,
                        start: span.start,
                        end: span.end,
                    });
                }
            }
        }

        Self {
            text: String::from(self.text.substring(start, end)),
            style,
        }
    }

    pub fn style_at(&self, index: usize) -> Style {
        for span in &self.style {
            if index < span.end {
                return span.style;
            }
        }
        return Style::plain();
    }

    pub fn push(&mut self, other: &Self) {
        if other.style.len() > 0 {
            if self.style.len() == 0 {
                *self = other.clone();
            } else {
                let last = self.style.len() - 1;
                let mut skip_first = false;

                if self.style[last].style == other.style[0].style {
                    self.style[last].end += other.style[0].end;
                    skip_first = true;
                }

                for (i, span) in other.style.iter().enumerate() {
                    if i == 0 && skip_first {
                        continue;
                    }
                    self.style.push(StyleSpan {
                        style: span.style,
                        start: span.start + self.style[last].end,
                        end: span.end + self.style[last].end,
                    })
                }

                self.text.push_str(&other.text);
            }
        }
    }

    pub fn push_plain(&mut self, other: &str) {
        self.push(&Self::from(other, Style::plain()));
    }

    pub fn push_extend(&mut self, other: &str) {
        if self.style.len() == 0 {
            self.push(&Self::from(other, Style::plain()));
        } else {
            self.push(&Self::from(other, self.style[self.style.len() - 1].style));
        }
    }

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
        }
    }

    pub fn insert_plain(&mut self, index: usize, other: &str) {
        self.insert(index, &Self::from(other, Style::plain()));
    }

    pub fn insert_extend(&mut self, index: usize, other: &str) {
        self.insert(index, &Self::from(other, self.style_at(index)));
    }

    pub fn split(&self, separator: &str) -> Vec<Self> {
        let sepsize = separator.len();
        let mut pieces = Vec::new();

        let mut last = 0;
        for i in 0..(self.len() - sepsize) {
            if self.text.substring(i, i + sepsize) == separator {
                pieces.push(self.substring(last, i));
                last = i + sepsize;
            }
        }
        pieces.push(self.substring(last, self.len()));

        return pieces;
    }
}

impl Add<RichString> for &str {
    type Output = RichString;
    fn add(self, _rhs: RichString) -> RichString {
        todo!()
    }
}

impl Add<&str> for RichString {
    type Output = RichString;
    fn add(self, _rhs: &str) -> RichString {
        todo!()
    }
}

impl Clone for RichString {
    fn clone(&self) -> Self {
        todo!()
    }
}
