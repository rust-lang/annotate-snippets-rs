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

impl Span for &str {
    /// Byte index range into this string.
    type Subspan = Range<usize>;
    /// Byte index into this string.
    type Pos = usize;

    fn start(&self) -> Self::Pos {
        0
    }

    fn end(&self) -> Self::Pos {
        self.len()
    }

    fn slice(&self, range: Range<Self::Pos>) -> Range<usize> {
        range
    }
}

impl SpanFormatter<&str> for () {
    fn first_line(&self, span: &&str) -> WithLineNumber<Range<usize>> {
        let start = 0;
        let end = span
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, &b)| b == b'\n')
            .map_or_else(|| span.len(), |(i, _)| i);
        WithLineNumber {
            data: start..end,
            line_num: 1,
        }
    }

    fn next_line(
        &self,
        span: &&str,
        subspan: &WithLineNumber<Range<usize>>,
    ) -> Option<WithLineNumber<Range<usize>>> {
        let start = subspan.data.end + 1;
        let end = span
            .get(start..)?
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, &b)| b == b'\n')
            .map_or_else(|| span.len(), |(i, _)| i + start);
        Some(WithLineNumber {
            data: start..end,
            line_num: subspan.line_num + 1,
        })
    }

    fn count_columns(&self, span: &&str, subspan: &Range<usize>) -> usize {
        span[subspan.start..subspan.end].chars().count()
    }
}

impl SpanWriter<&str> for () {
    fn write(
        &self,
        w: &mut dyn io::Write,
        span: &&str,
        subspan: &Range<usize>,
    ) -> io::Result<()> {
        w.write_all(span[subspan.start..subspan.end].as_bytes())
    }
}

impl<S: Span> Span for WithLineNumber<S> {
    type Subspan = S::Subspan;
    type Pos = S::Pos;

    fn start(&self) -> S::Pos {
        self.data.start()
    }

    fn end(&self) -> S::Pos {
        self.data.end()
    }

    fn slice(&self, range: Range<S::Pos>) -> S::Subspan {
        self.data.slice(range)
    }
}

impl<S: Span, SF: SpanFormatter<S>> SpanFormatter<WithLineNumber<S>> for SF {
    fn first_line(&self, span: &WithLineNumber<S>) -> WithLineNumber<S::Subspan> {
        let wln = self.first_line(&span.data);
        WithLineNumber {
            data: wln.data,
            line_num: wln.line_num + span.line_num - 1,
        }
    }

    fn next_line(
        &self,
        span: &WithLineNumber<S>,
        subspan: &WithLineNumber<S::Subspan>,
    ) -> Option<WithLineNumber<S::Subspan>> {
        self.next_line(&span.data, subspan)
    }

    fn count_columns(&self, span: &WithLineNumber<S>, subspan: &S::Subspan) -> usize {
        self.count_columns(&span.data, subspan)
    }
}

impl<S: Span, SW: SpanWriter<S>> SpanWriter<WithLineNumber<S>> for SW {
    fn write(
        &self,
        w: &mut dyn io::Write,
        span: &WithLineNumber<S>,
        subspan: &S::Subspan,
    ) -> io::Result<()> {
        self.write(w, &span.data, subspan)
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
