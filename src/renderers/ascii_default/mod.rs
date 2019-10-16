pub mod styles;

#[cfg(feature = "ansi_term")]
use crate::renderers::ascii_default::styles::color::Style;
#[cfg(feature = "termcolor")]
use crate::renderers::ascii_default::styles::color2::Style;
#[cfg(all(not(feature = "ansi_term"), not(feature = "termcolor")))]
use crate::renderers::ascii_default::styles::plain::Style;

use super::Renderer as RendererTrait;
use crate::annotation::AnnotationType;
use crate::display_list::line::DisplayLine;
use crate::display_list::line::DisplayMark;
use crate::display_list::line::DisplayMarkType;
use crate::display_list::line::DisplayRawLine;
use crate::display_list::line::DisplaySourceLine;
use crate::DisplayList;
use std::cmp;
use std::io::Write;
use std::marker::PhantomData;
use styles::Style as StyleTrait;
use styles::StyleType;

fn digits(n: usize) -> usize {
    let mut n = n;
    let mut sum = 0;
    while n != 0 {
        n /= 10;
        sum += 1;
    }
    sum
}

pub struct Renderer<S: StyleTrait> {
    style: PhantomData<S>,
}

pub fn get_renderer() -> impl RendererTrait {
    Renderer::<Style>::new()
}

impl<S: StyleTrait> Renderer<S> {
    pub fn new() -> Self {
        Renderer { style: PhantomData }
    }

    pub fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()> {
        let lineno_max = dl.body.iter().rev().find_map(|line| {
            if let DisplayLine::Source {
                lineno: Some(lineno),
                ..
            } = line
            {
                Some(digits(*lineno))
            } else {
                None
            }
        });
        let inline_marks_width = dl.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => cmp::max(inline_marks.len(), max),
            _ => max,
        });
        for line in &dl.body {
            self.fmt_line(w, line, lineno_max, inline_marks_width)?;
        }
        Ok(())
    }

    fn fmt_line(
        &self,
        w: &mut impl Write,
        line: &DisplayLine,
        lineno_max: Option<usize>,
        inline_marks_width: usize,
    ) -> std::io::Result<()> {
        let lineno_max = lineno_max.unwrap_or(1);
        match line {
            DisplayLine::Source {
                lineno,
                inline_marks,
                line,
            } => {
                let style = &[StyleType::LineNo, StyleType::Emphasis];
                if let Some(lineno) = lineno {
                    S::fmt(
                        w,
                        format_args!("{:>width$} | ", lineno, width = lineno_max),
                        style,
                    )?;
                } else {
                    S::fmt(
                        w,
                        format_args!("{:>width$} | ", "", width = lineno_max),
                        style,
                    )?;
                }
                write!(
                    w,
                    "{:>width$}",
                    "",
                    width = inline_marks_width - inline_marks.len()
                )?;
                for mark in inline_marks {
                    self.fmt_display_mark(w, mark)?;
                }
                self.fmt_source_line(w, line)?;
                writeln!(w)
            }
            DisplayLine::Raw(l) => self.fmt_raw_line(w, l, lineno_max),
        }
    }

    fn fmt_source_line(
        &self,
        w: &mut impl std::io::Write,
        line: &DisplaySourceLine,
    ) -> std::io::Result<()> {
        match line {
            DisplaySourceLine::Content { text } => write!(w, " {}", text),
            DisplaySourceLine::Annotation { annotation, range } => {
                let (_, style) = self.get_annotation_type_style(&annotation.annotation_type);
                let styles = [StyleType::Emphasis, style];
                let indent = if range.start == 0 { 0 } else { range.start + 1 };
                write!(w, "{:>width$}", "", width = indent)?;
                if range.start == 0 {
                    S::fmt(
                        w,
                        format_args!(
                            "{:_>width$} {}",
                            "^",
                            annotation.label,
                            width = range.len() + 1
                        ),
                        &styles,
                    )
                } else {
                    S::fmt(
                        w,
                        format_args!("{:->width$} {}", "", annotation.label, width = range.len()),
                        &styles,
                    )
                }
            }
            DisplaySourceLine::Empty => Ok(()),
        }
    }

    fn fmt_raw_line(
        &self,
        w: &mut impl std::io::Write,
        line: &DisplayRawLine,
        lineno_max: usize,
    ) -> std::io::Result<()> {
        match line {
            DisplayRawLine::Origin { path, pos } => {
                write!(w, "{:>width$}", "", width = lineno_max)?;
                S::fmt(w, "-->", &[StyleType::Emphasis, StyleType::LineNo])?;
                write!(w, " {}", path)?;
                if let Some(line) = pos.0 {
                    write!(w, ":{}", line)?;
                }
                writeln!(w)
            }
            DisplayRawLine::Annotation { annotation, .. } => {
                let (desc, style) = self.get_annotation_type_style(&annotation.annotation_type);
                let s = [StyleType::Emphasis, style];
                S::fmt(w, desc, &s)?;
                if let Some(id) = annotation.id {
                    S::fmt(w, format_args!("[{}]", id), &s)?;
                }
                S::fmt(
                    w,
                    format_args!(":  {}\n", annotation.label),
                    &[StyleType::Emphasis],
                )
            }
        }
    }

    fn get_annotation_type_style(
        &self,
        annotation_type: &AnnotationType,
    ) -> (&'static str, StyleType) {
        match annotation_type {
            AnnotationType::Error => ("error", StyleType::Error),
            AnnotationType::Warning => ("warning", StyleType::Warning),
            AnnotationType::Info => ("info", StyleType::Info),
            AnnotationType::Note => ("note", StyleType::Note),
            AnnotationType::Help => ("help", StyleType::Help),
            AnnotationType::None => ("", StyleType::None),
        }
    }

    fn fmt_display_mark(
        &self,
        w: &mut impl std::io::Write,
        display_mark: &DisplayMark,
    ) -> std::io::Result<()> {
        let (_, style) = self.get_annotation_type_style(&display_mark.annotation_type);
        let ch = match display_mark.mark_type {
            DisplayMarkType::AnnotationStart => '/',
            DisplayMarkType::AnnotationThrough => '|',
        };
        S::fmt(w, ch, &[style])
    }
}

impl<S: StyleTrait> RendererTrait for Renderer<S> {
    fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()> {
        Renderer::fmt(self, w, dl)
    }
}
