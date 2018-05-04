use display_list::{DisplayAnnotationPart, DisplayAnnotationType, DisplayHeaderType, DisplayLine,
                   DisplayList, DisplayMark, DisplayMarkType, DisplayTextFragment,
                   DisplayTextStyle};
use std::fmt;

use format::NoColorStylesheet;
#[cfg(feature = "ansi_term")]
use format_color::AnsiTermStylesheet;

pub enum StyleClass {
    Error,
    Warning,
    Info,
    Note,
    Help,

    LineNo,
}

pub trait Style {
    fn paint(&self, text: String) -> String;
    fn bold(&self) -> Box<Style>;
}

pub trait Stylesheet {
    fn get_style(&self, class: StyleClass) -> Box<Style>;
}

pub struct DisplayListFormatter {
    stylesheet: Box<Stylesheet>,
}

impl DisplayListFormatter {
    fn new(color: bool) -> Self {
        if color {
            Self {
                #[cfg(feature = "ansi_term")]
                stylesheet: Box::new(AnsiTermStylesheet {}),
                #[cfg(not(feature = "ansi_term"))]
                stylesheet: Box::new(NoColorStylesheet {}),
            }
        } else {
            Self {
                stylesheet: Box::new(NoColorStylesheet {}),
            }
        }
    }

    fn format_annotation_type(&self, annotation_type: &DisplayAnnotationType) -> String {
        match annotation_type {
            DisplayAnnotationType::Error => "error".to_string(),
            DisplayAnnotationType::Warning => "warning".to_string(),
            DisplayAnnotationType::Info => "info".to_string(),
            DisplayAnnotationType::Note => "note".to_string(),
            DisplayAnnotationType::Help => "help".to_string(),
        }
    }

