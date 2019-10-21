use crate::{
    formatter::{DisplayLine, FormattedSnippet, Mark, MarkKind, RawLine, SourceLine},
    renderer::{log10usize, max_line_num, max_marks_width, Renderer},
    DebugAndDisplay, Level, SpanWriter,
};
use std::io;

pub struct Ascii {
    use_ansi: bool,
}

impl Ascii {
    pub fn plain() -> Self {
        Ascii { use_ansi: false }
    }

    pub fn ansi() -> Self {
        Ascii { use_ansi: true }
    }
}

impl Ascii {
    #[inline(always)]
    fn reset(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[0m")
        } else {
            Ok(())
        }
    }

    fn bold(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[0;1m")
        } else {
            Ok(())
        }
    }

    // fg(Fixed(9))
    #[inline(always)]
    fn bright_red(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[0;31;1m")
        } else {
            Ok(())
        }
    }

    // bold + fg(Fixed(9))
    #[inline(always)]
    fn bold_bright_red(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[1;31;1m")
        } else {
            Ok(())
        }
    }

    // fg(Fixed(11))
    #[inline(always)]
    fn bright_yellow(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[0;33;1m")
        } else {
            Ok(())
        }
    }

    // bold + fg(Fixed(11))
    #[inline(always)]
    fn bold_bright_yellow(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[1;33;1m")
        } else {
            Ok(())
        }
    }

    // fg(Fixed(12))
    #[inline(always)]
    fn bright_blue(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[0;34;1m")
        } else {
            Ok(())
        }
    }

    // bold + fg(Fixed(12))
    #[inline(always)]
    fn bold_bright_blue(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[1;34;1m")
        } else {
            Ok(())
        }
    }

    // fg(Fixed(14))
    #[inline(always)]
    fn bright_cyan(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[0;36;1m")
        } else {
            Ok(())
        }
    }

    // bold + fg(Fixed(14))
    #[inline(always)]
    fn bold_bright_cyan(&self, w: &mut dyn io::Write) -> io::Result<()> {
        if self.use_ansi {
            write!(w, "\x1B[1;36;1m")
        } else {
            Ok(())
        }
    }

    // FIXME: emitted ANSI codes are highly redundant when repeated
    #[inline(always)]
    fn style_for(&self, level: Level, w: &mut dyn io::Write) -> io::Result<()> {
        match level {
            Level::Error => self.bright_red(w),
            Level::Warning => self.bright_yellow(w),
            Level::Info => self.bright_blue(w),
            Level::Note => self.reset(w),
            Level::Help => self.bright_cyan(w),
        }
    }

    // FIXME: emitted ANSI codes are highly redundant when repeated
    #[inline(always)]
    fn style_bold_for(&self, level: Level, w: &mut dyn io::Write) -> io::Result<()> {
        match level {
            Level::Error => self.bold_bright_red(w),
            Level::Warning => self.bold_bright_yellow(w),
            Level::Info => self.bold_bright_blue(w),
            Level::Note => self.reset(w),
            Level::Help => self.bold_bright_cyan(w),
        }
    }
}

impl Ascii {
    fn render_marks(&self, marks: &[Mark], w: &mut dyn io::Write) -> io::Result<()> {
        for mark in marks {
            self.style_for(mark.level, w)?;
            let c = match mark.kind {
                MarkKind::Start => '/',
                MarkKind::Continue => '|',
                MarkKind::Here => '\\',
            };
            write!(w, "{}", c)?;
        }
        self.reset(w)
    }

    fn render_source_line<Span: crate::Span>(
        &self,
        line: &SourceLine<'_, Span>,
        f: &dyn SpanWriter<Span>,
        w: &mut dyn io::Write,
    ) -> io::Result<()> {
        match line {
            SourceLine::Content { span, subspan } => f.write(w, span, subspan),
            SourceLine::Annotation { message, underline } => {
                write!(w, "{:>width$}", "", width = underline.0)?;
                self.style_bold_for(message.map_or(Level::Info, |message| message.level), w)?;
                // FIXME: respect level for pointer character
                if underline.0 == 0 {
                    write!(w, "{:_>width$} ", "^", width = underline.1)?;
                } else {
                    write!(w, "{:->width$} ", "", width = underline.1)?;
                }
                write!(
                    w,
                    "{}",
                    message.map_or(&"" as &dyn DebugAndDisplay, |message| message.text)
                )?;
                self.reset(w)
            }
            SourceLine::Empty => Ok(()),
        }
    }

    fn render_raw_line(
        &self,
        line: &RawLine<'_>,
        line_num_width: usize,
        w: &mut dyn io::Write,
    ) -> io::Result<()> {
        match line {
            &RawLine::Origin { path, pos } => {
                write!(w, "{:>width$}", "", width = line_num_width)?;
                self.bold_bright_blue(w)?;
                write!(w, "-->")?;
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
                writeln!(w, ":  {}", title.message.text)
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
                if let Some(lineno) = lineno {
                    write!(w, "{:>width$} | ", lineno, width = line_num_width)?;
                } else {
                    write!(w, "{:>width$} | ", "", width = line_num_width)?;
                }
                self.reset(w)?;
                write!(
                    w,
                    "{:>width$}",
                    "",
                    width = marks_width - inline_marks.len()
                )?;
                self.render_marks(inline_marks, w)?;
                self.render_source_line(line, f, w)?;
                writeln!(w)
            }
            DisplayLine::Raw(line) => self.render_raw_line(line, line_num_width, w),
        }
    }
}
