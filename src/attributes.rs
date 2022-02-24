#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    INHERIT,
    RED,
    YELLOW,
    GREEN,
    CYAN,
    BLUE,
    PURPLE,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Decoration {
    INHERIT,
    NONE,
    UNDERLINE,
    LINETHROUGH,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Feature {
    INHERIT,
    ENABLED,
    DISABLED,
}
