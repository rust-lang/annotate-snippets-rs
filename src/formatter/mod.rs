//! DisplayListFormatter is a module handling the formatting of a
//! `DisplayList` into a formatted string.
//!
//! Besides formatting into a string it also uses a `style::Stylesheet` to
//! provide additional styling like colors and emphasis to the text.

pub mod style;

use self::style::{Style, StyleClass, Stylesheet};
use crate::display_list::*;
use std::cmp;

use crate::stylesheets::no_color::NoColorStylesheet;
#[cfg(feature = "ansi_term")]
use crate::stylesheets::color::AnsiTermStylesheet;

fn repeat_char(c: char, n: usize) -> String {
    let mut s = String::with_capacity(c.len_utf8());
    s.push(c);
    s.repeat(n)
}

/// DisplayListFormatter' constructor accepts two arguments:
///
/// * `color` allows the formatter to optionally apply colors and emphasis
/// using the `ansi_term` crate.
/// * `anonymized_line_numbers` will replace line numbers in the left column with the text `LL`.
///
/// Example:
///
/// ```
/// use annotate_snippets::formatter::DisplayListFormatter;
/// use annotate_snippets::display_list::{DisplayList, DisplayLine, DisplaySourceLine};
///
/// let dlf = DisplayListFormatter::new(false, false); // Don't use colors, Don't anonymize line numbers
///
/// let dl = DisplayList {
///     body: vec![
///         DisplayLine::Source {
///             lineno: Some(192),
///             inline_marks: vec![],
///             line: DisplaySourceLine::Content {
///                 text: "Example line of text".into(),
///                 range: (0, 21)
///             }
///         }
///     ]
/// };
/// assert_eq!(dlf.format(&dl), "192 | Example line of text");
/// ```
pub struct DisplayListFormatter {
    stylesheet: Box<dyn Stylesheet>,
    anonymized_line_numbers: bool,
}

impl DisplayListFormatter {
    const ANONYMIZED_LINE_NUM: &'static str = "LL";

    /// Constructor for the struct.
    ///
    /// The argument `color` selects the stylesheet depending on the user preferences and
    /// `ansi_term` crate availability.
    ///
    /// The argument `anonymized_line_numbers` will replace line numbers in the left column with
    /// the text `LL`. This can be useful to enable when running UI tests, such as in the Rust
    /// test suite.
    pub fn new(color: bool, anonymized_line_numbers: bool) -> Self {
        if color {
            Self {
                #[cfg(feature = "ansi_term")]
                stylesheet: Box::new(AnsiTermStylesheet {}),
                #[cfg(not(feature = "ansi_term"))]
                stylesheet: Box::new(NoColorStylesheet {}),
                anonymized_line_numbers,
            }
        } else {
            Self {
                stylesheet: Box::new(NoColorStylesheet {}),
                anonymized_line_numbers,
            }
        }
    }

