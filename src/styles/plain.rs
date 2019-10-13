use super::{StyleClass, Stylesheet};
use std::fmt;
use std::fmt::Display;

#[derive(Default)]
pub struct StylesheetPlain {}

impl Stylesheet for StylesheetPlain {
    fn format(
        &self,
        f: &mut fmt::Formatter,
        pattern: impl Display,
        _style: &[StyleClass],
    ) -> fmt::Result {
        write!(f, "{}", pattern)
    }
}