    fn format_source_annotation_lines(
        &self,
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
        let style = match annotation_type {
            DisplayAnnotationType::Error => StyleClass::Error,
            DisplayAnnotationType::Warning => StyleClass::Warning,
            DisplayAnnotationType::Info => StyleClass::Info,
            DisplayAnnotationType::Note => StyleClass::Note,
            DisplayAnnotationType::Help => StyleClass::Help,
        };
        let color = self.stylesheet.get_style(style);
        let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);
        let indent = if indent_char == " " {
            indent_char.repeat(range.0)
        } else {
            format!("{}", color.paint(indent_char.repeat(range.0)))
        };
        if let Some((first, rest)) = self.format_label(label)
            .lines()
            .collect::<Vec<&str>>()
            .split_first()
        {
            writeln!(
                f,
                "{}{}{}{} {}",
                lineno_color.paint(format!("{} |", " ".repeat(lineno_width))),
                inline_marks,
                indent,
                color.paint(mark.repeat(range.1 - range.0)),
                color.paint(String::from(*first)),
            )?;
            for line in rest {
                writeln!(
                    f,
                    "{}{}{} {}",
                    lineno_color.paint(format!("{} |", " ".repeat(lineno_width))),
                    inline_marks,
                    " ".repeat(range.1),
                    color.paint(String::from(*line)),
                )?;
            }
        } else {
            writeln!(
                f,
                "{}{}{}{}",
                lineno_color.paint(format!("{} |", " ".repeat(lineno_width))),
                inline_marks,
                indent,
                color.paint(mark.repeat(range.1 - range.0)),
            )?;
        }
        Ok(())
    }

    fn format_label(&self, label: &[DisplayTextFragment]) -> String {
        label
            .iter()
            .map(|fragment| match fragment.style {
                DisplayTextStyle::Regular => fragment.content.clone(),
                DisplayTextStyle::Emphasis => format!("{}", fragment.content.clone()),
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn format_inline_marks(
        &self,
        inline_marks: &[DisplayMark],
        inline_marks_width: usize,
    ) -> String {
        format!(
            "{}{}",
            " ".repeat(inline_marks_width - inline_marks.len()),
            inline_marks
                .iter()
                .map(|mark| {
                    let sigil = match mark.mark_type {
                        DisplayMarkType::AnnotationThrough => "|",
                        DisplayMarkType::AnnotationStart => "/",
                    };
                    let style = match mark.annotation_type {
                        DisplayAnnotationType::Error => StyleClass::Error,
                        DisplayAnnotationType::Warning => StyleClass::Warning,
                        DisplayAnnotationType::Info => StyleClass::Info,
                        DisplayAnnotationType::Note => StyleClass::Note,
                        DisplayAnnotationType::Help => StyleClass::Help,
                    };
                    let color = self.stylesheet.get_style(style);
                    format!("{}", color.paint(String::from(sigil)))
                })
                .collect::<Vec<String>>()
                .join(""),
        )
    }

    fn format_line(
        &self,
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
                let style = match annotation_type {
                    DisplayAnnotationType::Error => StyleClass::Error,
                    DisplayAnnotationType::Warning => StyleClass::Warning,
                    DisplayAnnotationType::Info => StyleClass::Info,
                    DisplayAnnotationType::Note => StyleClass::Note,
                    DisplayAnnotationType::Help => StyleClass::Help,
                };
                let color = self.stylesheet.get_style(style);
                let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);
                let formatted_type = self.format_annotation_type(&annotation_type);
                let name = if let Some(id) = id {
                    format!("{}[{}]", formatted_type, id)
                } else {
                    formatted_type
                };
                let prefix = if *aligned {
                    format!("{} = ", " ".repeat(lineno_width))
                } else {
                    "".to_string()
                };
                if let Some((first, rest)) = self.format_label(label)
                    .lines()
                    .collect::<Vec<&str>>()
                    .split_first()
                {
                    let indent = prefix.len() + name.len() + 2;
                    writeln!(
                        f,
                        "{}{}{}",
                        lineno_color.bold().paint(prefix),
                        color.bold().paint(name),
                        format!(": {}", first)
                    )?;
                    for line in rest {
                        writeln!(f, "{}{}", " ".repeat(indent), format!("{}", line))?;
                    }
                }
            }
            DisplayLine::Origin {
                path,
                pos,
                header_type,
            } => {
                let header_sigil = match header_type {
                    DisplayHeaderType::Initial => String::from("-->"),
                    DisplayHeaderType::Continuation => String::from(":::"),
                };
                let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);
                if let Some((row, col)) = pos {
                    writeln!(
                        f,
                        "{}{} {}:{}:{}",
                        " ".repeat(lineno_width),
                        lineno_color.paint(header_sigil),
                        path,
                        row,
                        col
                    )?;
                } else {
                    writeln!(
                        f,
                        "{}{} {}",
                        " ".repeat(lineno_width),
                        lineno_color.paint(header_sigil),
                        path,
                    )?;
                }
            }
            DisplayLine::EmptySource => {
                let prefix = format!("{} |", " ".repeat(lineno_width));
                let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);
                writeln!(f, "{}", lineno_color.paint(prefix))?;
            }
            DisplayLine::Source {
                lineno,
                inline_marks,
                content,
                ..
            } => {
                let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);
                let prefix = format!("{:>width$} |", lineno, width = lineno_width);
                writeln!(
                    f,
                    "{}{} {}",
                    lineno_color.paint(prefix),
                    self.format_inline_marks(&inline_marks, inline_marks_width),
                    content,
                )?;
            }
            DisplayLine::SourceAnnotation {
                inline_marks,
                range,
                label,
                annotation_type,
                annotation_part,
            } => self.format_source_annotation_lines(
                f,
                lineno_width,
                self.format_inline_marks(&inline_marks, inline_marks_width),
                range,
                &label,
                &annotation_type,
                &annotation_part,
            )?,
            DisplayLine::Fold { inline_marks } => writeln!(
                f,
                "... {}",
                self.format_inline_marks(&inline_marks, inline_marks_width),
            )?,
        }
        Ok(())
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

        let dlf = DisplayListFormatter::new(true);

        for line in &self.body {
            dlf.format_line(f, line, lineno_width, inline_marks_width)?;
        }
        Ok(())
    }
}
