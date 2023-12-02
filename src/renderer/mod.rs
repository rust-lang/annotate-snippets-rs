pub mod stylesheet;

use crate::display_list::{DisplayList, Margin};
use crate::snippet::Snippet;
use std::fmt::Display;
use stylesheet::Stylesheet;
use yansi_term::Color::Fixed;
use yansi_term::Style;

#[derive(Clone)]
pub struct Renderer {
    anonymized_line_numbers: bool,
    margin: Option<Margin>,
    stylesheet: Stylesheet,
}

impl Renderer {
    /// No terminal styling
    pub fn plain() -> Self {
        Self {
            anonymized_line_numbers: false,
            margin: None,
            stylesheet: Stylesheet::default(),
        }
    }

    /// Default terminal styling
    pub fn styled() -> Self {
        Self {
            stylesheet: Stylesheet {
                error: Fixed(9).bold(),
                warning: Fixed(11).bold(),
                info: Fixed(12).bold(),
                note: Style::new().bold(),
                help: Fixed(14).bold(),
                line_no: Fixed(12).bold(),
                emphasis: Style::new().bold(),
                none: Style::new(),
            },
            ..Self::plain()
        }
    }

    pub fn anonymized_line_numbers(mut self, anonymized_line_numbers: bool) -> Self {
        self.anonymized_line_numbers = anonymized_line_numbers;
        self
    }

    pub fn margin(mut self, margin: Option<Margin>) -> Self {
        self.margin = margin;
        self
    }

    pub fn error(mut self, style: Style) -> Self {
        self.stylesheet.error = style;
        self
    }

    pub fn warning(mut self, style: Style) -> Self {
        self.stylesheet.warning = style;
        self
    }

    pub fn info(mut self, style: Style) -> Self {
        self.stylesheet.info = style;
        self
    }

    pub fn note(mut self, style: Style) -> Self {
        self.stylesheet.note = style;
        self
    }

    pub fn help(mut self, style: Style) -> Self {
        self.stylesheet.help = style;
        self
    }

    pub fn line_no(mut self, style: Style) -> Self {
        self.stylesheet.line_no = style;
        self
    }

    pub fn emphasis(mut self, style: Style) -> Self {
        self.stylesheet.emphasis = style;
        self
    }

    pub fn none(mut self, style: Style) -> Self {
        self.stylesheet.none = style;
        self
    }

    pub fn render<'a>(&'a self, snippet: Snippet<'a>) -> impl Display + 'a {
        DisplayList::new(
            snippet,
            self.stylesheet,
            self.anonymized_line_numbers,
            self.margin,
        )
    }
}
