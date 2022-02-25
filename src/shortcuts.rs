use crate::{Color, Decoration, Feature, Style};

macro_rules! foreground {
    ($doc: literal $color: ident) => {
        #[doc = $doc]
        pub const $color: Style = Style {
            foreground: Color::$color,
            background: Color::INHERIT,
            bold: Feature::INHERIT,
            italic: Feature::INHERIT,
            decoration: Decoration::INHERIT,
        };
    };
}

foreground! { "Black text" BLACK }
foreground! { "Red text" RED }
foreground! { "Yellow text" YELLOW }
foreground! { "Green text" GREEN }
foreground! { "Cyan text" CYAN }
foreground! { "Blue text" BLUE }
foreground! { "Purple text" PURPLE }
foreground! { "Light grey text, the default text color of the terminal" LIGHTGREY }

macro_rules! background {
    ($doc: literal $name: ident $color: ident) => {
        #[doc = $doc]
        pub const $name: Style = Style {
            foreground: Color::INHERIT,
            background: Color::$color,
            bold: Feature::INHERIT,
            italic: Feature::INHERIT,
            decoration: Decoration::INHERIT,
        };
    };
}

background! { "Black background, the default background color of the terminal" ON_BLACK BLACK }
background! { "Red background" ON_RED RED }
background! { "Yellow background" ON_YELLOW YELLOW }
background! { "Green backgound" ON_GREEN GREEN }
background! { "Cyan background" ON_CYAN CYAN }
background! { "Blue background" ON_BLUE BLUE }
background! { "Purple background" ON_PURPLE PURPLE }
background! { "Light grey backgound" ON_LIGHTGREY LIGHTGREY }

/// Enables bold
pub const BOLD: Style = Style {
    foreground: Color::INHERIT,
    background: Color::INHERIT,
    bold: Feature::ENABLED,
    italic: Feature::INHERIT,
    decoration: Decoration::INHERIT,
};

/// Enables italic
pub const ITALIC: Style = Style {
    foreground: Color::INHERIT,
    background: Color::INHERIT,
    bold: Feature::INHERIT,
    italic: Feature::ENABLED,
    decoration: Decoration::INHERIT,
};

/// Undeline decoration
pub const UNDERLINE: Style = Style {
    foreground: Color::INHERIT,
    background: Color::INHERIT,
    bold: Feature::INHERIT,
    italic: Feature::INHERIT,
    decoration: Decoration::UNDERLINE,
};
