use super::{StyleClass, Stylesheet};
use std::fmt;
use std::fmt::Display;

use ansi_term::Colour::Red;
use ansi_term::Style;

#[derive(Default)]
pub struct StylesheetColor {}

impl Stylesheet for StylesheetColor {
    fn format(
        &self,
        f: &mut fmt::Formatter,
        pattern: impl Display,
        styles: &[StyleClass],
    ) -> fmt::Result {
        let mut style = Style::new();
        for s in styles {
            match s {
                StyleClass::TitleLineAnnotationType => style = style.bold(),
                StyleClass::TitleLine => style = style.bold(),
                StyleClass::AnnotationTypeError => style = style.fg(Red),
            }
        }
        write!(f, "{}", style.paint(pattern.to_string()))
    }
}
