//! `display_list` module stores the output model for the snippet.
//!
//! `DisplayList` is a central structure in the crate, which contains
//! the structured list of lines to be displayed.
//!
//! It is made of two types of lines: `Source` and `Raw`. All `Source` lines
//! are structured using four columns:
//!
//! ```text
//!  /------------ (1) Line number column.
//!  |  /--------- (2) Line number column delimiter.
//!  |  | /------- (3) Inline marks column.
//!  |  | |   /--- (4) Content column with the source and annotations for slices.
//!  |  | |   |
//! =============================================================================
//! error[E0308]: mismatched types
//!    --> src/format.rs:51:5
//!     |
//! 151 | /   fn test() -> String {
//! 152 | |       return "test";
//! 153 | |   }
//!     | |___^ error: expected `String`, for `&str`.
//! ```
//!
//! The first two lines of the example above are `Raw` lines, while the rest
//! are `Source` lines.
//!
//! `DisplayList` does not store column alignment information, and those are
//! only calculated by the implementation of `std::fmt::Display` using information such as
//! styling.
//!
//! The above snippet has been built out of the following structure:

mod constants;
mod cursor_line;
mod display_annotations;
mod display_header;
mod display_line;
mod display_list;
mod display_mark;
mod display_set;
mod display_text;
mod end_line;

use crate::snippet;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

use crate::renderer::styled_buffer::StyledBuffer;
use crate::renderer::{stylesheet::Stylesheet, Margin, DEFAULT_TERM_WIDTH};

use constants::*;
use cursor_line::CursorLines;
use display_annotations::{
    get_annotation_style, Annotation, DisplayAnnotationPart, DisplayAnnotationType,
    DisplaySourceAnnotation,
};
use display_header::DisplayHeaderType;
use display_line::{DisplayLine, DisplayRawLine, DisplaySourceLine};
use display_mark::{DisplayMark, DisplayMarkType};
use display_set::DisplaySet;
use display_text::{DisplayTextFragment, DisplayTextStyle};

pub(crate) use display_list::DisplayList;

pub(crate) fn format_message(
    message: snippet::Message<'_>,
    term_width: usize,
    anonymized_line_numbers: bool,
    primary: bool,
) -> Vec<DisplaySet<'_>> {
    let snippet::Message {
        level,
        id,
        title,
        footer,
        snippets,
    } = message;

    let mut sets = vec![];
    let body = if !snippets.is_empty() || primary {
        vec![format_title(level, id, title)]
    } else {
        format_footer(level, id, title)
    };

    for (idx, snippet) in snippets.into_iter().enumerate() {
        let snippet = fold_prefix_suffix(snippet);
        sets.push(format_snippet(
            snippet,
            idx == 0,
            term_width,
            anonymized_line_numbers,
        ));
    }

    if let Some(first) = sets.first_mut() {
        for line in body {
            first.display_lines.insert(0, line);
        }
    } else {
        sets.push(DisplaySet {
            display_lines: body,
            margin: Margin::new(0, 0, 0, 0, DEFAULT_TERM_WIDTH, 0),
        });
    }

    for annotation in footer {
        sets.extend(format_message(
            annotation,
            term_width,
            anonymized_line_numbers,
            false,
        ));
    }

    sets
}

fn format_title<'a>(level: crate::Level, id: Option<&'a str>, label: &'a str) -> DisplayLine<'a> {
    DisplayLine::Raw(DisplayRawLine::Annotation {
        annotation: Annotation {
            annotation_type: DisplayAnnotationType::from(level),
            id,
            label: format_label(Some(label), Some(DisplayTextStyle::Emphasis)),
        },
        source_aligned: false,
        continuation: false,
    })
}

fn format_footer<'a>(
    level: crate::Level,
    id: Option<&'a str>,
    label: &'a str,
) -> Vec<DisplayLine<'a>> {
    let mut result = vec![];
    for (i, line) in label.lines().enumerate() {
        result.push(DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::from(level),
                id,
                label: format_label(Some(line), None),
            },
            source_aligned: true,
            continuation: i != 0,
        }));
    }
    result
}

fn format_label(
    label: Option<&str>,
    style: Option<DisplayTextStyle>,
) -> Vec<DisplayTextFragment<'_>> {
    let mut result = vec![];
    if let Some(label) = label {
        let element_style = style.unwrap_or(DisplayTextStyle::Regular);
        result.push(DisplayTextFragment {
            content: label,
            style: element_style,
        });
    }
    result
}

