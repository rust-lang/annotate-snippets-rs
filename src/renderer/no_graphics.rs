use alloc::{format, string::String, vec::Vec};
use core::cmp::Reverse;
use core::fmt;

use crate::{
    Id, Renderer, Report,
    renderer::{
        ElementStyle,
        render::{MessageOrTitle, PreProcessedElement, PreProcessedGroup, pre_process},
        source_map,
        styled_buffer::StyledBuffer,
    },
};

pub(crate) fn render_no_graphics(
    renderer: &Renderer,
    groups: Report<'_>,
) -> Result<String, fmt::Error> {
    // We will render output as follows:
    //
    // error EXXXX: main message
    //  on $DIR/file.ext, line LL, column CC: span label
    //   at line LL, column KK: span label
    // note: note message
    //  on $DIR/file.ext, line LL, column CC
    // help: suggestion message
    //   at line LL, column KK, add `suggestion`

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
        let mut buffer = StyledBuffer::new();
        let mut line = 0;
        if let Some(title) = &group.title {
            render_title(title, &mut line, &mut buffer, ElementStyle::MainHeaderMsg);
        }

        let mut message_iter = elements.into_iter().enumerate().peekable();
        let mut last_suggestion_path = None;
        while let Some((_i, section)) = message_iter.next() {
            let peek = message_iter.peek().map(|(_, s)| s);
            match section {
                PreProcessedElement::Message(message) => {
                    if last_suggestion_path.is_some() && message.level.name == Some(None) {
                        let text = message.text.as_ref();
                        let text = text.strip_prefix("and ").unwrap_or(text);

                        buffer.append(line, "  or ", ElementStyle::NoStyle);
                        buffer.append(line, text, ElementStyle::NoStyle);
                        last_suggestion_path = None;
                    } else {
                        last_suggestion_path = None;
                        render_title(message, &mut line, &mut buffer, ElementStyle::HeaderMsg);
                    }
                }
                PreProcessedElement::Cause((snippet, _, _)) => {
                    let sm = source_map::SourceMap::new(&snippet.source, snippet.line_start);

                    let mut annotations = snippet.markers.iter().collect::<Vec<_>>();
                    annotations.sort_by_key(|a| (Reverse(a.kind.is_primary()), a.span.start));

                    let start_line = line;
                    for (i, annotation) in annotations.iter().enumerate() {
                        let label = annotation.label.as_ref().filter(|s| !s.is_empty());
                        if i > 0 && label.is_none() {
                            continue;
                        }
                        let (lo, hi) =
                            sm.span_to_locations(annotation.span.start..annotation.span.end);
                        if i == 0 {
                            if let Some(path) = &snippet.path {
                                // `at $DIR/file.txt, on line LL, column CC: label`
                                //  ^^^^^^^^^^^^^^^^^
                                buffer.append(line, " at ", ElementStyle::NoStyle);
                                buffer.append(line, path, ElementStyle::NoStyle);
                                buffer.append(line, ",", ElementStyle::NoStyle);
                            }
                        }

                        let (prefix, suffix) = if lo.line == hi.line {
                            // `on line LL, column CC`
                            //  ^^
                            ("on", String::new())
                        } else {
                            // This is a multiline highlight, so we mention both the start and the
                            // end. `from line LL, column CC to line MM, column DD`
                            //       ^^^^                   ^^^^^^^^^^^^^^^^^^^^^^
                            (
                                "from",
                                format!(" to line {}, column {}", hi.line, hi.char + 1),
                            )
                        };

                        // If the position within the file is on its own line, without a path, we
                        // indent it one space further.
                        let indent = if start_line == line { "" } else { " " };
                        buffer.append(
                            line,
                            &format!(
                                " {indent}{prefix} line {}, column {}{suffix}",
                                lo.line,
                                lo.char + 1
                            ),
                            ElementStyle::NoStyle,
                        );

                        if let Some(label) = label {
                            // If the span has a label, we render it to the right of the position
                            // information.
                            // `on line LL, column CC: this is the label`
                            //                       ^^^^^^^^^^^^^^^^^^^
                            buffer.append(line, ": ", ElementStyle::NoStyle);
                            buffer.append(line, label, ElementStyle::NoStyle);
                        }
                        line += 1;
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

                    let no_preceding_line = line == 0;
                    if no_preceding_line {
                        line += 1;
                    }
                    let replacement = first_patch.replacement.trim_end_matches('\n');
                    if last_suggestion_path.is_none() {
                        let (lo, _) =
                            sm.span_to_locations(first_patch.span.start..first_patch.span.end);
                        let col = lo.char.max(1);
                        let on = match (&suggestion.path, og_primary_path) {
                            (Some(path), Some(primary)) if path != primary => {
                                // We only include the file path when it is different to the
                                // primary file.
                                //
                                // `at $DIR/file.txt, on line LL, column CC: label`
                                //  ^^^^^^^^^^^^^^^^^
                                buffer.append(line, " at ", ElementStyle::NoStyle);
                                buffer.append(line, path, ElementStyle::NoStyle);
                                buffer.append(line, ",", ElementStyle::NoStyle);
                                "on"
                            }
                            _ => "at",
                        };
                        if next_is_suggestion {
                            // We have multiple suggestions. We will render on their own line, first
                            // the message, then the position, and finally each of the suggestions.
                            //
                            // help: suggestion message
                            //  at line LL, column CC, add one of
                            //   first suggestion
                            //   second suggestion
                            buffer.append(
                                line,
                                &format!(" {on} line {}, column {col}, add one of", lo.line),
                                ElementStyle::NoStyle,
                            );
                            line += 1;
                        } else {
                            // We have a single suggestion. We will render first the message, then
                            // the position followed by the suggestion on the next line.
                            //
                            // help: suggestion message
                            //  at line LL, column CC, add `addition`
                            buffer.append(
                                line,
                                &format!(" {on} line {}, column {col}", lo.line),
                                ElementStyle::NoStyle,
                            );
                            if !replacement.trim().is_empty() {
                                // If it is a removal, we shorten the output.
                                //
                                // help: suggestion message to remove something
                                //  at line LL, column CC
                                buffer.append(line, ", add ", ElementStyle::NoStyle);
                            }
                        }
                    }

                    if next_is_suggestion || last_suggestion_path.is_some() {
                        // Multiple suggestions.
                        buffer.append(line, &format!("  {replacement}"), ElementStyle::NoStyle);
                    } else if !replacement.trim().is_empty() {
                        // Single addition suggestion
                        buffer.append(line, &format!("`{replacement}`"), ElementStyle::NoStyle);
                    }
                    line += 1;

                    last_suggestion_path = Some(suggestion.path.as_ref());
                }
                PreProcessedElement::Origin(origin) => {
                    buffer.append(line, " at ", ElementStyle::NoStyle);
                    buffer.append(line, &origin.path, ElementStyle::NoStyle);
                    if let Some(origin_line) = origin.line {
                        buffer.append(
                            line,
                            &format!(", on line {origin_line}"),
                            ElementStyle::NoStyle,
                        );
                        if let Some(col) = origin.char_column {
                            buffer.append(line, &format!(", column {col}"), ElementStyle::NoStyle);
                        }
                    }
                    line += 1;
                }
                PreProcessedElement::Padding(_) => {}
            }
        }

        buffer.render(&group.primary_level, &renderer.stylesheet, &mut output)?;
        if iter.peek().is_some() {
            output.push('\n');
        }
    }

    Ok(output)
}

fn render_title(
    title: &dyn MessageOrTitle,
    line: &mut usize,
    buffer: &mut StyledBuffer,
    style: ElementStyle,
) {
    let mut label_width = 0;

    if title.level().name != Some(None) {
        // error EXXXX: message
        // ^^^^^
        buffer.append(
            *line,
            title.level().as_str(),
            ElementStyle::Level(title.level().level),
        );
        label_width += title.level().as_str().len();

        if let Some(Id {
            id: Some(id),
            url: _,
        }) = &title.id()
        {
            // error EXXXX: message
            //       ^^^^^
            buffer.append(*line, " ", ElementStyle::NoStyle);
            buffer.append(*line, id, ElementStyle::NoStyle);
            label_width += 1 + id.len();
        }
        // error EXXXX: message
        //            ^
        buffer.append(*line, ": ", ElementStyle::NoStyle);
        label_width += 2;
    }
    let padding = " ".repeat(label_width);

    // error EXXXX: message
    //              ^^^^^^^
    let title_str = title.text();
    for (i, text) in title_str.split('\n').enumerate() {
        if i != 0 {
            buffer.append(*line, &padding, ElementStyle::NoStyle);
        }
        buffer.append(*line, text, style);
        *line += 1;
    }
}
