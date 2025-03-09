use crate::renderer::{char_width, is_different, num_overlap, LineAnnotation, LineAnnotationType};
use crate::{Annotation, AnnotationKind, Patch};
use std::cmp::{max, min};
use std::ops::Range;

#[derive(Debug)]
pub(crate) struct SourceMap<'a> {
    lines: Vec<LineInfo<'a>>,
    pub(crate) source: &'a str,
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

    pub(crate) fn span_to_snippet(&self, span: Range<usize>) -> Option<&str> {
        self.source.get(span)
    }

    pub(crate) fn span_to_lines(&self, span: Range<usize>) -> Vec<&LineInfo<'a>> {
        let mut lines = vec![];
        let start = span.start;
        let end = span.end;
        for line_info in &self.lines {
            if start >= line_info.end_byte {
                continue;
            }
            if end <= line_info.start_byte {
                break;
            }
            lines.push(line_info);
        }
        lines
    }

    pub(crate) fn annotated_lines(
        &self,
        annotations: Vec<Annotation<'a>>,
        fold: bool,
    ) -> (usize, Vec<AnnotatedLineInfo<'a>>) {
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
            kind,
            highlight_source,
        } in annotations
        {
            let (lo, mut hi) = self.span_to_locations(range.clone());

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
                    kind,
                    label,
                    annotation_type: LineAnnotationType::Singleline,
                    highlight_source,
                };
                self.add_annotation_to_file(&mut annotated_line_infos, lo.line, line_ann);
            } else {
                multiline_annotations.push(MultilineAnnotation {
                    depth: 1,
                    start: lo,
                    end: hi,
                    kind,
                    label,
                    overlaps_exactly: false,
                    highlight_source,
                });
            }
        }

        let mut primary_spans = vec![];

        // Find overlapping multiline annotations, put them at different depths
        multiline_annotations
            .sort_by_key(|ml| (ml.start.line, usize::MAX - ml.end.line, ml.start.byte));
        for ann in multiline_annotations.clone() {
            if ann.kind.is_primary() {
                primary_spans.push((ann.start, ann.end));
            }
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
                    if primary_spans
                        .iter()
                        .any(|(s, e)| a.start == *s && a.end == *e)
                    {
                        a.kind = AnnotationKind::Primary;
                    }
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

        (max_depth, annotated_line_infos)
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

    pub(crate) fn splice_lines<'b>(
        &'b self,
        mut patches: Vec<Patch<'b>>,
    ) -> Vec<(String, Vec<Patch<'b>>, Vec<Vec<SubstitutionHighlight>>)> {
        fn push_trailing(
            buf: &mut String,
            line_opt: Option<&str>,
            lo: &Loc,
            hi_opt: Option<&Loc>,
        ) -> usize {
            let mut line_count = 0;
            // Convert CharPos to Usize, as CharPose is character offset
            // Extract low index and high index
            let (lo, hi_opt) = (lo.char, hi_opt.map(|hi| hi.char));
            if let Some(line) = line_opt {
                if let Some(lo) = line.char_indices().map(|(i, _)| i).nth(lo) {
                    // Get high index while account for rare unicode and emoji with char_indices
                    let hi_opt = hi_opt.and_then(|hi| line.char_indices().map(|(i, _)| i).nth(hi));
                    match hi_opt {
                        // If high index exist, take string from low to high index
                        Some(hi) if hi > lo => {
                            // count how many '\n' exist
                            line_count = line[lo..hi].matches('\n').count();
                            buf.push_str(&line[lo..hi]);
                        }
                        Some(_) => (),
                        // If high index absence, take string from low index till end string.len
                        None => {
                            // count how many '\n' exist
                            line_count = line[lo..].matches('\n').count();
                            buf.push_str(&line[lo..]);
                        }
                    }
                }
                // If high index is None
                if hi_opt.is_none() {
                    buf.push('\n');
                }
            }
            line_count
        }
        // Assumption: all spans are in the same file, and all spans
        // are disjoint. Sort in ascending order.
        patches.sort_by_key(|p| p.range.start);

        // Find the bounding span.
        let Some(lo) = patches.iter().map(|p| p.range.start).min() else {
            return Vec::new();
        };
        let Some(hi) = patches.iter().map(|p| p.range.end).max() else {
            return Vec::new();
        };

        let lines = self.span_to_lines(lo..hi);

        let mut highlights = vec![];
        // To build up the result, we do this for each span:
        // - push the line segment trailing the previous span
        //   (at the beginning a "phantom" span pointing at the start of the line)
        // - push lines between the previous and current span (if any)
        // - if the previous and current span are not on the same line
        //   push the line segment leading up to the current span
        // - splice in the span substitution
        //
        // Finally push the trailing line segment of the last span
        let (mut prev_hi, _) = self.span_to_locations(lo..hi);
        prev_hi.char = 0;
        let mut prev_line = lines.first().map(|line| line.line);
        let mut buf = String::new();

        let mut line_highlight = vec![];
        // We need to keep track of the difference between the existing code and the added
        // or deleted code in order to point at the correct column *after* substitution.
        let mut acc = 0;
        for part in &mut patches {
            // If this is a replacement of, e.g. `"a"` into `"ab"`, adjust the
            // suggestion and snippet to look as if we just suggested to add
            // `"b"`, which is typically much easier for the user to understand.
            part.trim_trivial_replacements(self);
            let (cur_lo, cur_hi) = self.span_to_locations(part.range.clone());
            if prev_hi.line == cur_lo.line {
                let mut count = push_trailing(&mut buf, prev_line, &prev_hi, Some(&cur_lo));
                while count > 0 {
                    highlights.push(std::mem::take(&mut line_highlight));
                    acc = 0;
                    count -= 1;
                }
            } else {
                acc = 0;
                highlights.push(std::mem::take(&mut line_highlight));
                let mut count = push_trailing(&mut buf, prev_line, &prev_hi, None);
                while count > 0 {
                    highlights.push(std::mem::take(&mut line_highlight));
                    count -= 1;
                }
                // push lines between the previous and current span (if any)
                for idx in prev_hi.line + 1..(cur_lo.line) {
                    if let Some(line) = self.get_line(idx) {
                        buf.push_str(line.as_ref());
                        buf.push('\n');
                        highlights.push(std::mem::take(&mut line_highlight));
                    }
                }
                if let Some(cur_line) = self.get_line(cur_lo.line) {
                    let end = match cur_line.char_indices().nth(cur_lo.char) {
                        Some((i, _)) => i,
                        None => cur_line.len(),
                    };
                    buf.push_str(&cur_line[..end]);
                }
            }
            // Add a whole line highlight per line in the snippet.
            let len: isize = part
                .replacement
                .split('\n')
                .next()
                .unwrap_or(part.replacement)
                .chars()
                .map(|c| match c {
                    '\t' => 4,
                    _ => 1,
                })
                .sum();
            if !is_different(self, part.replacement, part.range.clone()) {
                // Account for cases where we are suggesting the same code that's already
                // there. This shouldn't happen often, but in some cases for multipart
                // suggestions it's much easier to handle it here than in the origin.
            } else {
                line_highlight.push(SubstitutionHighlight {
                    start: (cur_lo.char as isize + acc) as usize,
                    end: (cur_lo.char as isize + acc + len) as usize,
                });
            }
            buf.push_str(part.replacement);
            // Account for the difference between the width of the current code and the
            // snippet being suggested, so that the *later* suggestions are correctly
            // aligned on the screen. Note that cur_hi and cur_lo can be on different
            // lines, so cur_hi.col can be smaller than cur_lo.col
            acc += len - (cur_hi.char as isize - cur_lo.char as isize);
            prev_hi = cur_hi;
            prev_line = self.get_line(prev_hi.line);
            for line in part.replacement.split('\n').skip(1) {
                acc = 0;
                highlights.push(std::mem::take(&mut line_highlight));
                let end: usize = line
                    .chars()
                    .map(|c| match c {
                        '\t' => 4,
                        _ => 1,
                    })
                    .sum();
                line_highlight.push(SubstitutionHighlight { start: 0, end });
            }
        }
        highlights.push(std::mem::take(&mut line_highlight));
        // if the replacement already ends with a newline, don't print the next line
        if !buf.ends_with('\n') {
            push_trailing(&mut buf, prev_line, &prev_hi, None);
        }
        // remove trailing newlines
        while buf.ends_with('\n') {
            buf.pop();
        }
        if highlights.iter().all(|parts| parts.is_empty()) {
            Vec::new()
        } else {
            vec![(buf, patches, highlights)]
        }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct MultilineAnnotation<'a> {
    pub depth: usize,
    pub start: Loc,
    pub end: Loc,
    pub kind: AnnotationKind,
    pub label: Option<&'a str>,
    pub overlaps_exactly: bool,
    pub highlight_source: bool,
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
            kind: self.kind,
            label: None,
            annotation_type: LineAnnotationType::MultilineStart(self.depth),
            highlight_source: self.highlight_source,
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
            kind: self.kind,
            label: self.label,
            annotation_type: LineAnnotationType::MultilineEnd(self.depth),
            highlight_source: self.highlight_source,
        }
    }

    pub(crate) fn as_line(&self) -> LineAnnotation<'a> {
        LineAnnotation {
            start: Loc::default(),
            end: Loc::default(),
            kind: self.kind,
            label: None,
            annotation_type: LineAnnotationType::MultilineLine(self.depth),
            highlight_source: self.highlight_source,
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

/// Used to translate between `Span`s and byte positions within a single output line in highlighted
/// code of structured suggestions.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SubstitutionHighlight {
    pub(crate) start: usize,
    pub(crate) end: usize,
}
