use display_list::{DisplayAnnotationType, DisplayHeaderType, DisplayLine, DisplayList,
                   DisplayTextFragment};
use display_list_formatting::DisplayListFormatting;
use std::fmt;

struct Formatter {}

fn repeat_char(c: char, n: usize) -> String {
    let mut s = String::with_capacity(c.len_utf8());
    s.push(c);
    return s.repeat(n);
}

impl DisplayListFormatting for Formatter {
    fn format_source_annotation_parts(
        _annotation_type: &DisplayAnnotationType,
        indent_char: char,
        mark: char,
        range: &(usize, usize),
        lineno_width: usize,
    ) -> (String, String, String) {
        let lineno = format!("{} |", " ".repeat(lineno_width));
        let indent = repeat_char(indent_char, range.0);
        let pointer = repeat_char(mark, range.1 - range.0);
        return (lineno, indent, pointer);
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
