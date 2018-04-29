use std::fmt;
use structs::display_list::{DisplayLine, DisplayList};
use display_list_formatting::DisplayListFormatting;

struct Formatter {}

impl DisplayListFormatting for Formatter {}

impl fmt::Display for DisplayList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lineno_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { lineno, .. } => {
                let width = lineno.to_string().len();
                if width > max {
                    width
                } else {
                    max
                }
            }
            _ => max,
        });
        let inline_marks_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => {
                let width = inline_marks.len();
                if width > max {
                    width + 1
                } else {
                    max
                }
            }
            _ => max,
        });
        let body = self.body
            .clone()
            .into_iter()
            .map(|line| Formatter::format_line(line, lineno_width, inline_marks_width))
            .collect::<Vec<String>>();
        write!(f, "{}", body.join("\n"))
    }
}
