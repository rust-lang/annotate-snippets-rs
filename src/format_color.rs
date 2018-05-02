extern crate ansi_term;

use self::ansi_term::Color::Fixed;
use self::ansi_term::Style;
use display_list::{DisplayAnnotationType, DisplayLine, DisplayList, DisplayMark,
                   DisplayAnnotationPart, DisplayHeaderType};
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

    fn format_annotation_content(
        range: &(usize, usize),
        label: &Option<String>,
        annotation_type: &DisplayAnnotationType,
        annotation_part: &DisplayAnnotationPart,
    ) -> String {
        let label = label.clone().map_or("".to_string(), |l| format!(" {}", l));
        let prefix = match annotation_part {
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
        format!("{}{}{}",
          prefix.repeat(range.0),
          color.paint(mark.repeat(range.1 - range.0)),
          color.paint(label),
        )
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
                writeln!(
                    f,
                    "{}{}",
                    color.bold().paint(name),
                    Style::new().bold().paint(format!(": {}", label))
                )
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
                    writeln!(f, "{}{} {}", " ".repeat(lineno_width), Fixed(12).bold().paint(header_sigil), path,)
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
            } => {
                let prefix = format!("{} |", " ".repeat(lineno_width));
                writeln!(
                    f,
                    "{}{}{}",
                    Fixed(12).bold().paint(prefix),
                    Self::format_inline_marks(&inline_marks, inline_marks_width),
                    Self::format_annotation_content(range, &label, &annotation_type, &annotation_part),
                )
            }
            DisplayLine::Fold { inline_marks } => writeln!(
                f,
                "... {}",
                Self::format_inline_marks(&inline_marks, inline_marks_width),
            ),
            DisplayLine::AlignedAnnotation {
                label,
                annotation_type,
            } => {
                let color = match annotation_type {
                    DisplayAnnotationType::Error => Fixed(9).bold(),
                    DisplayAnnotationType::Warning => Fixed(11).bold(),
                    DisplayAnnotationType::Note => Style::new().bold(),
                    DisplayAnnotationType::Help => Fixed(14).bold(),
                };
                let prefix = format!("{} =", " ".repeat(lineno_width));
                writeln!(
                    f,
                    "{} {}: {}",
                    Fixed(12).bold().paint(prefix),
                    color.paint(Self::format_annotation_type(annotation_type)),
                    label
                )
            }
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
