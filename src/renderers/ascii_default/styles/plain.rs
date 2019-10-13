use super::Style as StyleTrait;

use std::fmt;

pub struct Style {}

impl StyleTrait for Style {
    fn fmt(w: &mut dyn std::io::Write, pattern: impl fmt::Display) -> std::io::Result<()> {
        write!(w, "{}", pattern)
    }
}
