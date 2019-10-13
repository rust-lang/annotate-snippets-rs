use ansi_term::Style as AnsiTermStyle;

use super::Style as StyleTrait;

use std::fmt;

pub struct Style {
}

impl StyleTrait for Style {
    fn fmt(
        w: &mut dyn std::io::Write,
        pattern: impl fmt::Display,
        ) -> std::io::Result<()> {
        let style = AnsiTermStyle::new().bold();
        write!(w, "{}", style.paint(pattern.to_string()))
    }
}
