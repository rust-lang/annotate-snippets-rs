use crate::renderer::{char_width, num_overlap, LineAnnotation, LineAnnotationType};
use crate::{Annotation, Level};
use std::cmp::{max, min};
use std::ops::Range;

#[derive(Debug)]
pub(crate) struct SourceMap<'a> {
    lines: Vec<LineInfo<'a>>,
    source: &'a str,
}

impl<'a> SourceMap<'a> {
    pub(crate) fn new(source: &'a str, line_start: usize) -> Self {
        let mut current_index = 0;

        let mut mapping = vec![];
        for (idx, (line, end_line)) in CursorLines::new(source).enumerate() {
            let line_length = line.len();
            let line_range = current_index..current_index + line_length;
            let end_line_size = end_line.len();

            mapping.push(LineInfo {
                line,
                line_index: line_start + idx,
                start_byte: line_range.start,
                end_byte: line_range.end + end_line_size,
                end_line_size,
            });

            current_index += line_length + end_line_size;
        }
        Self {
            lines: mapping,
            source,
        }
    }

    pub(crate) fn get_line(&self, idx: usize) -> Option<&'a str> {
        self.lines
            .iter()
            .find(|l| l.line_index == idx)
            .map(|info| info.line)
    }

    pub(crate) fn span_to_locations(&self, span: Range<usize>) -> (Loc, Loc) {
        let start_info = self
            .lines
            .iter()
            .find(|info| span.start >= info.start_byte && span.start < info.end_byte)
            .unwrap_or(self.lines.last().unwrap());
        let (mut start_char_pos, start_display_pos) = start_info.line
            [0..(span.start - start_info.start_byte).min(start_info.line.len())]
            .chars()
            .fold((0, 0), |(char_pos, byte_pos), c| {
                let display = char_width(c);
                (char_pos + 1, byte_pos + display)
            });
        // correct the char pos if we are highlighting the end of a line
        if (span.start - start_info.start_byte).saturating_sub(start_info.line.len()) > 0 {
            start_char_pos += 1;
        }
        let start = Loc {
            line: start_info.line_index,
            char: start_char_pos,
            display: start_display_pos,
            byte: span.start,
        };

        if span.start == span.end {
            return (start, start);
        }

        let end_info = self
            .lines
            .iter()
            .find(|info| info.end_byte > span.end.saturating_sub(1))
            .unwrap_or(self.lines.last().unwrap());
        let (mut end_char_pos, end_display_pos) = end_info.line
            [0..(span.end - end_info.start_byte).min(end_info.line.len())]
            .chars()
            .fold((0, 0), |(char_pos, byte_pos), c| {
                let display = char_width(c);
                (char_pos + 1, byte_pos + display)
            });

        // correct the char pos if we are highlighting the end of a line
        if (span.end - end_info.start_byte).saturating_sub(end_info.line.len()) > 0 {
            end_char_pos += 1;
        }
        let mut end = Loc {
            line: end_info.line_index,
            char: end_char_pos,
            display: end_display_pos,
            byte: span.end,
        };
        if start.line != end.line && end.byte > end_info.end_byte - end_info.end_line_size {
            end.char += 1;
            end.display += 1;
        }

        (start, end)
    }

    pub(crate) fn annotated_lines(
        &self,
        annotations: Vec<Annotation<'a>>,
        fold: bool,
    ) -> Vec<AnnotatedLineInfo<'a>> {
        let source_len = self.source.len();
        if let Some(bigger) = annotations.iter().find_map(|x| {
            // Allow highlighting one past the last character in the source.
            if source_len + 1 < x.range.end {
                Some(&x.range)
            } else {
                None
            }
        }) {
            panic!("Annotation range `{bigger:?}` is beyond the end of buffer `{source_len}`")
        }

        let mut annotated_line_infos = self
            .lines
            .iter()
            .map(|info| AnnotatedLineInfo {
                line: info.line,
                line_index: info.line_index,
                annotations: vec![],
            })
            .collect::<Vec<_>>();
        let mut multiline_annotations = vec![];

        for Annotation {
            range,
            label,
            level,
        } in annotations
        {
            let (lo, mut hi) = self.span_to_locations(range);

            // Watch out for "empty spans". If we get a span like 6..6, we
            // want to just display a `^` at 6, so convert that to
            // 6..7. This is degenerate input, but it's best to degrade
            // gracefully -- and the parser likes to supply a span like
            // that for EOF, in particular.

            if lo.display == hi.display && lo.line == hi.line {
                hi.display += 1;
            }

            if lo.line == hi.line {
                let line_ann = LineAnnotation {
                    start: lo,
                    end: hi,
                    level,
                    label,
                    annotation_type: LineAnnotationType::Singleline,
                };
                self.add_annotation_to_file(&mut annotated_line_infos, lo.line, line_ann);
            } else {
                multiline_annotations.push(MultilineAnnotation {
                    depth: 1,
                    start: lo,
                    end: hi,
                    level,
                    label,
                    overlaps_exactly: false,
                });
            }
        }

        // Find overlapping multiline annotations, put them at different depths
        multiline_annotations
            .sort_by_key(|ml| (ml.start.line, usize::MAX - ml.end.line, ml.start.byte));
        for ann in multiline_annotations.clone() {
            for a in &mut multiline_annotations {
                // Move all other multiline annotations overlapping with this one
                // one level to the right.
                if !ann.same_span(a)
                    && num_overlap(ann.start.line, ann.end.line, a.start.line, a.end.line, true)
                {
                    a.increase_depth();
                } else if ann.same_span(a) && &ann != a {
                    a.overlaps_exactly = true;
                } else {
                    break;
                }
            }
        }

        let mut max_depth = 0; // max overlapping multiline spans
        for ann in &multiline_annotations {
            max_depth = max(max_depth, ann.depth);
        }
        // Change order of multispan depth to minimize the number of overlaps in the ASCII art.
        for a in &mut multiline_annotations {
            a.depth = max_depth - a.depth + 1;
        }
        for ann in multiline_annotations {
            let mut end_ann = ann.as_end();
            if ann.overlaps_exactly {
                end_ann.annotation_type = LineAnnotationType::Singleline;
            } else {
                // avoid output like
                //
                //  |        foo(
                //  |   _____^
                //  |  |_____|
                //  | ||         bar,
                //  | ||     );
                //  | ||      ^
                //  | ||______|
                //  |  |______foo
                //  |         baz
                //
                // and instead get
                //
                //  |       foo(
                //  |  _____^
                //  | |         bar,
                //  | |     );
                //  | |      ^
                //  | |      |
                //  | |______foo
                //  |        baz
                self.add_annotation_to_file(
                    &mut annotated_line_infos,
                    ann.start.line,
                    ann.as_start(),
                );
                // 4 is the minimum vertical length of a multiline span when presented: two lines
                // of code and two lines of underline. This is not true for the special case where
                // the beginning doesn't have an underline, but the current logic seems to be
                // working correctly.
                let middle = min(ann.start.line + 4, ann.end.line);
                // We'll show up to 4 lines past the beginning of the multispan start.
                // We will *not* include the tail of lines that are only whitespace, a comment or
                // a bare delimiter.
                let filter = |s: &str| {
                    let s = s.trim();
                    // Consider comments as empty, but don't consider docstrings to be empty.
                    !(s.starts_with("//") && !(s.starts_with("///") || s.starts_with("//!")))
                        // Consider lines with nothing but whitespace, a single delimiter as empty.
                        && !["", "{", "}", "(", ")", "[", "]"].contains(&s)
                };
                let until = (ann.start.line..middle)
                    .rev()
                    .filter_map(|line| self.get_line(line).map(|s| (line + 1, s)))
                    .find(|(_, s)| filter(s))
                    .map_or(ann.start.line, |(line, _)| line);
                for line in ann.start.line + 1..until {
                    // Every `|` that joins the beginning of the span (`___^`) to the end (`|__^`).
                    self.add_annotation_to_file(&mut annotated_line_infos, line, ann.as_line());
                }
                let line_end = ann.end.line - 1;
                let end_is_empty = self.get_line(line_end).map_or(false, |s| !filter(s));
                if middle < line_end && !end_is_empty {
                    self.add_annotation_to_file(&mut annotated_line_infos, line_end, ann.as_line());
                }
            }
            self.add_annotation_to_file(&mut annotated_line_infos, end_ann.end.line, end_ann);
        }

        if fold {
            annotated_line_infos.retain(|l| !l.annotations.is_empty());
        }

        annotated_line_infos
            .iter_mut()
            .for_each(|l| l.annotations.sort_by(|a, b| a.start.cmp(&b.start)));

        annotated_line_infos
    }

    fn add_annotation_to_file(
        &self,
        annotated_line_infos: &mut Vec<AnnotatedLineInfo<'a>>,
        line_index: usize,
        line_ann: LineAnnotation<'a>,
    ) {
        if let Some(line_info) = annotated_line_infos
            .iter_mut()
            .find(|line_info| line_info.line_index == line_index)
        {
            line_info.annotations.push(line_ann);
        } else {
            let info = self
                .lines
                .iter()
                .find(|l| l.line_index == line_index)
                .unwrap();
            annotated_line_infos.push(AnnotatedLineInfo {
                line: info.line,
                line_index,
                annotations: vec![line_ann],
            });
            annotated_line_infos.sort_by_key(|l| l.line_index);
        }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct MultilineAnnotation<'a> {
    pub depth: usize,
    pub start: Loc,
    pub end: Loc,
    pub level: Level,
    pub label: Option<&'a str>,
    pub overlaps_exactly: bool,
}

