use super::Stylesheet;
use crate::AnnotationType;
use std::fmt;
use std::fmt::Display;

#[derive(Default)]
pub struct StylesheetPlain {}

impl Stylesheet for StylesheetPlain {
    fn format(
        &self,
        f: &mut fmt::Formatter,
        _annotation_type: &AnnotationType,
        value: impl Display,
    ) -> fmt::Result {
        write!(f, "{}", value)
    }
}
