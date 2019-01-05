//! Trait for converting `Snippet` to `DisplayList`.
use super::*;
use crate::snippet;

fn format_label(label: Option<&str>, style: Option<DisplayTextStyle>) -> Vec<DisplayTextFragment> {
    let mut result = vec![];
    if let Some(label) = label {
        let elements: Vec<&str> = label.split("__").collect();
        for (idx, element) in elements.iter().enumerate() {
            let element_style = match style {
                Some(s) => s,
                None => {
                    if idx % 2 == 0 {
                        DisplayTextStyle::Regular
                    } else {
                        DisplayTextStyle::Emphasis
                    }
                }
            };
            result.push(DisplayTextFragment {
                content: element.to_string(),
                style: element_style,
            });
        }
    }
    result
}

fn format_title(annotation: &snippet::Annotation) -> DisplayLine {
    let label = annotation.label.clone().unwrap_or_default();
    DisplayLine::Raw(DisplayRawLine::Annotation {
        annotation: Annotation {
            annotation_type: DisplayAnnotationType::from(annotation.annotation_type),
            id: annotation.id.clone(),
            label: format_label(Some(&label), Some(DisplayTextStyle::Emphasis)),
        },
        source_aligned: false,
        continuation: false,
    })
}

fn format_annotation(annotation: &snippet::Annotation) -> Vec<DisplayLine> {
    let mut result = vec![];
    let label = annotation.label.clone().unwrap_or_default();
    for (i, line) in label.lines().enumerate() {
        result.push(DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::from(annotation.annotation_type),
                id: None,
                label: format_label(Some(line), None),
            },
            source_aligned: true,
            continuation: i != 0,
        }));
    }
    result
}

fn format_slice(slice: &snippet::Slice, is_first: bool, has_footer: bool) -> Vec<DisplayLine> {
    let mut body = format_body(slice, has_footer);
    let mut result = vec![];

    let header = format_header(slice, &body, is_first);
    if let Some(header) = header {
        result.push(header);
    }
    result.append(&mut body);
    result
}

fn format_header(
    slice: &snippet::Slice,
    body: &[DisplayLine],
    is_first: bool,
) -> Option<DisplayLine> {
    let main_annotation = slice.annotations.get(0);

    let display_header = if is_first {
        DisplayHeaderType::Initial
    } else {
        DisplayHeaderType::Continuation
    };

    if let Some(annotation) = main_annotation {
        let mut col = 1;
        let mut row = slice.line_start;

        for item in body.iter() {
            if let DisplayLine::Source {
                line: DisplaySourceLine::Content { range, .. },
                ..
            } = item
            {
                if annotation.range.0 >= range.0 && annotation.range.0 <= range.1 {
                    col = annotation.range.0 - range.0;
                    break;
                }
                row += 1;
            }
        }
        if let Some(ref path) = slice.origin {
            return Some(DisplayLine::Raw(DisplayRawLine::Origin {
                path: path.to_string(),
                pos: Some((row, col)),
                header_type: display_header,
            }));
        }
    }
    if let Some(ref path) = slice.origin {
        return Some(DisplayLine::Raw(DisplayRawLine::Origin {
            path: path.to_string(),
            pos: None,
            header_type: display_header,
        }));
    }
    None
}

fn fold_body(body: &[DisplayLine]) -> Vec<DisplayLine> {
    let mut new_body = vec![];

    let mut no_annotation_lines_counter = 0;
    let mut idx = 0;

    while idx < body.len() {
        match body[idx] {
            DisplayLine::Source {
                line: DisplaySourceLine::Annotation { .. },
                ref inline_marks,
                ..
            } => {
                if no_annotation_lines_counter > 2 {
                    let fold_start = idx - no_annotation_lines_counter;
                    let fold_end = idx;
                    let pre_len = if no_annotation_lines_counter > 8 {
                        4
                    } else {
                        0
                    };
                    let post_len = if no_annotation_lines_counter > 8 {
                        2
                    } else {
                        1
                    };
                    for item in body.iter().take(fold_start + pre_len).skip(fold_start) {
                        new_body.push(item.clone());
                    }
                    new_body.push(DisplayLine::Fold {
                        inline_marks: inline_marks.clone(),
                    });
                    for item in body.iter().take(fold_end).skip(fold_end - post_len) {
                        new_body.push(item.clone());
                    }
                } else {
                    let start = idx - no_annotation_lines_counter;
                    for item in body.iter().take(idx).skip(start) {
                        new_body.push(item.clone());
                    }
                }
                no_annotation_lines_counter = 0;
            }
            DisplayLine::Source { .. } => {
                no_annotation_lines_counter += 1;
                idx += 1;
                continue;
            }
            _ => {
                no_annotation_lines_counter += 1;
            }
        }
        new_body.push(body[idx].clone());
        idx += 1;
    }

    new_body
}