impl<'a> MultilineAnnotation<'a> {
    pub(crate) fn increase_depth(&mut self) {
        self.depth += 1;
    }

    /// Compare two `MultilineAnnotation`s considering only the `Span` they cover.
    pub(crate) fn same_span(&self, other: &MultilineAnnotation<'_>) -> bool {
        self.start == other.start && self.end == other.end
    }

    pub(crate) fn as_start(&self) -> LineAnnotation<'a> {
        LineAnnotation {
            start: self.start,
            end: Loc {
                line: self.start.line,
                char: self.start.char + 1,
                display: self.start.display + 1,
                byte: self.start.byte + 1,
            },
            level: self.level,
            label: None,
            annotation_type: LineAnnotationType::MultilineStart(self.depth),
        }
    }

    pub(crate) fn as_end(&self) -> LineAnnotation<'a> {
        LineAnnotation {
            start: Loc {
                line: self.end.line,
                char: self.end.char.saturating_sub(1),
                display: self.end.display.saturating_sub(1),
                byte: self.end.byte.saturating_sub(1),
            },
            end: self.end,
            level: self.level,
            label: self.label,
            annotation_type: LineAnnotationType::MultilineEnd(self.depth),
        }
    }

    pub(crate) fn as_line(&self) -> LineAnnotation<'a> {
        LineAnnotation {
            start: Loc::default(),
            end: Loc::default(),
            level: self.level,
            label: None,
            annotation_type: LineAnnotationType::MultilineLine(self.depth),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LineInfo<'a> {
    pub(crate) line: &'a str,
    pub(crate) line_index: usize,
    pub(crate) start_byte: usize,
    pub(crate) end_byte: usize,
    end_line_size: usize,
}

