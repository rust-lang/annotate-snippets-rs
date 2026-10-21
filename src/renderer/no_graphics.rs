use alloc::borrow::{Cow, ToOwned};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Reverse;
use core::fmt::{self, Write};

use super::graphics::{Hyperlink, MessageOrTitle, TitleStyle, str_width};
use super::preprocess::{Preprocessed, PreprocessedElement, PreprocessedGroup};
use super::{ElementStyle, Stylesheet, normalize_whitespace};
use crate::renderer::graphics::ANONYMIZED_LINE_NUM;
use crate::{AnnotationKind, Id, Renderer, Report};

/// Render all errors from a `Report` as text only.
///
/// We will render output as follows:
///
/// ```text
/// error EXXXX: main message
///  at $DIR/file.ext:LL:CC: span label
///   on line LL, column KK: span label
/// note: note message
///  at $DIR/file.ext:LL:CC
/// help: suggestion message
///   on line LL, add `suggestion`
/// ```
pub(crate) fn render_no_graphics(
    renderer: &Renderer,
    groups: Report<'_>,
) -> Result<String, fmt::Error> {
    let mut output = String::new();
    let Preprocessed {
        max_line_num: _,
        report_primary_path,
        groups,
    } = Preprocessed::preprocess(groups);

    for PreprocessedGroup {
        group,
        elements,
        primary_path,
        max_depth: _,
    } in groups
    {
        if let Some(title) = &group.title {
            if !output.is_empty() {
                writeln!(output)?;
            }
            let title_style = if title.allows_styling {
                TitleStyle::Secondary
            } else {
                TitleStyle::Primary
            };
            render_title(
                title,
                &mut output,
                title_style,
                &renderer.stylesheet,
                renderer.hyperlink,
            )?;
        }

        let mut last_suggestion_path = None;
        let mut suggestion_count = 0;

        let mut message_iter = elements.into_iter().peekable();
        while let Some(section) = message_iter.next() {
            match section {
                PreprocessedElement::Message(message) => {
                    last_suggestion_path = None;
                    if !output.is_empty() {
                        writeln!(output)?;
                    }
                    render_title(
                        message,
                        &mut output,
                        TitleStyle::Message,
                        &renderer.stylesheet,
                        renderer.hyperlink,
                    )?;
                }
                PreprocessedElement::Cause((snippet, sm, _)) => {
                    last_suggestion_path = None;

                    let mut annotations = snippet
                        .markers
                        .iter()
                        .filter(|ann| !matches!(ann.kind, AnnotationKind::Visible))
                        .collect::<Vec<_>>();
                    annotations.sort_by_key(|a| (Reverse(a.kind.is_primary()), a.span.start));
                    if annotations.is_empty() {
                        if !output.is_empty() {
                            writeln!(output)?;
                        }
                        // We have a diagnostic with no span labels, but we should show *some*
                        // position. We use the snippet start to provide that minimal context.
                        write!(output, " ")?;
                        render_path(
                            &mut output,
                            snippet.path.as_deref(),
                            Some(snippet.line_start),
                            None,
                            renderer.anonymized_origin_line_numbers,
                        )?;
                    } else {
                        for (i, annotation) in annotations.iter().enumerate() {
                            let label = annotation.label.as_ref().filter(|s| !s.is_empty());
                            if i > 0 && label.is_none() {
                                continue;
                            }
                            let (lo, hi) =
                                sm.span_to_locations(annotation.span.start..annotation.span.end);

                            if !output.is_empty() {
                                writeln!(output)?;
                            }
                            // Indent subsequent labels under the first label's file path.
                            // Without a path, keep all labels at the same indentation.
                            let padding = if i > 0 && snippet.path.is_some() {
                                "  "
                            } else {
                                " "
                            };
                            write!(output, "{padding}")?;

                            if i == 0
                                && let Some(path) = &snippet.path
                            {
                                // `at $DIR/file.txt:LL:CC: label`
                                //  ^^^^^^^^^^^^^^^^^^^^^^
                                render_path(
                                    &mut output,
                                    Some(path),
                                    Some(lo.line),
                                    Some(lo.char + 1),
                                    renderer.anonymized_origin_line_numbers,
                                )?;
                                if lo.line != hi.line {
                                    // This is a multiline highlight, so we mention both the start and the
                                    // end. `LL:CC to MM:DD`
                                    //            ^^^^^^^^^
                                    write!(output, " to {}:{}", hi.line, hi.char + 1)?;
                                }
                            } else {
                                let col = if let Some(line) = sm.get_line(lo.line)
                                    && line.chars().take(lo.char).all(|c| c.is_whitespace())
                                {
                                    // Everything before the span is whitespace, mentioning the column
                                    // doesn't add information.
                                    None
                                } else {
                                    Some(lo.char + 1)
                                };
                                render_path(
                                    &mut output,
                                    None,
                                    Some(lo.line),
                                    col,
                                    renderer.anonymized_origin_line_numbers,
                                )?;
                                if lo.line != hi.line {
                                    // This is a multiline highlight, so we mention both the start and the
                                    // end. `line LL, column CC to line MM, column DD`
                                    //                         ^^^^^^^^^^^^^^^^^^^^^^
                                    write!(output, " to line {}", hi.line)?;
                                    if let Some(line) = sm.get_line(hi.line)
                                        && let Some(post) = line.get(hi.char..)
                                        && post.chars().all(|c| c.is_whitespace())
                                    {
                                        // Everything after the span is whitespace, mentioning the
                                        // column doesn't add information.
                                    } else {
                                        write!(output, ", column {}", hi.char + 1)?;
                                    }
                                }
                            }

                            if let Some(label) = label {
                                // If the span has a label, we render it to the right of the position
                                // information.
                                // `at $DIR/file.txt:LL:CC: this is the label`
                                //                        ^^^^^^^^^^^^^^^^^^^
                                write!(output, ": {label}")?;
                            }
                        }
                    }
                }
                PreprocessedElement::Suggestion((
                    suggestion,
                    sm,
                    spliced_lines,
                    _display_suggestion,
                )) => {
                    if spliced_lines.patches.is_empty() {
                        // We have a suggestion with no patches, we don't render anything.
                        continue;
                    }

                    let matches_previous_suggestion =
                        last_suggestion_path == Some(suggestion.path.as_ref());

                    suggestion_count = if matches_previous_suggestion {
                        suggestion_count + 1
                    } else {
                        1
                    };

                    let matches_next_suggestion = matches!(
                        message_iter.peek(),
                        Some(PreprocessedElement::Suggestion((next, ..)))
                            if next.path.as_ref() == suggestion.path.as_ref()
                    );

                    // We only include the file path when it is different from
                    // the previous suggestion's path, or the primary path
                    //
                    // `at $DIR/file.txt:LL:CC: label`
                    //  ^^^^^^^^^^^^^^^^^
                    let previous_path =
                        last_suggestion_path.unwrap_or(primary_path.or(report_primary_path));
                    let path = if suggestion.path.as_ref() != previous_path
                        && let Some(path) = suggestion.path.as_ref()
                    {
                        Some(path.as_ref())
                    } else {
                        None
                    };

                    let padding = if matches_previous_suggestion || matches_next_suggestion {
                        if !output.is_empty() {
                            writeln!(output)?;
                        }
                        write!(output, " option {suggestion_count}")?;
                        "  "
                    } else {
                        " "
                    };

                    for patch in spliced_lines.patches.iter() {
                        let (lo, hi) = sm.span_to_locations(patch.span.start..patch.span.end);

                        let has_replacement = !patch.replacement.trim().is_empty();
                        let is_addition = patch.is_addition(&sm);
                        if !output.is_empty() {
                            writeln!(output)?;
                        }
                        write!(output, "{padding}")?;

                        if has_replacement && !is_addition {
                            render_path(
                                &mut output,
                                path,
                                Some(lo.line),
                                Some(lo.char + 1),
                                renderer.anonymized_origin_line_numbers,
                            )?;
                            if lo.line != hi.line {
                                let line = if renderer.anonymized_origin_line_numbers {
                                    ANONYMIZED_LINE_NUM.to_owned()
                                } else {
                                    hi.line.to_string()
                                };
                                write!(output, " to line {line}, column {}", hi.char + 1)?;
                            } else {
                                write!(output, " to column {}", hi.char + 1)?;
                            }
                        } else {
                            let col = if lo.line == hi.line
                                && let Some(line) = sm.get_line(lo.line)
                                && let Some(pre) = line.get(..lo.byte)
                                && pre.chars().all(|c| c.is_whitespace())
                                && let Some(post) = line.get(hi.byte..)
                                && (patch.replacement.lines().count() > 1
                                    || post.chars().all(|c| c.is_whitespace()))
                            {
                                // We are changing the whole text in the line, no need to mention the
                                // column.
                                None
                            } else {
                                Some(lo.char + 1)
                            };
                            render_path(
                                &mut output,
                                path,
                                Some(lo.line),
                                col,
                                renderer.anonymized_origin_line_numbers,
                            )?;
                        }

                        if has_replacement {
                            let action = if is_addition { "add" } else { "replace with" };
                            write!(output, " {action}: ")?;

                            let st = ElementStyle::Addition
                                .color_spec(&crate::Level::NOTE, &renderer.stylesheet);

                            write!(
                                output,
                                "{st}{}{st:#}",
                                normalize_whitespace(patch.replacement.trim_end_matches('\n'))
                            )?;
                        }
                    }
                    last_suggestion_path = Some(suggestion.path.as_ref());
                }
                PreprocessedElement::Origin(origin) => {
                    last_suggestion_path = None;
                    if !output.is_empty() {
                        writeln!(output)?;
                    }
                    write!(output, " ")?;
                    render_path(
                        &mut output,
                        Some(&origin.path),
                        origin.line,
                        origin.char_column,
                        renderer.anonymized_origin_line_numbers,
                    )?;
                }
                PreprocessedElement::Padding(_) => {
                    last_suggestion_path = None;
                }
            }
        }
    }

    Ok(output)
}

