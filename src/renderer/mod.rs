use crate::{
    formatter::{DisplayLine, FormattedSnippet},
    SpanWriter,
};
use std::io;

pub fn max_line_num<Span: crate::Span>(snippet: &FormattedSnippet<'_, Span>) -> Option<usize> {
    // note that the line numbers of multiple slices might not be in order
    snippet
        .lines
        .iter()
        .filter_map(|line| match *line {
            DisplayLine::Source { lineno, .. } => lineno,
            _ => None,
        })
        .max()
}

pub fn max_marks_width<Span: crate::Span>(snippet: &FormattedSnippet<'_, Span>) -> usize {
    snippet
        .lines
        .iter()
        .filter_map(|line| match line {
            DisplayLine::Source { inline_marks, .. } => Some(inline_marks.len()),
            _ => None,
        })
        .max()
        .unwrap_or(0)
}

pub trait Renderer {
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

        Ok(())
    }

    fn render_line<Span: crate::Span>(
        &self,
        line: &DisplayLine<'_, Span>,
        line_num_width: usize,
        marks_width: usize,
        f: &dyn SpanWriter<Span>,
        w: &mut dyn io::Write,
    ) -> io::Result<()>;
}

fn log10usize(mut n: usize) -> usize {
    let mut sum = 0;
    while n != 0 {
        n /= 10;
        sum += 1;
    }
    sum
}

mod default;
pub use default::Ascii;