fn format_body(slice: &snippet::Slice, has_footer: bool) -> Vec<DisplayLine> {
    let mut body = vec![];

    let mut current_line = slice.line_start;
    let mut current_index = 0;
    let mut line_index_ranges = vec![];

    for line in slice.source.lines() {
        let line_length = line.chars().count() + 1;
        let line_range = (current_index, current_index + line_length);
        body.push(DisplayLine::Source {
            lineno: Some(current_line),
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: line.to_string(),
                range: line_range,
            },
        });
        line_index_ranges.push(line_range);
        current_line += 1;
        current_index += line_length + 1;
    }

    let mut annotation_line_count = 0;
    let mut annotations = slice.annotations.clone();
    for idx in 0..body.len() {
        let (line_start, line_end) = line_index_ranges[idx];
        // It would be nice to use filter_drain here once it's stable.
        annotations = annotations
            .into_iter()
            .filter(|annotation| {
                let body_idx = idx + annotation_line_count;
                let annotation_type = match annotation.annotation_type {
                    snippet::AnnotationType::Error => DisplayAnnotationType::None,
                    snippet::AnnotationType::Warning => DisplayAnnotationType::None,
                    _ => DisplayAnnotationType::from(annotation.annotation_type),
                };
                match annotation.range {
                    (start, _) if start > line_end => true,
                    (start, end) if start >= line_start && end <= line_end + 1 => {
                        let range = (start - line_start, end - line_start);
                        body.insert(
                            body_idx + 1,
                            DisplayLine::Source {
                                lineno: None,
                                inline_marks: vec![],
                                line: DisplaySourceLine::Annotation {
                                    annotation: Annotation {
                                        annotation_type,
                                        id: None,
                                        label: format_label(Some(&annotation.label), None),
                                    },
                                    range,
                                    annotation_type: DisplayAnnotationType::from(
                                        annotation.annotation_type,
                                    ),
                                    annotation_part: DisplayAnnotationPart::Standalone,
                                },
                            },
                        );
                        annotation_line_count += 1;
                        false
                    }
                    (start, end) if start >= line_start && start <= line_end && end > line_end => {
                        if start - line_start == 0 {
                            if let DisplayLine::Source {
                                ref mut inline_marks,
                                ..
                            } = body[body_idx]
                            {
                                inline_marks.push(DisplayMark {
                                    mark_type: DisplayMarkType::AnnotationStart,
                                    annotation_type: DisplayAnnotationType::from(
                                        annotation.annotation_type,
                                    ),
                                });
                            }
                        } else {
                            let range = (start - line_start, start - line_start + 1);
                            body.insert(
                                body_idx + 1,
                                DisplayLine::Source {
                                    lineno: None,
                                    inline_marks: vec![],
                                    line: DisplaySourceLine::Annotation {
                                        annotation: Annotation {
                                            annotation_type: DisplayAnnotationType::None,
                                            id: None,
                                            label: vec![],
                                        },
                                        range,
                                        annotation_type: DisplayAnnotationType::from(
                                            annotation.annotation_type,
                                        ),
                                        annotation_part: DisplayAnnotationPart::MultilineStart,
                                    },
                                },
                            );
                            annotation_line_count += 1;
                        }
                        true
                    }
                    (start, end) if start < line_start && end > line_end => {
                        if let DisplayLine::Source {
                            ref mut inline_marks,
                            ..
                        } = body[body_idx]
                        {
                            inline_marks.push(DisplayMark {
                                mark_type: DisplayMarkType::AnnotationThrough,
                                annotation_type: DisplayAnnotationType::from(
                                    annotation.annotation_type,
                                ),
                            });
                        }
                        true
                    }
                    (start, end) if start < line_start && end >= line_start && end <= line_end => {
                        if let DisplayLine::Source {
                            ref mut inline_marks,
                            ..
                        } = body[body_idx]
                        {
                            inline_marks.push(DisplayMark {
                                mark_type: DisplayMarkType::AnnotationThrough,
                                annotation_type: DisplayAnnotationType::from(
                                    annotation.annotation_type,
                                ),
                            });
                        }
                        let range = (end - line_start, end - line_start + 1);
                        body.insert(
                            body_idx + 1,
                            DisplayLine::Source {
                                lineno: None,
                                inline_marks: vec![DisplayMark {
                                    mark_type: DisplayMarkType::AnnotationThrough,
                                    annotation_type: DisplayAnnotationType::from(
                                        annotation.annotation_type,
                                    ),
                                }],
                                line: DisplaySourceLine::Annotation {
                                    annotation: Annotation {
                                        annotation_type,
                                        id: None,
                                        label: format_label(Some(&annotation.label), None),
                                    },
                                    range,
                                    annotation_type: DisplayAnnotationType::from(
                                        annotation.annotation_type,
                                    ),
                                    annotation_part: DisplayAnnotationPart::MultilineEnd,
                                },
                            },
                        );
                        annotation_line_count += 1;
                        false
                    }
                    _ => true,
                }
            })
            .collect();
    }

    if slice.fold {
        body = fold_body(&body);
    }

    body.insert(
        0,
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Empty,
        },
    );
    if has_footer {
        body.push(DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Empty,
        });
    } else if let Some(DisplayLine::Source { .. }) = body.last() {
        body.push(DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Empty,
        });
    }
    body
}

impl From<snippet::Snippet> for DisplayList {
    fn from(snippet: snippet::Snippet) -> Self {
        let mut body = vec![];
        if let Some(annotation) = snippet.title {
            body.push(format_title(&annotation));
        }

        for (idx, slice) in snippet.slices.iter().enumerate() {
            body.append(&mut format_slice(
                &slice,
                idx == 0,
                !snippet.footer.is_empty(),
            ));
        }

        for annotation in snippet.footer {
            body.append(&mut format_annotation(&annotation));
        }

        Self { body }
    }
}

impl From<snippet::AnnotationType> for DisplayAnnotationType {
    fn from(at: snippet::AnnotationType) -> Self {
        match at {
            snippet::AnnotationType::Error => DisplayAnnotationType::Error,
            snippet::AnnotationType::Warning => DisplayAnnotationType::Warning,
            snippet::AnnotationType::Info => DisplayAnnotationType::Info,
            snippet::AnnotationType::Note => DisplayAnnotationType::Note,
            snippet::AnnotationType::Help => DisplayAnnotationType::Help,
        }
    }
}
