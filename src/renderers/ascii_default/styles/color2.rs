use termcolor::{Ansi, Color, ColorSpec, WriteColor};

use super::Style as StyleTrait;
use super::StyleType;

use std::fmt;
use std::io::Write;

pub struct Style {}

impl StyleTrait for Style {
    fn fmt(
        w: &mut dyn std::io::Write,
        pattern: impl fmt::Display,
        styles: &[StyleType],
    ) -> std::io::Result<()> {
        let mut color = ColorSpec::new();
        for style_type in styles {
            match style_type {
                StyleType::Emphasis => {
                    color.set_bold(true);
                }
                StyleType::Error => {
                    color.set_fg(Some(Color::Red));
                }
                StyleType::Warning => {
                    color.set_fg(Some(Color::Yellow));
                }
                StyleType::LineNo => {
                    color.set_fg(Some(Color::Ansi256(12)));
                }
                _ => {}
            }
        }
        let mut ansi = Ansi::new(w);
        ansi.set_color(&color).unwrap();
        //ansi.set_color(ColorSpec::new().set_bold(true)).unwrap();
        write!(ansi, "{}", pattern)?;
        ansi.reset().unwrap();
        Ok(())
    }
}
