//! DisplayListFormatter is a module handling the formatting of a
//! [DisplayList](super::display_list::DisplayList) into
//! a formatted string.
//!
//! Besides formatting into a string it also uses a `style::Stylesheet` to
//! provide additional styling like colors and emphasis to the text.

pub mod style;

use display_list::*;

use self::style::{StyleClass, Stylesheet};

#[cfg(feature = "ansi_term")]
use stylesheets::color::AnsiTermStylesheet;
use stylesheets::no_color::NoColorStylesheet;

fn repeat_char(c: char, n: usize) -> String {
    let mut s = String::with_capacity(c.len_utf8());
    s.push(c);
    s.repeat(n)
}

/// DisplayListFormatter' constructor accepts a single argument which
/// allows the formatter to optionally apply colors and emphasis
/// using `ansi_term` crate.
///
/// Example:
///
/// ```
/// use annotate_snippets::formatter::DisplayListFormatter;
/// use annotate_snippets::display_list::{DisplayList, DisplayLine, DisplaySourceLine};
///
/// let dlf = DisplayListFormatter::new(false); // Don't use colors
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
    stylesheet: Box<Stylesheet>,
}

impl DisplayListFormatter {
    /// Constructor for the struct. The argument `color` selects
    /// the stylesheet depending on the user preferences and `ansi_term`
    /// crate availability.
    pub fn new(color: bool) -> Self {
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

    /// Formats a `DisplayList` into a String.
    pub fn format(&self, dl: &DisplayList) -> String {
        let lineno_width = dl.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source {
                lineno: Some(lineno),
                ..
            } => {
                let width = lineno.to_string().len();
                if width > max {
                    width
                } else {
                    max
                }
            }
            _ => max,
        });
        let inline_marks_width = dl.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => {
                let width = inline_marks.len();
                if width > max {
                    width
                } else {
                    max
                }
            }
            _ => max,
        });

        dl.body
            .iter()
            .map(|line| self.format_line(line, lineno_width, inline_marks_width))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn format_annotation_type(&self, annotation_type: &DisplayAnnotationType) -> String {
        match annotation_type {
            DisplayAnnotationType::Error => "error".to_string(),
            DisplayAnnotationType::Warning => "warning".to_string(),
            DisplayAnnotationType::Info => "info".to_string(),
            DisplayAnnotationType::Note => "note".to_string(),
            DisplayAnnotationType::Help => "help".to_string(),
            DisplayAnnotationType::None => "".to_string(),
        }
    }

    fn format_label(&self, label: &[DisplayTextFragment]) -> String {
        let emphasis_style = self.stylesheet.get_style(StyleClass::Emphasis);
        label
            .iter()
            .map(|fragment| match fragment.style {
                DisplayTextStyle::Regular => fragment.content.clone(),
                DisplayTextStyle::Emphasis => emphasis_style.paint(fragment.content.clone()),
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
        let style = match annotation.annotation_type {
            DisplayAnnotationType::Error => StyleClass::Error,
            DisplayAnnotationType::Warning => StyleClass::Warning,
            DisplayAnnotationType::Info => StyleClass::Info,
            DisplayAnnotationType::Note => StyleClass::Note,
            DisplayAnnotationType::Help => StyleClass::Help,
            DisplayAnnotationType::None => StyleClass::None,
        };
        let color = self.stylesheet.get_style(style);
        let formatted_type = self.format_annotation_type(&annotation.annotation_type);
        let label = self.format_label(&annotation.label);

        let label_part = if label.is_empty() {
            "".to_string()
        } else if in_source {
            color.paint(format!(": {}", self.format_label(&annotation.label)))
        } else {
            format!(": {}", self.format_label(&annotation.label))
        };
        if continuation {
            let indent = if let Some(ref id) = annotation.id {
                formatted_type.len() + id.len() + 4
            } else if !formatted_type.is_empty() {
                formatted_type.len() + 2
            } else {
                2
            };
            return format!("{}{}", repeat_char(' ', indent), label);
        }
        if let Some(ref id) = annotation.id {
            format!(
                "{}{}",
                color.paint(format!("{}[{}]", formatted_type, id)),
                label_part
            )
        } else if !formatted_type.is_empty() {
            format!("{}{}", color.paint(formatted_type), label_part)
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
                let style = match annotation_type {
                    DisplayAnnotationType::Error => StyleClass::Error,
                    DisplayAnnotationType::Warning => StyleClass::Warning,
                    DisplayAnnotationType::Info => StyleClass::Info,
                    DisplayAnnotationType::Note => StyleClass::Note,
                    DisplayAnnotationType::Help => StyleClass::Help,
                    DisplayAnnotationType::None => StyleClass::None,
                };
                let color = self.stylesheet.get_style(style);
                let indent_length = match annotation_part {
                    DisplayAnnotationPart::LabelContinuation => range.1,
                    DisplayAnnotationPart::Consequitive => range.1,
                    _ => range.0,
                };
                let indent = color.paint(repeat_char(indent_char, indent_length + 1));
                let marks = color.paint(repeat_char(mark, range.1 - indent_length));
                let annotation = self.format_annotation(
                    annotation,
                    annotation_part == &DisplayAnnotationPart::LabelContinuation,
                    true,
                );
                if annotation.is_empty() {
                    return Some(format!("{}{}", indent, marks));
                }
                return Some(format!("{}{} {}", indent, marks, color.paint(annotation)));
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
                    DisplayHeaderType::Initial => String::from("-->"),
                    DisplayHeaderType::Continuation => String::from(":::"),
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
                            lineno_color.paint("=".to_string()),
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
                let lineno = self.format_lineno(*lineno, lineno_width);
                let marks = self.format_inline_marks(inline_marks, inline_marks_width);
                let lf = self.format_source_line(line);
                let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);

                let mut prefix = lineno_color.paint(format!("{} |", lineno));

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
                    let style = match mark.annotation_type {
                        DisplayAnnotationType::Error => StyleClass::Error,
                        DisplayAnnotationType::Warning => StyleClass::Warning,
                        DisplayAnnotationType::Info => StyleClass::Info,
                        DisplayAnnotationType::Note => StyleClass::Note,
                        DisplayAnnotationType::Help => StyleClass::Help,
                        DisplayAnnotationType::None => StyleClass::None,
                    };
                    let color = self.stylesheet.get_style(style);
                    color.paint(String::from(sigil))
                })
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
