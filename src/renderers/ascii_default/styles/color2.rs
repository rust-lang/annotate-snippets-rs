use termcolor::{Ansi, ColorSpec, WriteColor};

use super::Style as StyleTrait;

use std::fmt;
use std::io::Write;

pub struct Style {}

impl StyleTrait for Style {
    fn fmt(w: &mut dyn std::io::Write, pattern: impl fmt::Display) -> std::io::Result<()> {
        let mut ansi = Ansi::new(w);
        ansi.set_color(ColorSpec::new().set_bold(true)).unwrap();
        write!(ansi, "{}", pattern)?;
        ansi.reset().unwrap();
        Ok(())
    }
}