fn format_snippet(
    snippet: snippet::Snippet<'_>,
    is_first: bool,
    term_width: usize,
    anonymized_line_numbers: bool,
) -> DisplaySet<'_> {
    let main_range = snippet.annotations.first().map(|x| x.range.start);
    let origin = snippet.origin;
    let need_empty_header = origin.is_some() || is_first;
    let mut body = format_body(
        snippet,
        need_empty_header,
        term_width,
        anonymized_line_numbers,
    );
    let header = format_header(origin, main_range, &body.display_lines, is_first);

    if let Some(header) = header {
        body.display_lines.insert(0, header);
    }

    body
}

#[inline]
// TODO: option_zip
fn zip_opt<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    a.and_then(|a| b.map(|b| (a, b)))
}

fn format_header<'a>(
    origin: Option<&'a str>,
    main_range: Option<usize>,
    body: &[DisplayLine<'_>],
    is_first: bool,
) -> Option<DisplayLine<'a>> {
    let display_header = if is_first {
        DisplayHeaderType::Initial
    } else {
        DisplayHeaderType::Continuation
    };

    if let Some((main_range, path)) = zip_opt(main_range, origin) {
        let mut col = 1;
        let mut line_offset = 1;

        for item in body {
            if let DisplayLine::Source {
                line:
                    DisplaySourceLine::Content {
                        text,
                        range,
                        end_line,
                    },
                lineno,
                ..
            } = item
            {
                if main_range >= range.0 && main_range < range.1 + max(*end_line as usize, 1) {
                    let char_column = text[0..(main_range - range.0).min(text.len())]
                        .chars()
                        .count();
                    col = char_column + 1;
                    line_offset = lineno.unwrap_or(1);
                    break;
                }
            }
        }

        return Some(DisplayLine::Raw(DisplayRawLine::Origin {
            path,
            pos: Some((line_offset, col)),
            header_type: display_header,
        }));
    }

    if let Some(path) = origin {
        return Some(DisplayLine::Raw(DisplayRawLine::Origin {
            path,
            pos: None,
            header_type: display_header,
        }));
    }

    None
}

