extern crate ansi_term;

use self::ansi_term::Color::Fixed;
use self::ansi_term::Style;
use display_list::{DisplayAnnotationPart, DisplayAnnotationType, DisplayHeaderType, DisplayLine,
                   DisplayList, DisplayMark, DisplayTextFragment, DisplayTextStyle};
use display_list_formatting::DisplayListFormatting;
use std::fmt;

struct Formatter {}

impl DisplayListFormatting for Formatter {
    fn format_annotation_type(annotation_type: &DisplayAnnotationType) -> String {
        match annotation_type {
            DisplayAnnotationType::Error => "error".to_string(),
            DisplayAnnotationType::Warning => "warning".to_string(),
            DisplayAnnotationType::Note => "note".to_string(),
            DisplayAnnotationType::Help => "help".to_string(),
        }
    }

    fn format_inline_marks(inline_marks: &[DisplayMark], inline_marks_width: usize) -> String {
        format!(
            "{:>width$}",
            inline_marks
                .iter()
                .map(|mark| match mark {
                    DisplayMark::AnnotationThrough => "|",
                    DisplayMark::AnnotationStart => "/",
                })
                .collect::<Vec<&str>>()
                .join(""),
            width = inline_marks_width
        )
    }

    fn format_source_annotation_lines(
        f: &mut fmt::Formatter,
        lineno_width: usize,
        inline_marks: String,
        range: &(usize, usize),
        label: &[DisplayTextFragment],
        annotation_type: &DisplayAnnotationType,
        annotation_part: &DisplayAnnotationPart,
    ) -> fmt::Result {
        let indent_char = match annotation_part {
            DisplayAnnotationPart::Singleline => " ",
            DisplayAnnotationPart::MultilineStart => "_",
            DisplayAnnotationPart::MultilineEnd => "_",
        };
        let mark = match annotation_type {
            DisplayAnnotationType::Error => "^",
            DisplayAnnotationType::Warning => "-",
            DisplayAnnotationType::Note => "-",
            DisplayAnnotationType::Help => "-",
        };
        let color = match annotation_type {
            DisplayAnnotationType::Error => Fixed(9).bold(),
            DisplayAnnotationType::Warning => Fixed(11).bold(),
            DisplayAnnotationType::Note => Style::new().bold(),
            DisplayAnnotationType::Help => Fixed(14).bold(),
        };
        if let Some((first, rest)) = Self::format_label(label)
            .lines()
            .collect::<Vec<&str>>()
            .split_first()
        {
            let indent = range.1;
            writeln!(
                f,
                "{}{}{}{} {}",
                Fixed(12)
                    .bold()
                    .paint(format!("{} |", " ".repeat(lineno_width))),
                inline_marks,
                indent_char.repeat(range.0),
                color.paint(mark.repeat(range.1 - range.0)),
                color.paint(*first),
            )?;
            for line in rest {
                writeln!(
                    f,
                    "{}{}{} {}",
                    Fixed(12)
                        .bold()
                        .paint(format!("{} |", " ".repeat(lineno_width))),
                    inline_marks,
                    " ".repeat(indent),
                    color.paint(*line),
                )?;
            }
        } else {
            writeln!(
                f,
                "{}{}{}{}",
                Fixed(12)
                    .bold()
                    .paint(format!("{} |", " ".repeat(lineno_width))),
                inline_marks,
                indent_char.repeat(range.0),
                color.paint(mark.repeat(range.1 - range.0)),
            )?;
        }
        Ok(())
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
