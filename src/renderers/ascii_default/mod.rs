pub mod styles;

use super::Renderer as RendererTrait;
use crate::annotation::AnnotationType;
use crate::display_list::annotation::Annotation;
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
                if let Some(lineno) = lineno {
                    write!(w, "{:>1$}", lineno, lineno_max)?;
                } else {
                    write!(w, "{:>1$}", "", lineno_max)?;
                }
                write!(w, " | ")?;
                write!(w, "{:>1$}", "", inline_marks_width - inline_marks.len())?;
                for mark in inline_marks {
                    self.fmt_display_mark(w, mark)?;
                }
                self.fmt_source_line(w, line)?;
                write!(w, "\n")
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
            DisplaySourceLine::Annotation {
                annotation,
                range: (start, end),
            } => {
                let indent = if start == &0 { 0 } else { start + 1 };
                write!(w, "{:>1$}", "", indent)?;
                if start == &0 {
                    write!(w, "{:_>1$}", "^", end - start + 1)?;
                } else {
                    write!(w, "{:->1$}", "", end - start)?;
                }
                write!(w, " {}", annotation.label)
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
                S::fmt(w, format_args!("{:>1$}", "", lineno_max))?;
                S::fmt(w, format_args!("--> {}", path))?;
                if let Some(line) = pos.0 {
                    S::fmt(w, format_args!(":{}", line))?;
                }
                write!(w, "\n")
            }
            DisplayRawLine::Annotation { annotation, .. } => {
                self.fmt_annotation(w, annotation)?;
                if let Some(id) = annotation.id {
                    write!(w, "[{}]", id)?;
                }
                writeln!(w, ": {}", annotation.label)
            }
        }
    }

    fn fmt_annotation(
        &self,
        w: &mut impl std::io::Write,
        annotation: &Annotation,
    ) -> std::io::Result<()> {
        match annotation.annotation_type {
            AnnotationType::None => Ok(()),
            AnnotationType::Error => write!(w, "error"),
            AnnotationType::Warning => write!(w, "warning"),
            AnnotationType::Info => write!(w, "info"),
            AnnotationType::Note => write!(w, "note"),
            AnnotationType::Help => write!(w, "help"),
        }
    }

    fn fmt_display_mark(
        &self,
        w: &mut impl std::io::Write,
        display_mark: &DisplayMark,
    ) -> std::io::Result<()> {
        match display_mark.mark_type {
            DisplayMarkType::AnnotationStart => write!(w, "/"),
            DisplayMarkType::AnnotationThrough => write!(w, "|"),
        }
    }
}

impl<S: StyleTrait> RendererTrait for Renderer<S> {
    fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()> {
        Renderer::fmt(self, w, dl)
    }
}