/// Render the main message line for a diagnostic
///
/// This will print out:
///
/// ```text
/// LEVEL CODE: message
/// ```
/// like the following:
/// ```text
/// error E0308: mismatched types
/// ```
fn render_title(
    title: &dyn MessageOrTitle,
    buffer: &mut String,
    title_style: TitleStyle,
    stylesheet: &Stylesheet,
    hyperlink: bool,
) -> Result<(), fmt::Error> {
    let (label_style, title_element_style) = match title_style {
        TitleStyle::Primary => (
            ElementStyle::Level(title.level().level),
            ElementStyle::MainHeaderMsg,
        ),
        TitleStyle::Secondary => (
            ElementStyle::Level(title.level().level),
            ElementStyle::HeaderMsg,
        ),
        TitleStyle::Message => (ElementStyle::MainHeaderMsg, ElementStyle::NoStyle),
    };
    let label_style = label_style.color_spec(title.level(), stylesheet);
    let title_element_style = title_element_style.color_spec(title.level(), stylesheet);

    let mut label_width = 0;
    let level_is_visible = title.level().name != Some(None);
    if level_is_visible || title.id().is_some() {
        if title_style == TitleStyle::Message {
            write!(buffer, " ")?;
            label_width += 1;
        }

        if level_is_visible {
            // error EXXXX: message
            // ^^^^^
            write!(buffer, "{}{}{0:#}", label_style, title.level().as_str(),)?;
            label_width += str_width(title.level().as_str());
        }

        if let Some(Id { id: Some(id), url }) = &title.id() {
            let url = url
                .as_deref()
                .filter(|_| hyperlink)
                .map(Hyperlink::with_url)
                .unwrap_or_default();

            if level_is_visible {
                // error EXXXX: message
                //      ^
                write!(buffer, " ")?;
                label_width += 1;
            }
            // error EXXXX: message
            //       ^^^^^
            write!(buffer, "{label_style}{url}{id}{url:#}{label_style:#}")?;
            label_width += str_width(id);
        }

        if let Some(footnote_marker) = title.footnote_marker() {
            let descr = " footnote ";
            label_width += str_width(descr);
            write!(
                buffer,
                "{label_style} {descr}{footnote_marker}{label_style:#}"
            )?;
            label_width += str_width(footnote_marker);
        }

        // error EXXXX: message
        //            ^
        write!(buffer, "{title_element_style}: {title_element_style:#}")?;
        label_width += 2;
    }
    let padding = " ".repeat(label_width);

    // error EXXXX: message
    //              ^^^^^^^
    let (title_str, st) = if title.allows_styling() {
        (
            Cow::Borrowed(title.text()),
            ElementStyle::NoStyle.color_spec(title.level(), stylesheet),
        )
    } else {
        (normalize_whitespace(title.text()), title_element_style)
    };
    for (i, text) in title_str.split('\n').enumerate() {
        if i != 0 {
            write!(buffer, "\n{padding}")?;
        }
        write!(buffer, "{st}{text}{st:#}")?;
    }
    Ok(())
}

