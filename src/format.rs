use display_list::{DisplayAnnotationPart, DisplayAnnotationType, DisplayHeaderType, DisplayLine,
                   DisplayList, DisplayMark, DisplayTextFragment};
use display_list_formatting::DisplayListFormatting;
use std::fmt;

struct Formatter {}

impl DisplayListFormatting for Formatter {
    fn format_annotation_type(annotation_type: &DisplayAnnotationType) -> String {
        match annotation_type {
            DisplayAnnotationType::Error => "error".to_string(),
            DisplayAnnotationType::Warning => "warning".to_string(),
            DisplayAnnotationType::Info => "info".to_string(),
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
            DisplayAnnotationType::Info => "-",
            DisplayAnnotationType::Note => "-",
            DisplayAnnotationType::Help => "-",
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
                format!("{} |", " ".repeat(lineno_width)),
                inline_marks,
                indent_char.repeat(range.0),
                mark.repeat(range.1 - range.0),
                first,
            )?;
            for line in rest {
                writeln!(
                    f,
                    "{}{}{} {}",
                    format!("{} |", " ".repeat(lineno_width)),
                    inline_marks,
                    " ".repeat(indent),
                    line,
                )?;
            }
        } else {
            writeln!(
                f,
                "{}{}{}{}",
                format!("{} |", " ".repeat(lineno_width)),
                inline_marks,
                indent_char.repeat(range.0),
                mark.repeat(range.1 - range.0),
            )?;
        }
        Ok(())
    }

    fn format_label(label: &[DisplayTextFragment]) -> String {
        label
            .iter()
            .map(|fragment| fragment.content.as_str())
            .collect::<Vec<&str>>()
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
                    writeln!(f, "{}{}{}", prefix, name, format!(": {}", first))?;
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
                        header_sigil,
                        path,
                        row,
                        col
                    )
                } else {
                    writeln!(f, "{}{} {}", " ".repeat(lineno_width), header_sigil, path,)
                }
            }
            DisplayLine::EmptySource => writeln!(f, "{} |", " ".repeat(lineno_width)),
            DisplayLine::Source {
                lineno,
                inline_marks,
                content,
                ..
            } => writeln!(
                f,
                "{:>width$} |{} {}",
                lineno,
                Self::format_inline_marks(&inline_marks, inline_marks_width),
                content,
                width = lineno_width,
            ),
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
