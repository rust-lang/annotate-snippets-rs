use std::fmt;

use crate::formatter::style::{Style, StyleClass, Stylesheet};

pub struct NoOpStyle {}

impl Style for NoOpStyle {
    fn paint(&self, text: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(text)
    }

    fn bold(&self) -> Box<dyn Style> {
        Box::new(NoOpStyle {})
    }
}

pub struct NoColorStylesheet;

impl Stylesheet for NoColorStylesheet {
    fn get_style(&self, _class: StyleClass) -> Box<dyn Style> {
        Box::new(NoOpStyle {})
    }
}