#[derive(Debug)]
pub(crate) struct AnnotatedLineInfo<'a> {
    pub(crate) line: &'a str,
    pub(crate) line_index: usize,
    pub(crate) annotations: Vec<LineAnnotation<'a>>,
}

/// A source code location used for error reporting.
#[derive(Clone, Copy, Debug, Default, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct Loc {
    /// The (1-based) line number.
    pub(crate) line: usize,
    /// The (0-based) column offset.
    pub(crate) char: usize,
    /// The (0-based) column offset when displayed.
    pub(crate) display: usize,
    /// The (0-based) byte offset.
    pub(crate) byte: usize,
}

struct CursorLines<'a>(&'a str);

impl CursorLines<'_> {
    fn new(src: &str) -> CursorLines<'_> {
        CursorLines(src)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum EndLine {
    Eof,
    Lf,
    Crlf,
}

impl EndLine {
    /// The number of characters this line ending occupies in bytes.
    pub(crate) fn len(self) -> usize {
        match self {
            EndLine::Eof => 0,
            EndLine::Lf => 1,
            EndLine::Crlf => 2,
        }
    }
}

impl<'a> Iterator for CursorLines<'a> {
    type Item = (&'a str, EndLine);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            self.0
                .find('\n')
                .map(|x| {
                    let ret = if 0 < x {
                        if self.0.as_bytes()[x - 1] == b'\r' {
                            (&self.0[..x - 1], EndLine::Crlf)
                        } else {
                            (&self.0[..x], EndLine::Lf)
                        }
                    } else {
                        ("", EndLine::Lf)
                    };
                    self.0 = &self.0[x + 1..];
                    ret
                })
                .or_else(|| {
                    let ret = Some((self.0, EndLine::Eof));
                    self.0 = "";
                    ret
                })
        }
    }
}
