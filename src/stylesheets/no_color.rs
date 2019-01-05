use crate::formatter::style::{Style, StyleClass, Stylesheet};

pub struct NoOpStyle {}

impl Style for NoOpStyle {
    fn paint(&self, text: &str) -> String {
        text.to_string()
    }

    fn bold(&self) -> Box<Style> {
        Box::new(NoOpStyle {})
    }
}

pub struct NoColorStylesheet {}

impl Stylesheet for NoColorStylesheet {
    fn get_style(&self, _class: StyleClass) -> Box<Style> {
        Box::new(NoOpStyle {})
    }
}
