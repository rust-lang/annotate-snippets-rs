use crate::formatter::style::{Style, StyleClass, Stylesheet};

use ansi_term::Color::Fixed;
use ansi_term::Style as AnsiTermStyle;

struct AnsiTermStyleWrapper {
    style: AnsiTermStyle,
}

impl Style for AnsiTermStyleWrapper {
    fn paint(&self, text: &str) -> String {
        format!("{}", self.style.paint(text))
    }

    fn bold(&self) -> Box<dyn Style> {
        Box::new(AnsiTermStyleWrapper {
            style: self.style.clone(),
        })
    }
}

pub struct AnsiTermStylesheet {}

impl Stylesheet for AnsiTermStylesheet {
    fn get_style(&self, class: StyleClass) -> Box<dyn Style> {
        let ansi_term_style = match class {
            StyleClass::Error => Fixed(9).bold(),
            StyleClass::Warning => Fixed(11).bold(),
            StyleClass::Info => Fixed(12).bold(),
            StyleClass::Note => AnsiTermStyle::new().bold(),
            StyleClass::Help => Fixed(14).bold(),

            StyleClass::LineNo => Fixed(12).bold(),

            StyleClass::Emphasis => AnsiTermStyle::new().bold(),

            StyleClass::None => AnsiTermStyle::new(),
        };
        Box::new(AnsiTermStyleWrapper {
            style: ansi_term_style,
        })
    }
}
