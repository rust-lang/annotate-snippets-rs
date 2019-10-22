use crate::{
    formatter::{DisplayLine, FormattedSnippet, Mark, MarkKind, RawLine, SourceLine},
    renderer::{log10usize, max_line_num, max_marks_width, Renderer},
    DebugAndDisplay, Level, SpanWriter,
};
use std::io;

#[derive(Debug, Copy, Clone, Default)]
pub struct Ascii {
    pub ansi: bool,
    #[allow(unused)] // TODO
    pub fold: bool,
    pub box_drawing: bool,
    #[doc(hidden)] // to allow structural creation with `Ascii { ..Default::default() }`
    pub __non_exhaustive: (),
}

impl Ascii {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ansi(&mut self, b: bool) -> &mut Self {
        self.ansi = b;
        self
    }

    pub fn box_drawing(&mut self, b: bool) -> &mut Self {
        self.box_drawing = b;
        self
    }
}

impl Ascii {
    #[inline(always)]
    fn reset(self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.ansi {
            write!(w, "\x1B[0m")
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn bold(self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.ansi {
            write!(w, "\x1B[0;1m")
        } else {
            Ok(())
        }
    }

    // bold + fg(Fixed(12))
    #[inline(always)]
    fn bold_bright_blue(self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.ansi {
            write!(w, "\x1B[1;34;1m")
        } else {
            Ok(())
        }
    }

    // FIXME: emitted ANSI codes are highly redundant when repeated
    #[inline(always)]
    fn style_for(self, level: Level, w: &mut dyn io::Write) -> io::Result<()> {
        if self.ansi {
            match level {
                Level::Error => write!(w, "\x1B[0;31;1m"),
                Level::Warning => write!(w, "\x1B[0;33;1m"),
                Level::Info => write!(w, "\x1B[0;34;1m"),
                Level::Note => self.reset(w),
                Level::Help => write!(w, "\x1B[0;36;1m"),
            }
        } else {
            Ok(())
        }
    }

    // FIXME: emitted ANSI codes are highly redundant when repeated
    #[inline(always)]
    fn style_bold_for(self, level: Level, w: &mut dyn io::Write) -> io::Result<()> {
        if self.ansi {
            match level {
                Level::Error => write!(w, "\x1B[1;31;1m"),
                Level::Warning => write!(w, "\x1B[1;33;1m"),
                Level::Info => write!(w, "\x1B[1;34;1m"),
                Level::Note => self.reset(w),
                Level::Help => write!(w, "\x1B[1;36;1m"),
            }
        } else {
            Ok(())
        }
    }
}

impl Ascii {
    fn render_marks(self, marks: &[Mark], w: &mut dyn io::Write) -> io::Result<()> {
        for mark in marks {
            self.style_for(mark.level, w)?;
            let c = if self.box_drawing {
                match mark.kind {
                    MarkKind::Start => '┌',
                    MarkKind::Continue => '│',
                    MarkKind::Here => '└',
                }
            } else {
                match mark.kind {
                    MarkKind::Start => '/',
                    MarkKind::Continue => '|',
                    MarkKind::Here => '\\',
                }
            };
            write!(w, "{}", c)?;
        }
        self.reset(w)
    }

    fn render_source_line<Span: crate::Span>(
        self,
        line: &SourceLine<'_, Span>,
        is_long: bool,
        f: &dyn SpanWriter<Span>,
        w: &mut dyn io::Write,
    ) -> io::Result<()> {
        match line {
            SourceLine::Content { span, subspan } => {
                write!(w, " ")?;
                f.write(w, span, subspan)
            }
            SourceLine::Annotation { message, underline } => {
                let (indent, len) = if is_long {
                    (0, underline.0 + underline.1 + 1)
                } else {
                    (underline.0 + 1, underline.1)
                };
                write!(w, "{:>width$}", "", width = indent)?;
                let level = message.map_or(Level::Info, |message| message.level);
                self.style_bold_for(level, w)?;
                if is_long {
                    if self.box_drawing {
                        write!(w, "{:─>width$} ", "┘", width = len)?;
                    } else {
                        write!(w, "{:_>width$} ", "^", width = len)?;
                    }
                } else {
                    match level {
                        Level::Error => write!(w, "{:^>width$} ", "", width = len)?,
                        Level::Warning => write!(w, "{:~>width$} ", "", width = len)?,
                        Level::Info | Level::Help | Level::Note => {
                            write!(w, "{:->width$} ", "", width = len)?
                        }
                    }
                }
                write!(
                    w,
                    "{}",
                    message.map_or(&"" as &dyn DebugAndDisplay, |message| message.text)
                )
            }
            SourceLine::Empty => Ok(()),
        }
    }

    fn render_raw_line(
        self,
        line: &RawLine<'_>,
        line_num_width: usize,
        w: &mut dyn io::Write,
    ) -> io::Result<()> {
        match line {
            &RawLine::Origin { path, pos } => {
                write!(w, "{:>width$}", "", width = line_num_width)?;
                self.bold_bright_blue(w)?;
                if self.box_drawing {
                    write!(w, "═╦═")?;
                } else {
                    write!(w, "-->")?;
                }
                self.reset(w)?;
                write!(w, " {}", path)?;
                if let Some((line, column)) = pos {
                    write!(w, ":{}:{}", line, column)?;
                }
                writeln!(w)
            }
            RawLine::Message { message } => {
                self.style_for(message.level, w)?;
                let cta = match message.level {
                    Level::Error => "error",
                    Level::Warning => "warning",
                    Level::Info => "info",
                    Level::Note => "note",
                    Level::Help => "help",
                };
                write!(w, "{:>width$} = {}", "", cta, width = line_num_width)?;
                writeln!(w, ": {}", message.text)
            }
            RawLine::Title { title } => {
                self.style_bold_for(title.message.level, w)?;
                let cta = match title.message.level {
                    Level::Error => "error",
                    Level::Warning => "warning",
                    Level::Info => "info",
                    Level::Note => "note",
                    Level::Help => "help",
                };
                write!(w, "{}", cta)?;
                if let Some(code) = title.code {
                    write!(w, "[{}]", code)?;
                }
                self.bold(w)?;
                writeln!(w, ": {}", title.message.text)
            }
        }
    }
}

impl Renderer for Ascii {
    fn render<'a, Span: crate::Span>(
        &self,
        snippet: &FormattedSnippet<'a, Span>,
        f: &dyn SpanWriter<Span>,
        w: &mut dyn io::Write,
    ) -> io::Result<()> {
        let max_line_num = max_line_num(snippet).unwrap_or(0);
        let marks_width = max_marks_width(snippet);

        for line in &snippet.lines {
            self.render_line(line, log10usize(max_line_num), marks_width, f, w)?;
        }

        self.reset(w)
    }

    fn render_line<Span: crate::Span>(
        &self,
        line: &DisplayLine<'_, Span>,
        line_num_width: usize,
        marks_width: usize,
        f: &dyn SpanWriter<Span>,
        w: &mut dyn io::Write,
    ) -> io::Result<()> {
        match line {
            DisplayLine::Source {
                lineno,
                inline_marks,
                line,
            } => {
                self.bold_bright_blue(w)?;
                let sep = if self.box_drawing { '║' } else { '|' };
                if let Some(lineno) = lineno {
                    write!(w, "{:>width$} {} ", lineno, sep, width = line_num_width)?;
                } else {
                    write!(w, "{:>width$} {} ", "", sep, width = line_num_width)?;
                }
                self.reset(w)?;
                write!(
                    w,
                    "{:>width$}",
                    "",
                    width = marks_width - inline_marks.len()
                )?;
                self.render_marks(inline_marks, w)?;
                let is_long = inline_marks
                    .last()
                    .map_or(false, |mark| mark.kind == MarkKind::Here);
                self.render_source_line(line, is_long, f, w)?;
                writeln!(w)
            }
            DisplayLine::Raw(line) => self.render_raw_line(line, line_num_width, w),
        }
    }
}
