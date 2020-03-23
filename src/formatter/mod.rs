use std::{
    cell::Cell,
    cmp,
    fmt::{self, Display, Formatter, Write},
};

pub mod style;

use self::style::{Style, StyleClass, Stylesheet};

#[cfg(feature = "ansi_term")]
use crate::stylesheets::color::AnsiTermStylesheet;
use crate::{display_list::*, stylesheets::no_color::NoColorStylesheet};

pub struct DisplayFn<F: FnOnce(&mut Formatter<'_>) -> fmt::Result>(std::cell::Cell<Option<F>>);

impl<F: FnOnce(&mut Formatter<'_>) -> fmt::Result> DisplayFn<F> {
    pub fn new(f: F) -> Self {
        Self(Cell::new(Some(f)))
    }
}

impl<F: FnOnce(&mut Formatter<'_>) -> fmt::Result> Display for DisplayFn<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.take().ok_or(fmt::Error).and_then(|cl| cl(f))
    }
}

fn repeat_char(c: char, n: usize) -> String {
    let mut s = String::with_capacity(c.len_utf8() * n);
    for _ in 0..n {
        s.push(c);
    }
    s
}

fn format_repeat_char(c: char, n: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for _ in 0..n {
        f.write_char(c)?;
    }
    Ok(())
}

#[inline]
fn is_annotation_empty(annotation: &Annotation) -> bool {
    annotation
        .label
        .iter()
        .all(|fragment| fragment.content.is_empty())
}

#[cfg(feature = "ansi_term")]
#[inline]
pub fn get_term_style(color: bool) -> Box<dyn Stylesheet> {
    if color {
        Box::new(AnsiTermStylesheet)
    } else {
        Box::new(NoColorStylesheet)
    }
}

#[cfg(not(feature = "ansi_term"))]
#[inline]
pub fn get_term_style(_color: bool) -> Box<dyn Stylesheet> {
    Box::new(NoColorStylesheet)
}

