use ansi_term::Color::Fixed;
use ansi_term::Style as AnsiTermStyle;

use super::Style as StyleTrait;
use super::StyleType;

use std::fmt;

pub struct Style {}

impl StyleTrait for Style {
    fn fmt(
        w: &mut dyn std::io::Write,
        pattern: impl fmt::Display,
        styles: &[StyleType],
    ) -> std::io::Result<()> {
        let mut style = AnsiTermStyle::new();
        for style_type in styles {
            match style_type {
                StyleType::Bold => {
                    style = style.bold();
                }
                StyleType::Error => style = style.fg(Fixed(9)),
                StyleType::Warning => style = style.fg(Fixed(11)),
                StyleType::Info => style = style.fg(Fixed(12)),
                StyleType::Note => {}
                StyleType::Help => style = style.fg(Fixed(14)),
                StyleType::LineNo => style = style.fg(Fixed(12)),
                StyleType::None => {}
            }
        }
        write!(w, "{}", style.paint(pattern.to_string()))
    }
}

//-impl Stylesheet for AnsiTermStylesheet {
//-    fn get_style(&self, class: StyleClass) -> Box<dyn Style> {
//-        let ansi_term_style = match class {
//-            StyleClass::Error => Fixed(9).bold(),
//-            StyleClass::Warning => Fixed(11).bold(),
//-            StyleClass::Info => Fixed(12).bold(),
//-            StyleClass::Note => AnsiTermStyle::new().bold(),
//-            StyleClass::Help => Fixed(14).bold(),
//-
//-            StyleClass::LineNo => Fixed(12).bold(),
//-
//-
//-            StyleClass::Emphasis => AnsiTermStyle::new().bold(),
//-
//-            StyleClass::None => AnsiTermStyle::new(),
//-        };
//-        Box::new(AnsiTermStyleWrapper {
//-            style: ansi_term_style,
//-        })
//-    }
//-}
