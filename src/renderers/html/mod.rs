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
use std::iter::repeat;

pub struct Renderer {}

pub fn get_renderer() -> impl RendererTrait {
    Renderer::new()
}

fn digits(n: usize) -> usize {
    let mut n = n;
    let mut sum = 0;
    while n != 0 {
        n /= 10;
        sum += 1;
    }
    sum
}

enum MarkKind {
    Vertical,
    Horizontal,
    DownRight,
    UpRight,
    UpLeft,
}

impl MarkKind {
    pub fn get(t: MarkKind) -> char {
        match t {
            MarkKind::Vertical => '│',
            MarkKind::Horizontal => '─',
            MarkKind::DownRight => '┌',
            MarkKind::UpRight => '└',
            MarkKind::UpLeft => '┘',
        }
    }
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()> {
        self.fmt_header(w)?;
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
        self.fmt_footer(w)?;
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
                let vertical_mark = MarkKind::get(MarkKind::Vertical);
                if let Some(lineno) = lineno {
                    write!(
                        w,
                        r#"<span class="lineno">{:>width$}</span> <span class="line">{}</span> "#,
                        lineno,
                        vertical_mark,
                        width = lineno_max
                    )?;
                } else {
                    write!(
                        w,
                        r#"<span class="lineno">{:>width$}</span> <span class="line">{}</span> "#,
                        "",
                        vertical_mark,
                        width = lineno_max
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
            DisplaySourceLine::Content { text } => {
                write!(w, r#" <span class="source">{}</span>"#, text)
            }
            DisplaySourceLine::Annotation { annotation, range } => {
                let indent = if range.start == 0 { 0 } else { range.start + 1 };
                write!(w, "{:>width$}", "", width = indent)?;
                let horizontal_mark = MarkKind::get(MarkKind::Horizontal);
                if range.start == 0 {
                    write!(
                        w,
                        "{}{} {}",
                        repeat(horizontal_mark)
                            .take(range.len())
                            .collect::<String>(),
                        MarkKind::get(MarkKind::UpLeft),
                        annotation.label,
                    )?;
                } else {
                    write!(
                        w,
                        "{} {}",
                        repeat(horizontal_mark)
                            .take(range.len())
                            .collect::<String>(),
                        annotation.label
                    )?;
                }
                Ok(())
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
                //S::fmt(
                //w,
                //format_args!(
                //"{}{}>",
                //MarkKind::get(MarkKind::Horizontal),
                //MarkKind::get(MarkKind::Horizontal),
                //),
                //&[StyleType::Emphasis, StyleType::LineNo],
                //)?;
                //write!(w, " {}", path)?;
                //if let Some(line) = pos.0 {
                //write!(w, ":{}", line)?;
                //}
                writeln!(w)
            }
            DisplayRawLine::Annotation { annotation, .. } => {
                let desc = self.get_annotation_type_style(&annotation.annotation_type);
                //let s = [StyleType::Emphasis, style];
                //S::fmt(w, desc, &s)?;
                //if let Some(id) = annotation.id {
                //S::fmt(w, format_args!("[{}]", id), &s)?;
                //}
                //S::fmt(
                //w,
                //format_args!(":  {}\n", annotation.label),
                //&[StyleType::Emphasis],
                //)
                Ok(())
            }
        }
    }

    fn get_annotation_type_style(&self, annotation_type: &AnnotationType) -> &'static str {
        match annotation_type {
            AnnotationType::Error => "error",
            AnnotationType::Warning => "warning",
            AnnotationType::Info => "info",
            AnnotationType::Note => "note",
            AnnotationType::Help => "help",
            AnnotationType::None => "",
        }
    }

    fn fmt_display_mark(
        &self,
        w: &mut impl std::io::Write,
        display_mark: &DisplayMark,
    ) -> std::io::Result<()> {
        let ch = match display_mark.mark_type {
            DisplayMarkType::AnnotationStart => MarkKind::get(MarkKind::DownRight),
            DisplayMarkType::AnnotationEnd => MarkKind::get(MarkKind::UpRight),
            DisplayMarkType::AnnotationThrough => MarkKind::get(MarkKind::Vertical),
        };
        write!(w, "{}", ch)?;
        Ok(())
    }

    fn fmt_header(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        writeln!(w, "<html><head><style>")?;
        writeln!(w, r#".lineno {{ color: red; }}"#)?;
        writeln!(w, r#".line {{ color: blue; }}"#)?;
        writeln!(w, r#".source {{ color: gray; }}"#)?;
        write!(w, "</style></head><body><pre>")
    }

    fn fmt_footer(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        write!(w, "</pre></body></html>")
    }
}

impl RendererTrait for Renderer {
    fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()> {
        Renderer::fmt(self, w, dl)
    }
}
