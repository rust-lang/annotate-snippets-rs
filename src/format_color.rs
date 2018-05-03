extern crate ansi_term;

use self::ansi_term::Color::Fixed;
use self::ansi_term::Style;
use display_list::{DisplayAnnotationPart, DisplayAnnotationType, DisplayHeaderType, DisplayLine,
                   DisplayList, DisplayMark, DisplayTextFragment, DisplayTextStyle};
use display_list_formatting::DisplayListFormatting;
use std::fmt;

struct Formatter {}

fn repeat_char(c: char, n: usize) -> String {
    let mut s = String::with_capacity(c.len_utf8());
    s.push(c);
    return s.repeat(n);
}

impl DisplayListFormatting for Formatter {
    fn format_inline_mark(inline_mark: &DisplayMark) -> String {
        let sigil = Self::get_inline_mark(inline_mark);
        let color = match inline_mark.annotation_type {
            DisplayAnnotationType::Error => Fixed(9).bold(),
            DisplayAnnotationType::Warning => Fixed(11).bold(),
            DisplayAnnotationType::Info => Fixed(12).bold(),
            DisplayAnnotationType::Note => Style::new().bold(),
            DisplayAnnotationType::Help => Fixed(14).bold(),
        };
        format!("{}", color.paint(sigil))
    }

    fn format_source_annotation_parts(
        annotation_type: &DisplayAnnotationType,
        indent_char: char,
        mark: char,
        range: &(usize, usize),
        lineno_width: usize,
    ) -> (String, String, String) {
        let color = match annotation_type {
            DisplayAnnotationType::Error => Fixed(9).bold(),
            DisplayAnnotationType::Warning => Fixed(11).bold(),
            DisplayAnnotationType::Info => Fixed(12).bold(),
            DisplayAnnotationType::Note => Style::new().bold(),
            DisplayAnnotationType::Help => Fixed(14).bold(),
        };
        let lineno = format!(
            "{}",
            Fixed(12)
                .bold()
                .paint(format!("{} |", " ".repeat(lineno_width)))
        );
        let indent = if indent_char == ' ' {
            repeat_char(indent_char, range.0)
        } else {
            format!("{}", color.paint(repeat_char(indent_char, range.0)))
        };
        let pointer = format!("{}", color.paint(repeat_char(mark, range.1 - range.0)));
        return (lineno, indent, pointer);
    }

    fn format_label_line(annotation_type: &DisplayAnnotationType, line: &str) -> String {
        let color = match annotation_type {
            DisplayAnnotationType::Error => Fixed(9).bold(),
            DisplayAnnotationType::Warning => Fixed(11).bold(),
            DisplayAnnotationType::Info => Fixed(12).bold(),
            DisplayAnnotationType::Note => Style::new().bold(),
            DisplayAnnotationType::Help => Fixed(14).bold(),
        };
        return format!("{}", color.paint(line));
    }

    fn format_label(label: &[DisplayTextFragment]) -> String {
        label
            .iter()
            .map(|fragment| match fragment.style {
                DisplayTextStyle::Regular => fragment.content.clone(),
                DisplayTextStyle::Emphasis => {
                    format!("{}", Style::new().bold().paint(fragment.content.clone()))
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn format_line(
        f: &mut fmt::Formatter,
        dl: &DisplayLine,
        lineno_width: usize,
        inline_marks_width: usize,
    ) -> fmt::Result {
        match dl {
            DisplayLine::Annotation {
                annotation_type,
                id,
                aligned,
                label,
            } => {
                let color = match annotation_type {
                    DisplayAnnotationType::Error => Fixed(9).bold(),
                    DisplayAnnotationType::Warning => Fixed(11).bold(),
                    DisplayAnnotationType::Info => Fixed(12).bold(),
                    DisplayAnnotationType::Note => Style::new().bold(),
                    DisplayAnnotationType::Help => Fixed(14).bold(),
                };
                let name = if let Some(id) = id {
                    format!("{}[{}]", Self::format_annotation_type(&annotation_type), id)
                } else {
                    Self::format_annotation_type(&annotation_type)
                };
                let prefix = if *aligned {
                    format!("{} = ", " ".repeat(lineno_width))
                } else {
                    "".to_string()
                };
                if let Some((first, rest)) = Self::format_label(label)
                    .lines()
                    .collect::<Vec<&str>>()
                    .split_first()
                {
                    let indent = prefix.len() + name.len() + 2;
                    writeln!(
                        f,
                        "{}{}{}",
                        Fixed(12).bold().paint(prefix),
                        color.bold().paint(name),
                        format!(": {}", first)
                    )?;
                    for line in rest {
                        writeln!(f, "{}{}", " ".repeat(indent), format!("{}", line))?;
                    }
                }
                Ok(())
            }
            DisplayLine::Origin {
                path,
                pos,
                header_type,
            } => {
                let header_sigil = match header_type {
                    DisplayHeaderType::Initial => "-->",
                    DisplayHeaderType::Continuation => ":::",
                };
                if let Some((row, col)) = pos {
                    writeln!(
                        f,
                        "{}{} {}:{}:{}",
                        " ".repeat(lineno_width),
                        Fixed(12).bold().paint(header_sigil),
                        path,
                        row,
                        col
                    )
                } else {
                    writeln!(
                        f,
                        "{}{} {}",
                        " ".repeat(lineno_width),
                        Fixed(12).bold().paint(header_sigil),
                        path,
                    )
                }
            }
            DisplayLine::EmptySource => {
                let prefix = format!("{} |", " ".repeat(lineno_width));
                writeln!(f, "{}", Fixed(12).bold().paint(prefix))
            }
            DisplayLine::Source {
                lineno,
                inline_marks,
                content,
                ..
            } => {
                let prefix = format!("{:>width$} |", lineno, width = lineno_width);
                writeln!(
                    f,
                    "{}{} {}",
                    Fixed(12).bold().paint(prefix),
                    Self::format_inline_marks(&inline_marks, inline_marks_width),
                    content,
                )
            }
            DisplayLine::SourceAnnotation {
                inline_marks,
                range,
                label,
                annotation_type,
                annotation_part,
            } => Self::format_source_annotation_lines(
                f,
                lineno_width,
                Self::format_inline_marks(&inline_marks, inline_marks_width),
                range,
                &label,
                &annotation_type,
                &annotation_part,
            ),
            DisplayLine::Fold { inline_marks } => writeln!(
                f,
                "... {}",
                Self::format_inline_marks(&inline_marks, inline_marks_width),
            ),
        }
    }
}

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

        for line in &self.body {
            Formatter::format_line(f, line, lineno_width, inline_marks_width)?;
        }
        Ok(())
    }
}