fn fold_prefix_suffix(mut snippet: snippet::Snippet<'_>) -> snippet::Snippet<'_> {
    if !snippet.fold {
        return snippet;
    }

    let ann_start = snippet
        .annotations
        .iter()
        .map(|ann| ann.range.start)
        .min()
        .unwrap_or(0);
    if let Some(before_new_start) = snippet.source[0..ann_start].rfind('\n') {
        let new_start = before_new_start + 1;

        let line_offset = newline_count(&snippet.source[..new_start]);
        snippet.line_start += line_offset;

        snippet.source = &snippet.source[new_start..];

        for ann in &mut snippet.annotations {
            let range_start = ann.range.start - new_start;
            let range_end = ann.range.end - new_start;
            ann.range = range_start..range_end;
        }
    }

    let ann_end = snippet
        .annotations
        .iter()
        .map(|ann| ann.range.end)
        .max()
        .unwrap_or(snippet.source.len());
    if let Some(end_offset) = snippet.source[ann_end..].find('\n') {
        let new_end = ann_end + end_offset;
        snippet.source = &snippet.source[..new_end];
    }

    snippet
}

fn newline_count(body: &str) -> usize {
    #[cfg(feature = "simd")]
    {
        memchr::memchr_iter(b'\n', body.as_bytes()).count()
    }
    #[cfg(not(feature = "simd"))]
    {
        body.lines().count()
    }
}

fn fold_body(body: Vec<DisplayLine<'_>>) -> Vec<DisplayLine<'_>> {
    const INNER_CONTEXT: usize = 1;
    const INNER_UNFOLD_SIZE: usize = INNER_CONTEXT * 2 + 1;

    let mut lines = vec![];
    let mut unhighlighted_lines = vec![];
    for line in body {
        match &line {
            DisplayLine::Source { annotations, .. } => {
                if annotations.is_empty() {
                    unhighlighted_lines.push(line);
                } else {
                    if lines.is_empty() {
                        // Ignore leading unhighlighted lines
                        unhighlighted_lines.clear();
                    }
                    match unhighlighted_lines.len() {
                        0 => {}
                        n if n <= INNER_UNFOLD_SIZE => {
                            // Rather than render `...`, don't fold
                            lines.append(&mut unhighlighted_lines);
                        }
                        _ => {
                            lines.extend(unhighlighted_lines.drain(..INNER_CONTEXT));
                            let inline_marks = lines
                                .last()
                                .and_then(|line| {
                                    if let DisplayLine::Source {
                                        ref inline_marks, ..
                                    } = line
                                    {
                                        let inline_marks = inline_marks.clone();
                                        Some(inline_marks)
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or_default();
                            lines.push(DisplayLine::Fold {
                                inline_marks: inline_marks.clone(),
                            });
                            unhighlighted_lines
                                .drain(..unhighlighted_lines.len().saturating_sub(INNER_CONTEXT));
                            lines.append(&mut unhighlighted_lines);
                        }
                    }
                    lines.push(line);
                }
            }
            _ => {
                unhighlighted_lines.push(line);
            }
        }
    }

    lines
}

fn format_body(
    snippet: snippet::Snippet<'_>,
    need_empty_header: bool,
    term_width: usize,
    anonymized_line_numbers: bool,
) -> DisplaySet<'_> {
    let source_len = snippet.source.len();
    if let Some(bigger) = snippet.annotations.iter().find_map(|x| {
        // Allow highlighting one past the last character in the source.
        if source_len + 1 < x.range.end {
            Some(&x.range)
        } else {
            None
        }
    }) {
        panic!("SourceAnnotation range `{bigger:?}` is beyond the end of buffer `{source_len}`")
    }

    let mut body = vec![];
    let mut current_line = snippet.line_start;
    let mut current_index = 0;

    let mut whitespace_margin = usize::MAX;
    let mut span_left_margin = usize::MAX;
    let mut span_right_margin = 0;
    let mut label_right_margin = 0;
    let mut max_line_len = 0;

    let mut depth_map: HashMap<usize, usize> = HashMap::new();
    let mut current_depth = 0;
    let mut annotations = snippet.annotations;
    let ranges = annotations
        .iter()
        .map(|a| a.range.clone())
        .collect::<Vec<_>>();
    // We want to merge multiline annotations that have the same range into one
    // multiline annotation to save space. This is done by making any duplicate
    // multiline annotations into a single-line annotation pointing at the end
    // of the range.
    //
    // 3 |       X0 Y0 Z0
    //   |  _____^
    //   | | ____|
    //   | || ___|
    //   | |||
    // 4 | |||   X1 Y1 Z1
    // 5 | |||   X2 Y2 Z2
    //   | |||    ^
    //   | |||____|
    //   |  ||____`X` is a good letter
    //   |   |____`Y` is a good letter too
    //   |        `Z` label
    // Should be
    // error: foo
    //  --> test.rs:3:3
    //   |
    // 3 | /   X0 Y0 Z0
    // 4 | |   X1 Y1 Z1
    // 5 | |   X2 Y2 Z2
    //   | |    ^
    //   | |____|
    //   |      `X` is a good letter
    //   |      `Y` is a good letter too
    //   |      `Z` label
    //   |
    ranges.iter().enumerate().for_each(|(r_idx, range)| {
        annotations
            .iter_mut()
            .enumerate()
            .skip(r_idx + 1)
            .for_each(|(ann_idx, ann)| {
                // Skip if the annotation's index matches the range index
                if ann_idx != r_idx
                    // We only want to merge multiline annotations
                    && snippet.source[ann.range.clone()].lines().count() > 1
                    // We only want to merge annotations that have the same range
                    && ann.range.start == range.start
                    && ann.range.end == range.end
                {
                    ann.range.start = ann.range.end.saturating_sub(1);
                }
            });
    });
    annotations.sort_by_key(|a| a.range.start);
    let mut annotations = annotations.into_iter().enumerate().collect::<Vec<_>>();

    for (idx, (line, end_line)) in CursorLines::new(snippet.source).enumerate() {
        let line_length: usize = line.len();
        let line_range = (current_index, current_index + line_length);
        let end_line_size = end_line.len();
        body.push(DisplayLine::Source {
            lineno: Some(current_line),
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: line,
                range: line_range,
                end_line,
            },
            annotations: vec![],
        });

        let leading_whitespace = line
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| {
                match c {
                    // Tabs are displayed as 4 spaces
                    '\t' => 4,
                    _ => 1,
                }
            })
            .sum();
        if line.chars().any(|c| !c.is_whitespace()) {
            whitespace_margin = min(whitespace_margin, leading_whitespace);
        }
        max_line_len = max(max_line_len, line_length);

        let line_start_index = line_range.0;
        let line_end_index = line_range.1;
        current_line += 1;
        current_index += line_length + end_line_size;

        // It would be nice to use filter_drain here once it's stable.
        annotations.retain(|(key, annotation)| {
            let body_idx = idx;
            let annotation_type = match annotation.level {
                snippet::Level::Error => DisplayAnnotationType::None,
                snippet::Level::Warning => DisplayAnnotationType::None,
                _ => DisplayAnnotationType::from(annotation.level),
            };
            let label_right = annotation.label.map_or(0, |label| label.len() + 1);
            match annotation.range {
                // This handles if the annotation is on the next line. We add
                // the `end_line_size` to account for annotating the line end.
                Range { start, .. } if start > line_end_index + end_line_size => true,
                // This handles the case where an annotation is contained
                // within the current line including any line-end characters.
                Range { start, end }
                    if start >= line_start_index
                        // We add at least one to `line_end_index` to allow
                        // highlighting the end of a file
                        && end <= line_end_index + max(end_line_size, 1) =>
                {
                    if let DisplayLine::Source {
                        ref mut annotations,
                        ..
                    } = body[body_idx]
                    {
                        let annotation_start_col = line
                            [0..(start - line_start_index).min(line_length)]
                            .chars()
                            .map(|c| unicode_width::UnicodeWidthChar::width(c).unwrap_or(0))
                            .sum::<usize>();
                        let mut annotation_end_col = line
                            [0..(end - line_start_index).min(line_length)]
                            .chars()
                            .map(|c| unicode_width::UnicodeWidthChar::width(c).unwrap_or(0))
                            .sum::<usize>();
                        if annotation_start_col == annotation_end_col {
                            // At least highlight something
                            annotation_end_col += 1;
                        }

                        span_left_margin = min(span_left_margin, annotation_start_col);
                        span_right_margin = max(span_right_margin, annotation_end_col);
                        label_right_margin =
                            max(label_right_margin, annotation_end_col + label_right);

                        let range = (annotation_start_col, annotation_end_col);
                        annotations.push(DisplaySourceAnnotation {
                            annotation: Annotation {
                                annotation_type,
                                id: None,
                                label: format_label(annotation.label, None),
                            },
                            range,
                            annotation_type: DisplayAnnotationType::from(annotation.level),
                            annotation_part: DisplayAnnotationPart::Standalone,
                        });
                    }
                    false
                }
                // This handles the case where a multiline annotation starts
                // somewhere on the current line, including any line-end chars
                Range { start, end }
                    if start >= line_start_index
                        // The annotation can start on a line ending
                        && start <= line_end_index + end_line_size.saturating_sub(1)
                        && end > line_end_index =>
                {
                    if let DisplayLine::Source {
                        ref mut annotations,
                        ..
                    } = body[body_idx]
                    {
                        let annotation_start_col = line
                            [0..(start - line_start_index).min(line_length)]
                            .chars()
                            .map(|c| unicode_width::UnicodeWidthChar::width(c).unwrap_or(0))
                            .sum::<usize>();
                        let annotation_end_col = annotation_start_col + 1;

                        span_left_margin = min(span_left_margin, annotation_start_col);
                        span_right_margin = max(span_right_margin, annotation_end_col);
                        label_right_margin =
                            max(label_right_margin, annotation_end_col + label_right);

                        let range = (annotation_start_col, annotation_end_col);
                        annotations.push(DisplaySourceAnnotation {
                            annotation: Annotation {
                                annotation_type,
                                id: None,
                                label: vec![],
                            },
                            range,
                            annotation_type: DisplayAnnotationType::from(annotation.level),
                            annotation_part: DisplayAnnotationPart::MultilineStart(current_depth),
                        });
                        depth_map.insert(*key, current_depth);
                        current_depth += 1;
                    }
                    true
                }
                // This handles the case where a multiline annotation starts
                // somewhere before this line and ends after it as well
                Range { start, end }
                    if start < line_start_index && end > line_end_index + max(end_line_size, 1) =>
                {
                    if let DisplayLine::Source {
                        ref mut inline_marks,
                        ..
                    } = body[body_idx]
                    {
                        let depth = depth_map.get(key).cloned().unwrap_or_default();
                        inline_marks.push(DisplayMark {
                            mark_type: DisplayMarkType::AnnotationThrough(depth),
                            annotation_type: DisplayAnnotationType::from(annotation.level),
                        });
                    }
                    true
                }
                // This handles the case where a multiline annotation ends
                // somewhere on the current line, including any line-end chars
                Range { start, end }
                    if start < line_start_index
                        && end >= line_start_index
                        // We add at least one to `line_end_index` to allow
                        // highlighting the end of a file
                        && end <= line_end_index + max(end_line_size, 1) =>
                {
                    if let DisplayLine::Source {
                        ref mut annotations,
                        ..
                    } = body[body_idx]
                    {
                        let end_mark = line[0..(end - line_start_index).min(line_length)]
                            .chars()
                            .map(|c| unicode_width::UnicodeWidthChar::width(c).unwrap_or(0))
                            .sum::<usize>()
                            .saturating_sub(1);
                        // If the annotation ends on a line-end character, we
                        // need to annotate one past the end of the line
                        let (end_mark, end_plus_one) = if end > line_end_index
                            // Special case for highlighting the end of a file
                            || (end == line_end_index + 1 && end_line_size == 0)
                        {
                            (end_mark + 1, end_mark + 2)
                        } else {
                            (end_mark, end_mark + 1)
                        };

                        span_left_margin = min(span_left_margin, end_mark);
                        span_right_margin = max(span_right_margin, end_plus_one);
                        label_right_margin = max(label_right_margin, end_plus_one + label_right);

                        let range = (end_mark, end_plus_one);
                        let depth = depth_map.remove(key).unwrap_or(0);
                        annotations.push(DisplaySourceAnnotation {
                            annotation: Annotation {
                                annotation_type,
                                id: None,
                                label: format_label(annotation.label, None),
                            },
                            range,
                            annotation_type: DisplayAnnotationType::from(annotation.level),
                            annotation_part: DisplayAnnotationPart::MultilineEnd(depth),
                        });
                    }
                    false
                }
                _ => true,
            }
        });
        // Reset the depth counter, but only after we've processed all
        // annotations for a given line.
        let max = depth_map.len();
        if current_depth > max {
            current_depth = max;
        }
    }

    if snippet.fold {
        body = fold_body(body);
    }

    if need_empty_header {
        body.insert(
            0,
            DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: DisplaySourceLine::Empty,
                annotations: vec![],
            },
        );
    }

    let max_line_num_len = if anonymized_line_numbers {
        ANONYMIZED_LINE_NUM.len()
    } else {
        current_line.to_string().len()
    };

    let width_offset = 3 + max_line_num_len;

    if span_left_margin == usize::MAX {
        span_left_margin = 0;
    }

    let margin = Margin::new(
        whitespace_margin,
        span_left_margin,
        span_right_margin,
        label_right_margin,
        term_width.saturating_sub(width_offset),
        max_line_len,
    );

    DisplaySet {
        display_lines: body,
        margin,
    }
}

// We replace some characters so the CLI output is always consistent and underlines aligned.
const OUTPUT_REPLACEMENTS: &[(char, &str)] = &[
    ('\t', "    "),   // We do our own tab replacement
    ('\u{200D}', ""), // Replace ZWJ with nothing for consistent terminal output of grapheme clusters.
    ('\u{202A}', ""), // The following unicode text flow control characters are inconsistently
    ('\u{202B}', ""), // supported across CLIs and can cause confusion due to the bytes on disk
    ('\u{202D}', ""), // not corresponding to the visible source code, so we replace them always.
    ('\u{202E}', ""),
    ('\u{2066}', ""),
    ('\u{2067}', ""),
    ('\u{2068}', ""),
    ('\u{202C}', ""),
    ('\u{2069}', ""),
];

fn normalize_whitespace(str: &str) -> String {
    let mut s = str.to_owned();
    for (c, replacement) in OUTPUT_REPLACEMENTS {
        s = s.replace(*c, replacement);
    }
    s
}

fn overlaps(
    a1: &DisplaySourceAnnotation<'_>,
    a2: &DisplaySourceAnnotation<'_>,
    padding: usize,
) -> bool {
    (a2.range.0..a2.range.1).contains(&a1.range.0)
        || (a1.range.0..a1.range.1 + padding).contains(&a2.range.0)
}

fn format_inline_marks(
    line: usize,
    inline_marks: &[DisplayMark],
    lineno_width: usize,
    stylesheet: &Stylesheet,
    buf: &mut StyledBuffer,
) -> fmt::Result {
    for mark in inline_marks.iter() {
        let annotation_style = get_annotation_style(&mark.annotation_type, stylesheet);
        match mark.mark_type {
            DisplayMarkType::AnnotationThrough(depth) => {
                buf.putc(line, 3 + lineno_width + depth, '|', *annotation_style);
            }
        };
    }
    Ok(())
}
