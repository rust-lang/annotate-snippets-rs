use super::{StyleClass, Stylesheet};
use crate::AnnotationType;
use std::fmt;
use std::fmt::Display;

use ansi_term::Colour::{Red, Yellow};
use ansi_term::Style;

#[derive(Default)]
pub struct StylesheetColor {}

impl Stylesheet for StylesheetColor {
    fn format(
        &self,
        f: &mut fmt::Formatter,
        styles: &[StyleClass],
        value: impl Display,
    ) -> fmt::Result {
        let mut style = Style::new();
        for s in styles {
            match s {
                StyleClass::TitleLineAnnotationType => style = style.bold(),
                StyleClass::TitleLine => style = style.bold(),
                StyleClass::AnnotationTypeError => style = style.fg(Red),
            }
        }
        write!(f, "{}", style.paint(value.to_string()))
    }
}
