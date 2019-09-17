use super::{StyleClass, Stylesheet};
use std::fmt;
use std::fmt::Display;

#[derive(Default)]
pub struct StylesheetPlain {}

impl Stylesheet for StylesheetPlain {
    fn format(
        &self,
        f: &mut fmt::Formatter,
        _style: &[StyleClass],
        value: impl Display,
    ) -> fmt::Result {
        write!(f, "{}", value)
    }
}
