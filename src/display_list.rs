use snippet::{Annotation, AnnotationType, Slice, Snippet};

pub struct DisplayList {
    pub body: Vec<DisplayLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayLine {
    Annotation {
        label: Vec<DisplayTextFragment>,
        id: Option<String>,
        aligned: bool,
        annotation_type: DisplayAnnotationType,
    },
    Origin {
        path: String,
        pos: Option<(usize, usize)>,
        header_type: DisplayHeaderType,
    },
    EmptySource,
    Source {
        lineno: usize,
        inline_marks: Vec<DisplayMark>,
        content: String,
        range: (usize, usize),
    },
    SourceAnnotation {
        inline_marks: Vec<DisplayMark>,
        range: (usize, usize),
        label: Vec<DisplayTextFragment>,
        annotation_type: DisplayAnnotationType,
        annotation_part: DisplayAnnotationPart,
    },
    Fold {
        inline_marks: Vec<DisplayMark>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayTextFragment {
    pub content: String,
    pub style: DisplayTextStyle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayTextStyle {
    Regular,
    Emphasis,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayAnnotationPart {
    Singleline,
    MultilineStart,
    MultilineEnd,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayMark {
    AnnotationThrough,
    AnnotationStart,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayAnnotationType {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayHeaderType {
    Initial,
    Continuation,
}

// Formatting

fn format_label(label: Option<&str>, style: Option<DisplayTextStyle>) -> Vec<DisplayTextFragment> {
    let mut result = vec![];
    if let Some(label) = label {
        let elements: Vec<&str> = label.split("__").collect();
        let mut idx = 0;
        for element in elements {
            let element_style = match style {
                Some(s) => s,
                None => if idx % 2 == 0 {
                    DisplayTextStyle::Regular
                } else {
                    DisplayTextStyle::Emphasis
                },
            };
            result.push(DisplayTextFragment {
                content: element.to_string(),
                style: element_style,
            });
            idx += 1;
        }
    }
    return result;
}

fn format_title(annotation: &Annotation) -> DisplayLine {
    let label = annotation.label.clone().unwrap_or("".to_string());
    DisplayLine::Annotation {
        annotation_type: DisplayAnnotationType::from(annotation.annotation_type),
        id: annotation.id.clone(),
        aligned: false,
        label: format_label(Some(&label), Some(DisplayTextStyle::Emphasis)),
    }
}

fn format_annotation(annotation: &Annotation) -> DisplayLine {
    let label = annotation.label.clone().unwrap_or("".to_string());
    DisplayLine::Annotation {
        annotation_type: DisplayAnnotationType::from(annotation.annotation_type),
        aligned: true,
        id: None,
        label: format_label(Some(&label), None),
    }
}

fn format_slice(slice: &Slice, is_first: bool, has_footer: bool) -> Vec<DisplayLine> {
    let mut body = format_body(slice, has_footer);
    let mut result = vec![];

    let header = format_header(slice, &body, is_first);
    if let Some(header) = header {
        result.push(header);
    }
    result.append(&mut body);
    result
}

fn format_header(slice: &Slice, body: &[DisplayLine], is_first: bool) -> Option<DisplayLine> {
    let main_annotation = slice.annotations.get(0);

    let display_header = if is_first {
        DisplayHeaderType::Initial
    } else {
        DisplayHeaderType::Continuation
    };

    if let Some(annotation) = main_annotation {
        let mut col = 1;
        let mut row = slice.line_start;

        for idx in 0..body.len() {
            if let DisplayLine::Source { range, .. } = body[idx] {
                if annotation.range.0 >= range.0 && annotation.range.0 <= range.1 {
                    col = annotation.range.0 - range.0;
                    break;
                }
                row += 1;
            }
        }
        if let Some(ref path) = slice.origin {
            return Some(DisplayLine::Origin {
                path: path.to_string(),
                pos: Some((row, col)),
                header_type: display_header,
            });
        }
    } else {
        if let Some(ref path) = slice.origin {
            return Some(DisplayLine::Origin {
                path: path.to_string(),
                pos: None,
                header_type: display_header,
            });
        }
    }
    None
}

fn fold_body(body: &[DisplayLine]) -> Vec<DisplayLine> {
    let mut new_body = vec![];

    let mut no_annotation_lines_counter = 0;
    let mut idx = 0;

    while idx < body.len() {
        match body[idx] {
            DisplayLine::SourceAnnotation {
                ref inline_marks, ..
            } => {
                if no_annotation_lines_counter > 10 {
                    let fold_start = idx - no_annotation_lines_counter;
                    let fold_end = idx;
                    for i in fold_start..fold_start + 4 {
                        new_body.push(body[i].clone());
                    }
                    new_body.push(DisplayLine::Fold {
                        inline_marks: inline_marks.clone(),
                    });
                    for i in fold_end - 2..fold_end {
                        new_body.push(body[i].clone());
                    }
                } else {
                    let start = idx - no_annotation_lines_counter;
                    for i in start..idx {
                        new_body.push(body[i].clone());
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

    return new_body;
}

fn format_body(slice: &Slice, has_footer: bool) -> Vec<DisplayLine> {
    let mut body = vec![];

    let mut current_line = slice.line_start;
    let mut current_index = 0;
    let mut line_index_ranges = vec![];

    for line in slice.source.lines() {
        let line_length = line.chars().count() + 1;
        let line_range = (current_index, current_index + line_length);
        body.push(DisplayLine::Source {
            lineno: current_line,
            inline_marks: vec![],
            content: line.to_string(),
            range: line_range,
        });
        line_index_ranges.push(line_range);
        current_line += 1;
        current_index += line_length + 1;
    }

    let mut annotation_line_count = 0;
    let mut annotations = slice.annotations.clone();
    for idx in 0..body.len() {
        let (line_start, line_end) = line_index_ranges[idx];
        annotations.drain_filter(|annotation| {
            let body_idx = idx + annotation_line_count;
            match annotation.range {
                (start, _) if start > line_end => false,
                (start, end) if start >= line_start && end <= line_end + 1 => {
                    let range = (start - line_start, end - line_start);
                    body.insert(
                        body_idx + 1,
                        DisplayLine::SourceAnnotation {
                            inline_marks: vec![],
                            range,
                            label: format_label(Some(&annotation.label), None),
                            annotation_type: DisplayAnnotationType::from(
                                annotation.annotation_type,
                            ),
                            annotation_part: DisplayAnnotationPart::Singleline,
                        },
                    );
                    annotation_line_count += 1;
                    true
                }
                (start, end) if start >= line_start && start <= line_end && end > line_end => {
                    if start - line_start == 0 {
                        if let DisplayLine::Source {
                            ref mut inline_marks,
                            ..
                        } = body[body_idx]
                        {
                            inline_marks.push(DisplayMark::AnnotationStart);
                        }
                    } else {
                        let range = (start - line_start, start - line_start + 1);
                        body.insert(
                            body_idx + 1,
                            DisplayLine::SourceAnnotation {
                                inline_marks: vec![],
                                range,
                                label: vec![],
                                annotation_type: DisplayAnnotationType::from(
                                    annotation.annotation_type,
                                ),
                                annotation_part: DisplayAnnotationPart::MultilineStart,
                            },
                        );
                        annotation_line_count += 1;
                    }
                    false
                }
                (start, end) if start < line_start && end > line_end => {
                    if let DisplayLine::Source {
                        ref mut inline_marks,
                        ..
                    } = body[body_idx]
                    {
                        inline_marks.push(DisplayMark::AnnotationThrough);
                    }
                    false
                }
                (start, end) if start < line_start && end >= line_start && end <= line_end => {
                    if let DisplayLine::Source {
                        ref mut inline_marks,
                        ..
                    } = body[body_idx]
                    {
                        inline_marks.push(DisplayMark::AnnotationThrough);
                    }
                    let range = (end - line_start, end - line_start + 1);
                    body.insert(
                        body_idx + 1,
                        DisplayLine::SourceAnnotation {
                            inline_marks: vec![DisplayMark::AnnotationThrough],
                            range,
                            label: format_label(Some(&annotation.label), None),
                            annotation_type: DisplayAnnotationType::from(
                                annotation.annotation_type,
                            ),
                            annotation_part: DisplayAnnotationPart::MultilineEnd,
                        },
                    );
                    annotation_line_count += 1;
                    true
                }
                _ => false,
            }
        });
    }

    if slice.fold {
        body = fold_body(&body);
    }

    body.insert(0, DisplayLine::EmptySource);
    if has_footer {
        body.push(DisplayLine::EmptySource);
    } else if let Some(DisplayLine::Source { .. }) = body.last() {
        body.push(DisplayLine::EmptySource);
    }
    body
}

impl From<Snippet> for DisplayList {
    fn from(snippet: Snippet) -> Self {
        let mut body = vec![];
        if let Some(annotation) = snippet.title {
            body.push(format_title(&annotation));
        }

        let mut slice_idx = 0;
        for slice in snippet.slices {
            body.append(&mut format_slice(
                &slice,
                slice_idx == 0,
                snippet.footer.is_some(),
            ));
            slice_idx += 1;
        }
        if let Some(annotation) = snippet.footer {
            body.push(format_annotation(&annotation));
        }

        Self { body }
    }
}

impl From<AnnotationType> for DisplayAnnotationType {
    fn from(at: AnnotationType) -> Self {
        match at {
            AnnotationType::Error => DisplayAnnotationType::Error,
            AnnotationType::Warning => DisplayAnnotationType::Warning,
            AnnotationType::Note => DisplayAnnotationType::Note,
            AnnotationType::Help => DisplayAnnotationType::Help,
        }
    }
}
