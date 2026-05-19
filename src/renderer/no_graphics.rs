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
    let mut output = String::new();
    let group_len = groups.len();
    let (_max_line_num, _og_primary_path, groups) = pre_process(groups);

    for (
        g,
        PreProcessedGroup {
            group,
            elements,
            primary_path: _,
            max_depth: _,
        },
    ) in groups.into_iter().enumerate()
    {
        let mut buffer = StyledBuffer::new();
        let mut line = 0;
        if let Some(title) = &group.title {
            render_title(title, &mut line, &mut buffer);
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

                        let target = line - 1;
                        buffer.append(target, " or ", ElementStyle::NoStyle);
                        buffer.append(target, text, ElementStyle::NoStyle);
                        last_suggestion_path = None;
                    } else {
                        last_suggestion_path = None;
                        render_title(message, &mut line, &mut buffer);
                    }
                }
                PreProcessedElement::Cause((snippet, _, _)) => {
                    let sm = source_map::SourceMap::new(&snippet.source, snippet.line_start);

                    let mut annotations = snippet.markers.iter().collect::<Vec<_>>();
                    annotations.sort_by_key(|a| (Reverse(a.kind.is_primary()), a.span.start));

                    for (i, annotation) in annotations.iter().enumerate() {
                        let label = annotation.label.as_ref().filter(|s| !s.is_empty());
                        if i > 0 && label.is_none() {
                            continue;
                        }
                        let (lo, hi) =
                            sm.span_to_locations(annotation.span.start..annotation.span.end);
                        if i == 0 {
                            if let Some(path) = &snippet.path {
                                buffer.append(line, "at ", ElementStyle::NoStyle);
                                buffer.append(line, path, ElementStyle::NoStyle);
                                buffer.append(line, ",", ElementStyle::NoStyle);
                            }
                        }

                        let (prefix, suffix) = if lo.line == hi.line {
                            ("on", String::new())
                        } else {
                            (
                                "from",
                                format!(" to line {}, column {}", hi.line, hi.char + 1),
                            )
                        };

                        buffer.append(
                            line,
                            &format!(" {prefix} line {}, column {}{suffix}", lo.line, lo.char + 1),
                            ElementStyle::NoStyle,
                        );

                        if let Some(label) = label {
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
                    let target = line - 1;
                    if last_suggestion_path.is_none() {
                        let (lo, _) =
                            sm.span_to_locations(first_patch.span.start..first_patch.span.end);
                        let col = lo.char.max(1);
                        let separator = if no_preceding_line { "" } else { ": " };
                        if next_is_suggestion {
                            buffer.append(
                                target,
                                &format!(
                                    "{separator}at line {}, column {col}, add one of ",
                                    lo.line
                                ),
                                ElementStyle::NoStyle,
                            );
                        } else {
                            buffer.append(
                                target,
                                &format!("{separator}at line {}, column {col}, add ", lo.line),
                                ElementStyle::NoStyle,
                            );
                        }
                    }

                    let replacement = first_patch.replacement.trim_end_matches('\n');
                    buffer.append(target, &format!("`{replacement}`"), ElementStyle::NoStyle);

                    if next_is_suggestion {
                        buffer.append(target, ", ", ElementStyle::NoStyle);
                    }
                    last_suggestion_path = Some(suggestion.path.as_ref());
                }
                PreProcessedElement::Origin(origin) => {
                    buffer.append(line, "at ", ElementStyle::NoStyle);
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
        if g != group_len - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

fn render_title(title: &dyn MessageOrTitle, line: &mut usize, buffer: &mut StyledBuffer) {
    let mut label_width = 0;

    if title.level().name != Some(None) {
        buffer.append(*line, title.level().as_str(), ElementStyle::NoStyle);
        label_width += title.level().as_str().len();

        if let Some(Id {
            id: Some(id),
            url: _,
        }) = &title.id()
        {
            buffer.append(*line, " ", ElementStyle::NoStyle);
            buffer.append(*line, id, ElementStyle::NoStyle);
            label_width += 1 + id.len();
        }
        buffer.append(*line, ": ", ElementStyle::NoStyle);
        label_width += 2;
    }
    let padding = " ".repeat(label_width);

    let title_str = title.text();
    for (i, text) in title_str.split('\n').enumerate() {
        if i != 0 {
            buffer.append(*line, &padding, ElementStyle::NoStyle);
        }
        buffer.append(*line, text, ElementStyle::NoStyle);
        *line += 1;
    }
}
