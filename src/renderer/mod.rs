mod margin;
pub(crate) mod stylesheet;

use crate::display_list::DisplayList;
use crate::snippet::Snippet;
pub use anstyle::*;
pub use margin::Margin;
use std::fmt::Display;
use stylesheet::Stylesheet;

#[derive(Clone)]
pub struct Renderer {
    anonymized_line_numbers: bool,
    margin: Option<Margin>,
    stylesheet: Stylesheet,
}

impl Renderer {
    /// No terminal styling
    pub const fn plain() -> Self {
        Self {
            anonymized_line_numbers: false,
            margin: None,
            stylesheet: Stylesheet::plain(),
        }
    }

    /// Default terminal styling
    pub const fn styled() -> Self {
        Self {
            stylesheet: Stylesheet {
                error: AnsiColor::BrightRed.on_default().effects(Effects::BOLD),
                warning: AnsiColor::BrightYellow.on_default().effects(Effects::BOLD),
                info: AnsiColor::BrightBlue.on_default().effects(Effects::BOLD),
                note: Style::new().effects(Effects::BOLD),
                help: AnsiColor::BrightCyan.on_default().effects(Effects::BOLD),
                line_no: AnsiColor::BrightBlue.on_default().effects(Effects::BOLD),
                emphasis: Style::new().effects(Effects::BOLD),
                none: Style::new(),
            },
            ..Self::plain()
        }
    }

    pub const fn anonymized_line_numbers(mut self, anonymized_line_numbers: bool) -> Self {
        self.anonymized_line_numbers = anonymized_line_numbers;
        self
    }

    pub const fn margin(mut self, margin: Option<Margin>) -> Self {
        self.margin = margin;
        self
    }

    pub const fn error(mut self, style: Style) -> Self {
        self.stylesheet.error = style;
        self
    }

    pub const fn warning(mut self, style: Style) -> Self {
        self.stylesheet.warning = style;
        self
    }

    pub const fn info(mut self, style: Style) -> Self {
        self.stylesheet.info = style;
        self
    }

    pub const fn note(mut self, style: Style) -> Self {
        self.stylesheet.note = style;
        self
    }

    pub const fn help(mut self, style: Style) -> Self {
        self.stylesheet.help = style;
        self
    }

    pub const fn line_no(mut self, style: Style) -> Self {
        self.stylesheet.line_no = style;
        self
    }

    pub const fn emphasis(mut self, style: Style) -> Self {
        self.stylesheet.emphasis = style;
        self
    }

    pub const fn none(mut self, style: Style) -> Self {
        self.stylesheet.none = style;
        self
    }

    pub fn render<'a>(&'a self, snippet: Snippet<'a>) -> impl Display + 'a {
        DisplayList::new(
            snippet,
            &self.stylesheet,
            self.anonymized_line_numbers,
            self.margin,
        )
    }
}
