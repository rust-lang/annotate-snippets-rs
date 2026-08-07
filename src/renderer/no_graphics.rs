use alloc::{string::String, vec::Vec};
use core::cmp::Reverse;
use core::fmt::{self, Write};

use crate::{
    Id, Renderer, Report,
    renderer::{
        ElementStyle, Stylesheet,
        render::{MessageOrTitle, PreProcessedElement, PreProcessedGroup, pre_process},
        source_map,
    },
};

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
) -> Result<(), fmt::Error> {
    match (path, line, col) {
        (Some(path), Some(line), Some(col)) => {
            // `at $DIR/file.txt:LL:CC`
            write!(output, "at {path}:{line}:{col}")?;
        }
        (Some(path), Some(line), None) => {
            // `at $DIR/file.txt:LL`
            write!(output, "at {path}:{line}")?;
        }
        (Some(path), None, None) => {
            // `at $DIR/file.txt`
            write!(output, "at {path}")?;
        }
        (None, Some(line), Some(col)) => {
            // We are not printing the path, so instead we show
            // ` on line LL, column CC`.
            write!(output, "on line {line}, column {col}")?;
        }
        (None, Some(line), None) => {
            // ` on line LL`
            write!(output, "on line {line}")?;
        }
        (None, None, Some(col)) => {
            // ` on column CC`
            write!(output, "on column {col}")?;
        }
        (Some(path), None, Some(line)) => {
            // This condition should never have been called.
            write!(output, "at {path}, line {line}")?;
        }
        (None, None, None) => {}
    }
    Ok(())
}

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
    let (_max_line_num, og_primary_path, groups) = pre_process(groups);

    let mut iter = groups.into_iter().peekable();
    while let Some(PreProcessedGroup {
        group,
        elements,
        primary_path: _,
        max_depth: _,
    }) = iter.next()
    {
        if let Some(title) = &group.title {
            render_title(
                title,
                &mut output,
                if title.allows_styling {
                    ElementStyle::HeaderMsg
                } else {
                    ElementStyle::MainHeaderMsg
                },
                &renderer.stylesheet,
            )?;
            if iter.peek().is_some()
                || elements
                    .iter()
                    .find(|section| !matches!(section, PreProcessedElement::Padding(_)))
                    .is_some()
            {
                // This diagnostic has content, add a newline after the title.
                writeln!(output)?;
            }
        }

        let mut message_iter = elements.into_iter().enumerate().peekable();
        let mut last_suggestion_path = None;
        while let Some((_i, section)) = message_iter.next() {
            let peek = message_iter.peek().map(|(_, s)| s);
            match section {
                PreProcessedElement::Message(message) => {
                    last_suggestion_path = None;
                    render_title(
                        message,
                        &mut output,
                        ElementStyle::HeaderMsg,
                        &renderer.stylesheet,
                    )?;
                    if peek.is_some() {
                        writeln!(output)?;
                    }
                }
                PreProcessedElement::Cause((snippet, _, _)) => {
                    last_suggestion_path = None;
                    let sm = source_map::SourceMap::new(&snippet.source, snippet.line_start);

                    let mut annotations = snippet.markers.iter().collect::<Vec<_>>();
                    annotations.sort_by_key(|a| (Reverse(a.kind.is_primary()), a.span.start));
                    if annotations.is_empty() {
                        // We have a diagnostic with no span labels, but we should show *some*
                        // position. We use the snippet start to provide that minimal context.
                        write!(output, " ")?;
                        render_path(
                            &mut output,
                            snippet.path.as_deref(),
                            Some(snippet.line_start),
                            None,
                        )?;
                        if peek.is_some() {
                            writeln!(output)?;
                        }
                    }

                    for (i, annotation) in annotations.iter().enumerate() {
                        let label = annotation.label.as_ref().filter(|s| !s.is_empty());
                        if i > 0 {
                            if label.is_none() {
                                continue;
                            } else if peek.is_none() {
                                writeln!(output)?;
                            }
                        }
                        let (lo, hi) =
                            sm.span_to_locations(annotation.span.start..annotation.span.end);
                        if i == 0
                            && let Some(path) = &snippet.path
                        {
                            // `at $DIR/file.txt:LL:CC: label`
                            //  ^^^^^^^^^^^^^^^^^^^^^^
                            write!(output, " ")?;
                            render_path(&mut output, Some(path), Some(lo.line), Some(lo.char + 1))?;
                            if lo.line != hi.line {
                                // This is a multiline highlight, so we mention both the start and the
                                // end. `LL:CC to MM:DD`
                                //            ^^^^^^^^^
                                write!(output, " to {}:{}", hi.line, hi.char + 1)?;
                            }
                        } else {
                            let col = if let Some(line) = sm.get_line(lo.line)
                                && let Some(pre) = line.get(..lo.char)
                                && pre.chars().all(|c| c.is_whitespace())
                            {
                                // Everything before the span is whitespace, mentioning the column
                                // doesn't add information.
                                None
                            } else {
                                Some(lo.char + 1)
                            };
                            // On subsequent labels, we don't need to repeat the path. We use
                            // additional whitespace to imply a nested relationship to the prior
                            // label.
                            write!(output, "  ")?;
                            render_path(&mut output, None, Some(lo.line), col)?;
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
                        if peek.is_some() {
                            writeln!(output)?;
                        }
                    }
                }
                PreProcessedElement::Suggestion((
                    suggestion,
                    _source_map,
                    _spliced_lines,
                    _display_suggestion,
                )) => {
                    let Some(first_patch) = suggestion.markers.first() else {
                        continue;
                    };
                    let sm = source_map::SourceMap::new(&suggestion.source, suggestion.line_start);
                    let next_is_suggestion =
                        matches!(peek, Some(PreProcessedElement::Suggestion(_)));

                    let replacement = first_patch.replacement.trim_end_matches('\n');
                    if last_suggestion_path.is_none() {
                        let (lo, hi) =
                            sm.span_to_locations(first_patch.span.start..first_patch.span.end);

                        let col = if lo.line == hi.line
                            && let Some(line) = sm.get_line(lo.line)
                            && let Some(pre) = line.get(..lo.char)
                            && pre.chars().all(|c| c.is_whitespace())
                            && let Some(post) = line.get(hi.char..)
                            && (replacement.lines().count() > 1
                                || post.chars().all(|c| c.is_whitespace()))
                        {
                            // We are changing the whole text in the line, no need to mention the
                            // column.
                            None
                        } else {
                            Some(lo.char.max(1))
                        };
                        let path: Option<&str> = match (&suggestion.path, og_primary_path) {
                            (Some(path), Some(primary)) if path != primary => {
                                // We only include the file path when it is different to the
                                // primary file.
                                //
                                // `at $DIR/file.txt:LL:CC: label`
                                //  ^^^^^^^^^^^^^^^^^
                                Some(path)
                            }
                            _ => None,
                        };
                        write!(output, " ")?;
                        render_path(&mut output, path, Some(lo.line), col)?;
                        let add = if let Some(snippet) =
                            sm.span_to_snippet(first_patch.span.start..first_patch.span.end)
                            && snippet.chars().all(|c| c.is_whitespace())
                        {
                            "add"
                        } else {
                            "replace with"
                        };

                        if next_is_suggestion {
                            // We have multiple suggestions. We will render on their own line, first
                            // the message, then the position, and finally each of the suggestions.
                            //
                            // help: suggestion message
                            //  at line LL, column CC, add one of
                            //   first suggestion
                            //   second suggestion
                            writeln!(output, ", {add} one of")?;
                        } else {
                            // We have a single suggestion. We will render first the message, then
                            // the position followed by the suggestion on the next line.
                            //
                            // help: suggestion message
                            //  on line LL, column CC, add `addition`
                            if !replacement.trim().is_empty() {
                                // If it is a removal, we shorten the output.
                                //
                                // help: suggestion message to remove something
                                //  at line LL, column CC
                                write!(output, ", {add} ")?;
                            }
                        }
                    }

                    let st = ElementStyle::Addition
                        .color_spec(&crate::Level::NOTE, &renderer.stylesheet);
                    if next_is_suggestion || last_suggestion_path.is_some() {
                        // Multiple suggestions.
                        writeln!(output, "  {st}{replacement}{st:#}")?;
                    } else if !replacement.trim().is_empty() {
                        // Single addition suggestion
                        write!(output, "`{st}{replacement}{st:#}`")?;
                    }

                    last_suggestion_path = Some(suggestion.path.as_ref());
                }
                PreProcessedElement::Origin(origin) => {
                    last_suggestion_path = None;
                    write!(output, " ")?;
                    render_path(
                        &mut output,
                        Some(&origin.path),
                        origin.line,
                        origin.char_column,
                    )?;
                    if peek.is_some() {
                        writeln!(output)?;
                    }
                }
                PreProcessedElement::Padding(_) => {}
            }
        }

        if iter.peek().is_some()
            && let Some(c) = output.chars().last()
            && c != '\n'
        {
            writeln!(output)?;
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
    style: ElementStyle,
    stylesheet: &Stylesheet,
) -> Result<(), fmt::Error> {
    let mut label_width = 0;
    let st = style.color_spec(title.level(), stylesheet);

    if title.level().name != Some(None) {
        // error EXXXX: message
        // ^^^^^
        write!(
            buffer,
            "{}{}{0:#}",
            ElementStyle::Level(title.level().level).color_spec(&crate::Level::NOTE, stylesheet),
            title.level().as_str(),
        )?;
        label_width += title.level().as_str().len();

        if let Some(Id {
            id: Some(id),
            url: _,
        }) = &title.id()
        {
            // error EXXXX: message
            //       ^^^^^
            write!(buffer, "{st:#} {id}",)?;
            label_width += 1 + id.len();
        }
        // error EXXXX: message
        //            ^
        write!(buffer, ": ")?;
        label_width += 2;
    }
    let padding = " ".repeat(label_width);

    // error EXXXX: message
    //              ^^^^^^^
    let title_str = title.text();
    for (i, text) in title_str.split('\n').enumerate() {
        if i != 0 {
            write!(buffer, "\n{padding}")?;
        }
        write!(buffer, "{st}{text}{st:#}")?;
    }
    Ok(())
}