/// Print out a file position optimized for the data available.
///
/// When the `path` is available, we print `at PATH:LL:CC`, which is detected by terminals and
/// editors as a path to a specific place in a file. Otherwise, we print`on line LL, column CC`,
/// for the elements that are available.
fn render_path(
    output: &mut String,
    path: Option<&str>,
    line: Option<usize>,
    col: Option<usize>,
    anonymized_origin_line_numbers: bool,
) -> Result<(), fmt::Error> {
    let line = line.map(|l| {
        if anonymized_origin_line_numbers {
            ANONYMIZED_LINE_NUM.to_owned()
        } else {
            l.to_string()
        }
    });

    if let Some(path) = path {
        // `at $DIR/file.txt`
        write!(output, "at {path}")?;
        if let Some(line) = line {
            // `at $DIR/file.txt:LL`
            write!(output, ":{line}")?;
            if let Some(col) = col {
                // `at $DIR/file.txt:LL:CC`
                write!(output, ":{col}")?;
            }
        }
    } else if let Some(line) = line {
        // We are not printing the path, so instead we show
        // ` on line LL`
        write!(output, "on line {line}")?;
        if let Some(col) = col {
            // ` on line LL, column CC`.
            write!(output, ", column {col}")?;
        }
    }
    Ok(())
}
