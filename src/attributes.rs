/// Colors
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    /// Don't change the color
    INHERIT,
    /// Black, default background color of the terminal
    BLACK,
    /// Red
    RED,
    /// Yellow
    YELLOW,
    /// Green
    GREEN,
    /// Cyan
    CYAN,
    /// Blue
    BLUE,
    /// Purple
    PURPLE,
    /// Light grey, default text color of the terminal
    LIGHTGREY,
    /// Dark grey
    DARKGREY,
    /// Light red
    LIGHTRED,
    /// Light yellow
    LIGHTYELLOW,
    /// Light green
    LIGHTGREEN,
    /// Light cyan
    LIGHTCYAN,
    /// Light blue
    LIGHTBLUE,
    /// Light purple
    LIGHTPURPLE,
    /// White, *not the default text color of the terminal, use `LIGHTGREY` for that purpose*
    WHITE,
}

/// Underline or line-through
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Decoration {
    /// Don't change the decoration
    INHERIT,
    /// No decoration
    NONE,
    /// Underline
    UNDERLINE,
    /// Line-through
    LINETHROUGH,
}

/// Boolean value with 3 states
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Feature {
    /// Don't change the value
    INHERIT,
    /// Enabled
    ENABLED,
    /// Disabled
    DISABLED,
}