impl fmt::Display for DisplayList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lineno_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source {
                lineno: Some(lineno),
                ..
            } => {
                if self.anonymized_line_numbers {
                    Self::ANONYMIZED_LINE_NUM.len()
                } else {
                    cmp::max(lineno.to_string().len(), max)
                }
            }
            _ => max,
        });
        let inline_marks_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => cmp::max(inline_marks.len(), max),
            _ => max,
        });

        for (i, line) in self.body.iter().enumerate() {
            self.format_line(line, lineno_width, inline_marks_width, f)?;
            if i + 1 < self.body.len() {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl DisplayList {
    const ANONYMIZED_LINE_NUM: &'static str = "LL";

    fn format_annotation_type(
        &self,
        annotation_type: &DisplayAnnotationType,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match annotation_type {
            DisplayAnnotationType::Error => f.write_str("error"),
            DisplayAnnotationType::Warning => f.write_str("warning"),
            DisplayAnnotationType::Info => f.write_str("info"),
            DisplayAnnotationType::Note => f.write_str("note"),
            DisplayAnnotationType::Help => f.write_str("help"),
            DisplayAnnotationType::None => Ok(()),
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

    fn format_label(
        &self,
        label: &[DisplayTextFragment],
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let emphasis_style = self.stylesheet.get_style(StyleClass::Emphasis);

        for fragment in label {
            match fragment.style {
                DisplayTextStyle::Regular => fragment.content.fmt(f)?,
                DisplayTextStyle::Emphasis => emphasis_style.paint(&fragment.content, f)?,
            }
        }
        Ok(())
    }

    fn format_annotation(
        &self,
        annotation: &Annotation,
        continuation: bool,
        in_source: bool,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let color = self.get_annotation_style(&annotation.annotation_type);

        let formatted_type = if let Some(id) = &annotation.id {
            DisplayFn::new(|f| {
                self.format_annotation_type(&annotation.annotation_type, f)?;
                f.write_char('[')?;
                f.write_str(id)?;
                f.write_char(']')
            })
            .to_string()
        } else {
            DisplayFn::new(|f| self.format_annotation_type(&annotation.annotation_type, f))
                .to_string()
        };

        if continuation {
            let indent = formatted_type.len() + 2;
            format_repeat_char(' ', indent, f)?;
            return self.format_label(&annotation.label, f);
        }
        if formatted_type.is_empty() {
            self.format_label(&annotation.label, f)
        } else {
            color.paint(&formatted_type, f)?;
            if !is_annotation_empty(annotation) {
                if in_source {
                    color.paint(
                        &DisplayFn::new(|f| {
                            f.write_str(": ")?;
                            self.format_label(&annotation.label, f)
                        })
                        .to_string(),
                        f,
                    )?;
                } else {
                    f.write_str(": ")?;
                    self.format_label(&annotation.label, f)?;
                }
            }
            Ok(())
        }
    }

    #[inline]
    fn format_source_line(
        &self,
        line: &DisplaySourceLine,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match line {
            DisplaySourceLine::Empty => Ok(()),
            DisplaySourceLine::Content { text, .. } => {
                f.write_char(' ')?;
                text.fmt(f)
            }
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

                color.paint(&repeat_char(indent_char, indent_length + 1), f)?;
                color.paint(&repeat_char(mark, range.1 - indent_length), f)?;

                if !is_annotation_empty(&annotation) {
                    f.write_char(' ')?;
                    color.paint(
                        &DisplayFn::new(|f| {
                            self.format_annotation(
                                annotation,
                                annotation_part == &DisplayAnnotationPart::LabelContinuation,
                                true,
                                f,
                            )
                        })
                        .to_string(),
                        f,
                    )?;
                }

                Ok(())
            }
        }
    }

    #[inline]
    fn format_lineno(&self, lineno: Option<usize>, lineno_width: usize) -> String {
        match lineno {
            Some(n) => format!("{:>width$}", n, width = lineno_width),
            None => repeat_char(' ', lineno_width),
        }
    }

    #[inline]
    fn format_raw_line(
        &self,
        line: &DisplayRawLine,
        lineno_width: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
                    format_repeat_char(' ', lineno_width, f)?;
                    lineno_color.paint(header_sigil, f)?;
                    f.write_char(' ')?;
                    path.fmt(f)?;
                    f.write_char(':')?;
                    col.fmt(f)?;
                    f.write_char(':')?;
                    row.fmt(f)
                } else {
                    format_repeat_char(' ', lineno_width, f)?;
                    lineno_color.paint(header_sigil, f)?;
                    f.write_char(' ')?;
                    path.fmt(f)
                }
            }
            DisplayRawLine::Annotation {
                annotation,
                source_aligned,
                continuation,
            } => {
                if *source_aligned {
                    if *continuation {
                        format_repeat_char(' ', lineno_width + 3, f)?;
                        self.format_annotation(annotation, *continuation, false, f)
                    } else {
                        let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);
                        format_repeat_char(' ', lineno_width, f)?;
                        f.write_char(' ')?;
                        lineno_color.paint("=", f)?;
                        f.write_char(' ')?;
                        self.format_annotation(annotation, *continuation, false, f)
                    }
                } else {
                    self.format_annotation(annotation, *continuation, false, f)
                }
            }
        }
    }

    #[inline]
    fn format_line(
        &self,
        dl: &DisplayLine,
        lineno_width: usize,
        inline_marks_width: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match dl {
            DisplayLine::Source {
                lineno,
                inline_marks,
                line,
            } => {
                let lineno_color = self.stylesheet.get_style(StyleClass::LineNo);
                if self.anonymized_line_numbers && lineno.is_some() {
                    lineno_color.paint(&format!("{} |", Self::ANONYMIZED_LINE_NUM), f)?;
                } else {
                    lineno_color.paint(
                        &format!("{} |", self.format_lineno(*lineno, lineno_width)),
                        f,
                    )?;
                }
                if *line != DisplaySourceLine::Empty {
                    if !inline_marks.is_empty() || 0 < inline_marks_width {
                        f.write_char(' ')?;
                        self.format_inline_marks(inline_marks, inline_marks_width, f)?;
                    }
                    self.format_source_line(line, f)?;
                } else if !inline_marks.is_empty() {
                    f.write_char(' ')?;
                    self.format_inline_marks(inline_marks, inline_marks_width, f)?;
                }
                Ok(())
            }
            DisplayLine::Fold { inline_marks } => {
                f.write_str("...")?;
                if !inline_marks.is_empty() || 0 < inline_marks_width {
                    format_repeat_char(' ', lineno_width, f)?;
                    self.format_inline_marks(inline_marks, inline_marks_width, f)?;
                }
                Ok(())
            }
            DisplayLine::Raw(line) => self.format_raw_line(line, lineno_width, f),
        }
    }

    fn format_inline_marks(
        &self,
        inline_marks: &[DisplayMark],
        inline_marks_width: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        format_repeat_char(' ', inline_marks_width - inline_marks.len(), f)?;
        for mark in inline_marks {
            self.get_annotation_style(&mark.annotation_type).paint(
                match mark.mark_type {
                    DisplayMarkType::AnnotationThrough => "|",
                    DisplayMarkType::AnnotationStart => "/",
                },
                f,
            )?;
        }
        Ok(())
    }
}
