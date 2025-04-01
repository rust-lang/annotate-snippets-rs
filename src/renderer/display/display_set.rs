use core::fmt;
use std::cmp::{max, Reverse};

use anstyle::Style;

use crate::renderer::{
    display::display_annotations::DisplayAnnotationPart, margin::Margin,
    styled_buffer::StyledBuffer, stylesheet::Stylesheet,
};

use super::{
    constants::ANONYMIZED_LINE_NUM,
    display_annotations::{
        annotation_type_len, annotation_type_str, get_annotation_style, is_annotation_empty,
        Annotation, DisplayAnnotationType,
    },
    display_header::DisplayHeaderType,
    display_line::{DisplayRawLine, DisplaySourceLine},
    display_mark::DisplayMarkType,
    display_text::{DisplayTextFragment, DisplayTextStyle},
    format_inline_marks, normalize_whitespace, overlaps, DisplayLine,
};

#[derive(Debug, PartialEq)]
pub(crate) struct DisplaySet<'a> {
    pub(crate) display_lines: Vec<DisplayLine<'a>>,
    pub(crate) margin: Margin,
}

impl DisplaySet<'_> {
    fn format_label(
        &self,
        line_offset: usize,
        label: &[DisplayTextFragment<'_>],
        stylesheet: &Stylesheet,
        buffer: &mut StyledBuffer,
    ) -> fmt::Result {
        for fragment in label {
            let style = match fragment.style {
                DisplayTextStyle::Regular => stylesheet.none(),
                DisplayTextStyle::Emphasis => stylesheet.emphasis(),
            };
            buffer.append(line_offset, fragment.content, *style);
        }
        Ok(())
    }
    fn format_annotation(
        &self,
        line_offset: usize,
        annotation: &Annotation<'_>,
        continuation: bool,
        stylesheet: &Stylesheet,
        buffer: &mut StyledBuffer,
    ) -> fmt::Result {
        let color = get_annotation_style(&annotation.annotation_type, stylesheet);
        let formatted_len = if let Some(id) = &annotation.id {
            2 + id.len() + annotation_type_len(&annotation.annotation_type)
        } else {
            annotation_type_len(&annotation.annotation_type)
        };

        if continuation {
            for _ in 0..formatted_len + 2 {
                buffer.append(line_offset, " ", Style::new());
            }
            return self.format_label(line_offset, &annotation.label, stylesheet, buffer);
        }
        if formatted_len == 0 {
            self.format_label(line_offset, &annotation.label, stylesheet, buffer)
        } else {
            let id = match &annotation.id {
                Some(id) => format!("[{id}]"),
                None => String::new(),
            };
            buffer.append(
                line_offset,
                &format!("{}{}", annotation_type_str(&annotation.annotation_type), id),
                *color,
            );

            if !is_annotation_empty(annotation) {
                buffer.append(line_offset, ": ", stylesheet.none);
                self.format_label(line_offset, &annotation.label, stylesheet, buffer)?;
            }
            Ok(())
        }
    }

    #[inline]
    fn format_raw_line(
        &self,
        line_offset: usize,
        line: &DisplayRawLine<'_>,
        lineno_width: usize,
        stylesheet: &Stylesheet,
        buffer: &mut StyledBuffer,
    ) -> fmt::Result {
        match line {
            DisplayRawLine::Origin {
                path,
                pos,
                header_type,
            } => {
                let header_sigil = match header_type {
                    DisplayHeaderType::Initial => "-->",
                    DisplayHeaderType::Continuation => ":::",
                };
                let lineno_color = stylesheet.line_no();
                buffer.puts(line_offset, lineno_width, header_sigil, *lineno_color);
                buffer.puts(line_offset, lineno_width + 4, path, stylesheet.none);
                if let Some((col, row)) = pos {
                    buffer.append(line_offset, ":", stylesheet.none);
                    buffer.append(line_offset, col.to_string().as_str(), stylesheet.none);
                    buffer.append(line_offset, ":", stylesheet.none);
                    buffer.append(line_offset, row.to_string().as_str(), stylesheet.none);
                }
                Ok(())
            }
            DisplayRawLine::Annotation {
                annotation,
                source_aligned,
                continuation,
            } => {
                if *source_aligned {
                    if *continuation {
                        for _ in 0..lineno_width + 3 {
                            buffer.append(line_offset, " ", stylesheet.none);
                        }
                    } else {
                        let lineno_color = stylesheet.line_no();
                        for _ in 0..lineno_width + 1 {
                            buffer.append(line_offset, " ", stylesheet.none);
                        }
                        buffer.append(line_offset, "=", *lineno_color);
                        buffer.append(line_offset, " ", *lineno_color);
                    }
                }
                self.format_annotation(line_offset, annotation, *continuation, stylesheet, buffer)
            }
        }
    }

    // Adapted from https://github.com/rust-lang/rust/blob/d371d17496f2ce3a56da76aa083f4ef157572c20/compiler/rustc_errors/src/emitter.rs#L706-L1211
    #[inline]
    pub(crate) fn format_line(
        &self,
        dl: &DisplayLine<'_>,
        lineno_width: usize,
        multiline_depth: usize,
        stylesheet: &Stylesheet,
        anonymized_line_numbers: bool,
        buffer: &mut StyledBuffer,
    ) -> fmt::Result {
        let line_offset = buffer.num_lines();
        match dl {
            DisplayLine::Source {
                lineno,
                inline_marks,
                line,
                annotations,
            } => {
                let lineno_color = stylesheet.line_no();
                if anonymized_line_numbers && lineno.is_some() {
                    let num = format!("{ANONYMIZED_LINE_NUM:>lineno_width$} |");
                    buffer.puts(line_offset, 0, &num, *lineno_color);
                } else {
                    match lineno {
                        Some(n) => {
                            let num = format!("{n:>lineno_width$} |");
                            buffer.puts(line_offset, 0, &num, *lineno_color);
                        }
                        None => {
                            buffer.putc(line_offset, lineno_width + 1, '|', *lineno_color);
                        }
                    };
                }
                if let DisplaySourceLine::Content { text, .. } = line {
                    // The width of the line number, a space, pipe, and a space
                    // `123 | ` is `lineno_width + 3`.
                    let width_offset = lineno_width + 3;
                    let code_offset = if multiline_depth == 0 {
                        width_offset
                    } else {
                        width_offset + multiline_depth + 1
                    };

                    // Add any inline marks to the code line
                    if !inline_marks.is_empty() || 0 < multiline_depth {
                        format_inline_marks(
                            line_offset,
                            inline_marks,
                            lineno_width,
                            stylesheet,
                            buffer,
                        )?;
                    }

                    let text = normalize_whitespace(text);
                    let line_len = text.len();
                    let left = self.margin.left(line_len);
                    let right = self.margin.right(line_len);

                    // On long lines, we strip the source line, accounting for unicode.
                    let mut taken = 0;
                    let code: String = text
                        .chars()
                        .skip(left)
                        .take_while(|ch| {
                            // Make sure that the trimming on the right will fall within the terminal width.
                            // FIXME: `unicode_width` sometimes disagrees with terminals on how wide a `char`
                            // is. For now, just accept that sometimes the code line will be longer than
                            // desired.
                            let next = unicode_width::UnicodeWidthChar::width(*ch).unwrap_or(1);
                            if taken + next > right - left {
                                return false;
                            }
                            taken += next;
                            true
                        })
                        .collect();
                    buffer.puts(line_offset, code_offset, &code, Style::new());
                    if self.margin.was_cut_left() {
                        // We have stripped some code/whitespace from the beginning, make it clear.
                        buffer.puts(line_offset, code_offset, "...", *lineno_color);
                    }
                    if self.margin.was_cut_right(line_len) {
                        buffer.puts(line_offset, code_offset + taken - 3, "...", *lineno_color);
                    }

                    let left: usize = text
                        .chars()
                        .take(left)
                        .map(|ch| unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1))
                        .sum();

                    let mut annotations = annotations.clone();
                    annotations.sort_by_key(|a| Reverse(a.range.0));

                    let mut annotations_positions = vec![];
                    let mut line_len: usize = 0;
                    let mut p = 0;
                    for (i, annotation) in annotations.iter().enumerate() {
                        for (j, next) in annotations.iter().enumerate() {
                            // This label overlaps with another one and both take space (
                            // they have text and are not multiline lines).
                            if overlaps(next, annotation, 0)
                                && annotation.has_label()
                                && j > i
                                && p == 0
                            // We're currently on the first line, move the label one line down
                            {
                                // If we're overlapping with an un-labelled annotation with the same span
                                // we can just merge them in the output
                                if next.range.0 == annotation.range.0
                                    && next.range.1 == annotation.range.1
                                    && !next.has_label()
                                {
                                    continue;
                                }

                                // This annotation needs a new line in the output.
                                p += 1;
                                break;
                            }
                        }
                        annotations_positions.push((p, annotation));
                        for (j, next) in annotations.iter().enumerate() {
                            if j > i {
                                let l = next
                                    .annotation
                                    .label
                                    .iter()
                                    .map(|label| label.content)
                                    .collect::<Vec<_>>()
                                    .join("")
                                    .len()
                                    + 2;
                                // Do not allow two labels to be in the same line if they
                                // overlap including padding, to avoid situations like:
                                //
                                // fn foo(x: u32) {
                                // -------^------
                                // |      |
                                // fn_spanx_span
                                //
                                // Both labels must have some text, otherwise they are not
                                // overlapping. Do not add a new line if this annotation or
                                // the next are vertical line placeholders. If either this
                                // or the next annotation is multiline start/end, move it
                                // to a new line so as not to overlap the horizontal lines.
                                if (overlaps(next, annotation, l)
                                    && annotation.has_label()
                                    && next.has_label())
                                    || (annotation.takes_space() && next.has_label())
                                    || (annotation.has_label() && next.takes_space())
                                    || (annotation.takes_space() && next.takes_space())
                                    || (overlaps(next, annotation, l)
                                        && next.range.1 <= annotation.range.1
                                        && next.has_label()
                                        && p == 0)
                                // Avoid #42595.
                                {
                                    // This annotation needs a new line in the output.
                                    p += 1;
                                    break;
                                }
                            }
                        }
                        line_len = max(line_len, p);
                    }

                    if line_len != 0 {
                        line_len += 1;
                    }

                    if annotations_positions.iter().all(|(_, ann)| {
                        matches!(
                            ann.annotation_part,
                            DisplayAnnotationPart::MultilineStart(_)
                        )
                    }) {
                        if let Some(max_pos) =
                            annotations_positions.iter().map(|(pos, _)| *pos).max()
                        {
                            // Special case the following, so that we minimize overlapping multiline spans.
                            //
                            // 3 │       X0 Y0 Z0
                            //   │ ┏━━━━━┛  │  │     < We are writing these lines
                            //   │ ┃┌───────┘  │     < by reverting the "depth" of
                            //   │ ┃│┌─────────┘     < their multilne spans.
                            // 4 │ ┃││   X1 Y1 Z1
                            // 5 │ ┃││   X2 Y2 Z2
                            //   │ ┃│└────╿──│──┘ `Z` label
                            //   │ ┃└─────│──┤
                            //   │ ┗━━━━━━┥  `Y` is a good letter too
                            //   ╰╴       `X` is a good letter
                            for (pos, _) in &mut annotations_positions {
                                *pos = max_pos - *pos;
                            }
                            // We know then that we don't need an additional line for the span label, saving us
                            // one line of vertical space.
                            line_len = line_len.saturating_sub(1);
                        }
                    }

                    // This is a special case where we have a multiline
                    // annotation that is at the start of the line disregarding
                    // any leading whitespace, and no other multiline
                    // annotations overlap it. In this case, we want to draw
                    //
                    // 2 |   fn foo() {
                    //   |  _^
                    // 3 | |
                    // 4 | | }
                    //   | |_^ test
                    //
                    // we simplify the output to:
                    //
                    // 2 | / fn foo() {
                    // 3 | |
                    // 4 | | }
                    //   | |_^ test
                    if multiline_depth == 1
                        && annotations_positions.len() == 1
                        && annotations_positions
                            .first()
                            .map_or(false, |(_, annotation)| {
                                matches!(
                                    annotation.annotation_part,
                                    DisplayAnnotationPart::MultilineStart(_)
                                ) && text
                                    .chars()
                                    .take(annotation.range.0)
                                    .all(|c| c.is_whitespace())
                            })
                    {
                        let (_, ann) = annotations_positions.remove(0);
                        let style = get_annotation_style(&ann.annotation_type, stylesheet);
                        buffer.putc(line_offset, 3 + lineno_width, '/', *style);
                    }

                    // Draw the column separator for any extra lines that were
                    // created
                    //
                    // After this we will have:
                    //
                    // 2 |   fn foo() {
                    //   |
                    //   |
                    //   |
                    // 3 |
                    // 4 |   }
                    //   |
                    if !annotations_positions.is_empty() {
                        for pos in 0..=line_len {
                            buffer.putc(
                                line_offset + pos + 1,
                                lineno_width + 1,
                                '|',
                                stylesheet.line_no,
                            );
                        }
                    }

                    // Write the horizontal lines for multiline annotations
                    // (only the first and last lines need this).
                    //
                    // After this we will have:
                    //
                    // 2 |   fn foo() {
                    //   |  __________
                    //   |
                    //   |
                    // 3 |
                    // 4 |   }
                    //   |  _
                    for &(pos, annotation) in &annotations_positions {
                        let style = get_annotation_style(&annotation.annotation_type, stylesheet);
                        let pos = pos + 1;
                        match annotation.annotation_part {
                            DisplayAnnotationPart::MultilineStart(depth)
                            | DisplayAnnotationPart::MultilineEnd(depth) => {
                                for col in width_offset + depth
                                    ..(code_offset + annotation.range.0).saturating_sub(left)
                                {
                                    buffer.putc(line_offset + pos, col + 1, '_', *style);
                                }
                            }
                            _ => {}
                        }
                    }

                    // Write the vertical lines for labels that are on a different line as the underline.
                    //
                    // After this we will have:
                    //
                    // 2 |   fn foo() {
                    //   |  __________
                    //   | |    |
                    //   | |
                    // 3 | |
                    // 4 | | }
                    //   | |_
                    for &(pos, annotation) in &annotations_positions {
                        let style = get_annotation_style(&annotation.annotation_type, stylesheet);
                        let pos = pos + 1;
                        if pos > 1 && (annotation.has_label() || annotation.takes_space()) {
                            for p in line_offset + 2..=line_offset + pos {
                                buffer.putc(
                                    p,
                                    (code_offset + annotation.range.0).saturating_sub(left),
                                    '|',
                                    *style,
                                );
                            }
                        }
                        match annotation.annotation_part {
                            DisplayAnnotationPart::MultilineStart(depth) => {
                                for p in line_offset + pos + 1..line_offset + line_len + 2 {
                                    buffer.putc(p, width_offset + depth, '|', *style);
                                }
                            }
                            DisplayAnnotationPart::MultilineEnd(depth) => {
                                for p in line_offset..=line_offset + pos {
                                    buffer.putc(p, width_offset + depth, '|', *style);
                                }
                            }
                            _ => {}
                        }
                    }

                    // Add in any inline marks for any extra lines that have
                    // been created. Output should look like above.
                    for inline_mark in inline_marks {
                        let DisplayMarkType::AnnotationThrough(depth) = inline_mark.mark_type;
                        let style = get_annotation_style(&inline_mark.annotation_type, stylesheet);
                        if annotations_positions.is_empty() {
                            buffer.putc(line_offset, width_offset + depth, '|', *style);
                        } else {
                            for p in line_offset..=line_offset + line_len + 1 {
                                buffer.putc(p, width_offset + depth, '|', *style);
                            }
                        }
                    }

                    // Write the labels on the annotations that actually have a label.
                    //
                    // After this we will have:
                    //
                    // 2 |   fn foo() {
                    //   |  __________
                    //   |      |
                    //   |      something about `foo`
                    // 3 |
                    // 4 |   }
                    //   |  _  test
                    for &(pos, annotation) in &annotations_positions {
                        if !is_annotation_empty(&annotation.annotation) {
                            let style =
                                get_annotation_style(&annotation.annotation_type, stylesheet);
                            let mut formatted_len = if let Some(id) = &annotation.annotation.id {
                                2 + id.len()
                                    + annotation_type_len(&annotation.annotation.annotation_type)
                            } else {
                                annotation_type_len(&annotation.annotation.annotation_type)
                            };
                            let (pos, col) = if pos == 0 {
                                (pos + 1, (annotation.range.1 + 1).saturating_sub(left))
                            } else {
                                (pos + 2, annotation.range.0.saturating_sub(left))
                            };
                            if annotation.annotation_part
                                == DisplayAnnotationPart::LabelContinuation
                            {
                                formatted_len = 0;
                            } else if formatted_len != 0 {
                                formatted_len += 2;
                                let id = match &annotation.annotation.id {
                                    Some(id) => format!("[{id}]"),
                                    None => String::new(),
                                };
                                buffer.puts(
                                    line_offset + pos,
                                    col + code_offset,
                                    &format!(
                                        "{}{}: ",
                                        annotation_type_str(&annotation.annotation_type),
                                        id
                                    ),
                                    *style,
                                );
                            } else {
                                formatted_len = 0;
                            }
                            let mut before = 0;
                            for fragment in &annotation.annotation.label {
                                let inner_col = before + formatted_len + col + code_offset;
                                buffer.puts(line_offset + pos, inner_col, fragment.content, *style);
                                before += fragment.content.len();
                            }
                        }
                    }

                    // Sort from biggest span to smallest span so that smaller spans are
                    // represented in the output:
                    //
                    // x | fn foo()
                    //   | ^^^---^^
                    //   | |  |
                    //   | |  something about `foo`
                    //   | something about `fn foo()`
                    annotations_positions.sort_by_key(|(_, ann)| {
                        // Decreasing order. When annotations share the same length, prefer `Primary`.
                        Reverse(ann.len())
                    });

                    // Write the underlines.
                    //
                    // After this we will have:
                    //
                    // 2 |   fn foo() {
                    //   |  ____-_____^
                    //   |      |
                    //   |      something about `foo`
                    // 3 |
                    // 4 |   }
                    //   |  _^  test
                    for &(_, annotation) in &annotations_positions {
                        let mark = match annotation.annotation_type {
                            DisplayAnnotationType::Error => '^',
                            DisplayAnnotationType::Warning => '-',
                            DisplayAnnotationType::Info => '-',
                            DisplayAnnotationType::Note => '-',
                            DisplayAnnotationType::Help => '-',
                            DisplayAnnotationType::None => ' ',
                        };
                        let style = get_annotation_style(&annotation.annotation_type, stylesheet);
                        for p in annotation.range.0..annotation.range.1 {
                            buffer.putc(
                                line_offset + 1,
                                (code_offset + p).saturating_sub(left),
                                mark,
                                *style,
                            );
                        }
                    }
                } else if !inline_marks.is_empty() {
                    format_inline_marks(
                        line_offset,
                        inline_marks,
                        lineno_width,
                        stylesheet,
                        buffer,
                    )?;
                }
                Ok(())
            }
            DisplayLine::Fold { inline_marks } => {
                buffer.puts(line_offset, 0, "...", *stylesheet.line_no());
                if !inline_marks.is_empty() || 0 < multiline_depth {
                    format_inline_marks(
                        line_offset,
                        inline_marks,
                        lineno_width,
                        stylesheet,
                        buffer,
                    )?;
                }
                Ok(())
            }
            DisplayLine::Raw(line) => {
                self.format_raw_line(line_offset, line, lineno_width, stylesheet, buffer)
            }
        }
    }
}
