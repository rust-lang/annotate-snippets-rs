use super::Stylesheet;
use crate::AnnotationType;
use std::fmt;
use std::fmt::Display;

use ansi_term::Colour::{Red, Yellow};

#[derive(Default)]
pub struct StylesheetColor {}

impl Stylesheet for StylesheetColor {
    fn format(
        &self,
        f: &mut fmt::Formatter,
        annotation_type: &AnnotationType,
        value: impl Display,
    ) -> fmt::Result {
        match annotation_type {
            AnnotationType::Error => write!(f, "{}", Red.paint(value.to_string()).to_string()),
            AnnotationType::Warning => write!(f, "{}", Yellow.paint(value.to_string()).to_string()),
            _ => write!(f, "{}", value),
        }
    }
}
