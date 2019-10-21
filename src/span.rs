use std::{io, ops::Range};

pub trait Span: Clone {
    type Subspan: Span<Pos = Self::Pos>;
    type Pos: Ord + Copy;

    fn start(&self) -> Self::Pos;
    fn end(&self) -> Self::Pos;
    fn slice(&self, range: Range<Self::Pos>) -> Self::Subspan;
}

pub trait SpanFormatter<Span: self::Span> {
    fn first_line(&self, span: &Span) -> WithLineNumber<Span::Subspan>;
    fn next_line(
        &self,
        span: &Span,
        subspan: &WithLineNumber<Span::Subspan>,
    ) -> Option<WithLineNumber<Span::Subspan>>;
    fn count_columns(&self, span: &Span, subspan: &Span::Subspan) -> usize;
}

pub trait SpanWriter<Span: crate::Span> {
    fn write(&self, w: &mut dyn io::Write, span: &Span, subspan: &Span::Subspan) -> io::Result<()>;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WithLineNumber<T> {
    pub line_num: usize,
    pub data: T,
}

impl Span for WithLineNumber<&str> {
    /// Byte index range into this string.
    type Subspan = Range<usize>;
    /// Byte index into this string.
    type Pos = usize;

    fn start(&self) -> Self::Pos {
        0
    }

    fn end(&self) -> Self::Pos {
        self.data.len()
    }

    fn slice(&self, range: Range<Self::Pos>) -> Range<usize> {
        range
    }
}

impl SpanFormatter<WithLineNumber<&str>> for () {
    fn first_line(&self, span: &WithLineNumber<&str>) -> WithLineNumber<Range<usize>> {
        let start = 0;
        let end = span
            .data
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, &b)| b == b'\n')
            .map_or_else(|| span.data.len(), |(i, _)| i);
        WithLineNumber {
            data: start..end,
            line_num: span.line_num,
        }
    }

    fn next_line(
        &self,
        span: &WithLineNumber<&str>,
        subspan: &WithLineNumber<Range<usize>>,
    ) -> Option<WithLineNumber<Range<usize>>> {
        let start = subspan.data.end + 1;
        let end = span
            .data
            .get(start..)?
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, &b)| b == b'\n')
            .map_or_else(|| span.data.len(), |(i, _)| i + start);
        Some(WithLineNumber {
            data: start..end,
            line_num: subspan.line_num + 1,
        })
    }

    fn count_columns(&self, span: &WithLineNumber<&str>, subspan: &Range<usize>) -> usize {
        span.data[subspan.start..subspan.end].chars().count()
    }
}

impl SpanWriter<WithLineNumber<&str>> for () {
    fn write(
        &self,
        w: &mut dyn io::Write,
        span: &WithLineNumber<&str>,
        subspan: &Range<usize>,
    ) -> io::Result<()> {
        w.write_all(span.data[subspan.start..subspan.end].as_bytes())
    }
}

impl Span for Range<usize> {
    /// Byte index into the source, _not_ this range.
    /// This is a "sibling" subspan.
    type Subspan = Range<usize>;
    /// Byte index into the source.
    type Pos = usize;

    fn start(&self) -> Self::Pos {
        self.start
    }

    fn end(&self) -> Self::Pos {
        self.end
    }

    fn slice(&self, range: Range<Self::Pos>) -> Range<usize> {
        range
    }
}

impl SpanFormatter<Range<usize>> for &str {
    fn first_line(&self, span: &Range<usize>) -> WithLineNumber<Range<usize>> {
        let start = self[..span.start]
            .as_bytes()
            .iter()
            .enumerate()
            .rfind(|(_, &b)| b == b'\n')
            .map_or_else(|| 0, |(i, _)| i + 1);
        let end = self[start..]
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, &b)| b == b'\n')
            .map_or_else(|| span.len(), |(i, _)| i + start);
        #[allow(clippy::naive_bytecount)]
        WithLineNumber {
            data: start..end,
            line_num: self[..start]
                .as_bytes()
                .iter()
                .filter(|&&b| b == b'\n')
                .count(),
        }
    }

    fn next_line(
        &self,
        span: &Range<usize>,
        subspan: &WithLineNumber<Range<usize>>,
    ) -> Option<WithLineNumber<Range<usize>>> {
        let start = subspan.data.end + 1;
        let end = self
            .get(start..)?
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, &b)| b == b'\n')
            .map_or_else(|| span.end, |(i, _)| i + start);
        if start <= span.end {
            Some(WithLineNumber {
                data: start..end,
                line_num: subspan.line_num + 1,
            })
        } else {
            None
        }
    }

    fn count_columns(&self, _span: &Range<usize>, subspan: &Range<usize>) -> usize {
        self[subspan.start..subspan.end].chars().count()
    }
}

impl SpanWriter<Range<usize>> for &str {
    fn write(
        &self,
        w: &mut dyn io::Write,
        _span: &Range<usize>,
        subspan: &Range<usize>,
    ) -> io::Result<()> {
        w.write_all(self[subspan.start..subspan.end].as_bytes())
    }
}