    /// Formats a `DisplayList` into a String.
    pub fn format(&self, dl: &DisplayList) -> String {
        let lineno_width = dl.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source {
                lineno: Some(lineno),
                ..
            } => {
                if self.anonymized_line_numbers {
                    Self::ANONYMIZED_LINE_NUM.len()
                } else {
                    cmp::max(lineno.to_string().len(), max)
                }
            },
            _ => max,
        });
        let inline_marks_width = dl.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => cmp::max(inline_marks.len(), max),
            _ => max,
        });

        dl.body
            .iter()
            .map(|line| self.format_line(line, lineno_width, inline_marks_width))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn format_annotation_type(&self, annotation_type: &DisplayAnnotationType) -> &'static str {
        match annotation_type {
            DisplayAnnotationType::Error => "error",
            DisplayAnnotationType::Warning => "warning",
            DisplayAnnotationType::Info => "info",
            DisplayAnnotationType::Note => "note",
            DisplayAnnotationType::Help => "help",
            DisplayAnnotationType::None => "",
        }
    }

    fn get_annotation_style(&self, annotation_type: &DisplayAnnotationType) -> Box<dyn Style> {
        self.stylesheet.get_style(match annotation_type {
            DisplayAnnotationType::Error => StyleClass::Error,
            DisplayAnnotationType::Warning => StyleClass::Warning,
            DisplayAnnotationType::Info => StyleClass::Info,
            DisplayAnnotationType::Note => StyleClass::Note,
            DisplayAnnotationType::Help => StyleClass::Help,
            DisplayAnnotationType::None => StyleClass::None,
        })
    }

    fn format_label(&self, label: &[DisplayTextFragment]) -> String {
        let emphasis_style = self.stylesheet.get_style(StyleClass::Emphasis);
        label
            .iter()
            .map(|fragment| match fragment.style {
                DisplayTextStyle::Regular => fragment.content.clone(),
                DisplayTextStyle::Emphasis => emphasis_style.paint(&fragment.content),
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn format_annotation(
        &self,
        annotation: &Annotation,
        continuation: bool,
        in_source: bool,
    ) -> String {
        let color = self.get_annotation_style(&annotation.annotation_type);
        let formatted_type = if let Some(ref id) = annotation.id {
            format!(
                "{}[{}]",
                self.format_annotation_type(&annotation.annotation_type),
                id
            )
        } else {
            self.format_annotation_type(&annotation.annotation_type)
                .to_string()
        };
        let label = self.format_label(&annotation.label);

        let label_part = if label.is_empty() {
            "".to_string()
        } else if in_source {
            color.paint(&format!(": {}", self.format_label(&annotation.label)))
        } else {
            format!(": {}", self.format_label(&annotation.label))
        };
        if continuation {
            let indent = formatted_type.len() + 2;
            return format!("{}{}", repeat_char(' ', indent), label);
        }
        if !formatted_type.is_empty() {
            format!("{}{}", color.paint(&formatted_type), label_part)
        } else {
            label
        }
    }

    fn format_source_line(&self, line: &DisplaySourceLine) -> Option<String> {
        match line {
            DisplaySourceLine::Empty => None,
            DisplaySourceLine::Content { text, .. } => Some(format!(" {}", text)),
            DisplaySourceLine::Annotation {
                range,
                annotation,
                annotation_type,
                annotation_part,
            } => {
                let indent_char = match annotation_part {
                    DisplayAnnotationPart::Standalone => ' ',
                    DisplayAnnotationPart::LabelContinuation => ' ',
                    DisplayAnnotationPart::Consequitive => ' ',
                    DisplayAnnotationPart::MultilineStart => '_',
                    DisplayAnnotationPart::MultilineEnd => '_',
                };
                let mark = match annotation_type {
                    DisplayAnnotationType::Error => '^',
                    DisplayAnnotationType::Warning => '-',
                    DisplayAnnotationType::Info => '-',
                    DisplayAnnotationType::Note => '-',
                    DisplayAnnotationType::Help => '-',
                    DisplayAnnotationType::None => ' ',
                };
                let color = self.get_annotation_style(annotation_type);
                let indent_length = match annotation_part {
                    DisplayAnnotationPart::LabelContinuation => range.1,
                    DisplayAnnotationPart::Consequitive => range.1,
                    _ => range.0,
                };
                let indent = color.paint(&repeat_char(indent_char, indent_length + 1));
                let marks = color.paint(&repeat_char(mark, range.1 - indent_length));
                let annotation = self.format_annotation(
                    annotation,
                    annotation_part == &DisplayAnnotationPart::LabelContinuation,
                    true,
                );
                if annotation.is_empty() {
                    return Some(format!("{}{}", indent, marks));
                }
                Some(format!("{}{} {}", indent, marks, color.paint(&annotation)))
            }
        }
    }

    fn format_lineno(&self, lineno: Option<usize>, lineno_width: usize) -> String {
        match lineno {
            Some(n) => format!("{:>width$}", n, width = lineno_width),
            None => repeat_char(' ', lineno_width),
        }
    }

    fn format_raw_line(&self, line: &DisplayRawLine, lineno_width: usize) -> String {
        match line {
            DisplayRawLine::Origin {
                path,
                pos,
                header_type,
            } => {
                let header_sigil = match header_type {
                    DisplayHeaderType::Initial => "-->",
                    DisplayHeaderType::Continuation => ":::",
                };
                let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);

                if let Some((col, row)) = pos {
                    format!(
                        "{}{} {}:{}:{}",
                        repeat_char(' ', lineno_width),
                        lineno_color.paint(header_sigil),
                        path,
                        col,
                        row
                    )
                } else {
                    format!(
                        "{}{} {}",
                        repeat_char(' ', lineno_width),
                        lineno_color.paint(header_sigil),
                        path
                    )
                }
            }
            DisplayRawLine::Annotation {
                annotation,
                source_aligned,
                continuation,
            } => {
                if *source_aligned {
                    if *continuation {
                        format!(
                            "{}{}",
                            repeat_char(' ', lineno_width + 3),
                            self.format_annotation(annotation, *continuation, false)
                        )
                    } else {
                        let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);
                        format!(
                            "{} {} {}",
                            repeat_char(' ', lineno_width),
                            lineno_color.paint("="),
                            self.format_annotation(annotation, *continuation, false)
                        )
                    }
                } else {
                    self.format_annotation(annotation, *continuation, false)
                }
            }
        }
    }

    fn format_line(
        &self,
        dl: &DisplayLine,
        lineno_width: usize,
        inline_marks_width: usize,
    ) -> String {
        match dl {
            DisplayLine::Source {
                lineno,
                inline_marks,
                line,
            } => {
                let lineno = if self.anonymized_line_numbers  && lineno.is_some() {
                    Self::ANONYMIZED_LINE_NUM.to_string()
                } else {
                    self.format_lineno(*lineno, lineno_width)
                };
                let marks = self.format_inline_marks(inline_marks, inline_marks_width);
                let lf = self.format_source_line(line);
                let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);

                let mut prefix = lineno_color.paint(&format!("{} |", lineno));

                match lf {
                    Some(lf) => {
                        if !marks.is_empty() {
                            prefix.push_str(&format!(" {}", marks));
                        }
                        format!("{}{}", prefix, lf)
                    }
                    None => {
                        if !marks.trim().is_empty() {
                            prefix.push_str(&format!(" {}", marks));
                        }
                        prefix
                    }
                }
            }
            DisplayLine::Fold { inline_marks } => {
                let marks = self.format_inline_marks(inline_marks, inline_marks_width);
                let indent = lineno_width;
                if marks.trim().is_empty() {
                    String::from("...")
                } else {
                    format!("...{}{}", repeat_char(' ', indent), marks)
                }
            }
            DisplayLine::Raw(line) => self.format_raw_line(line, lineno_width),
        }
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
                    let color = self.get_annotation_style(&mark.annotation_type);
                    color.paint(sigil)
                })
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
