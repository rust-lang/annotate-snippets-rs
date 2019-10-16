use super::Style as StyleTrait;
use super::StyleType;

use std::fmt;

pub struct Style {}

impl StyleTrait for Style {
    fn fmt(
        w: &mut dyn std::io::Write,
        pattern: impl fmt::Display,
        _styles: &[StyleType],
    ) -> std::io::Result<()> {
        write!(w, "{}", pattern)
    }
}
