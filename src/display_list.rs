//! `DisplayList` is an intermittent structure which converts the Snippet structure
//! into a list of lines that resemble the final output.
//!
//! # Example:
//!
//! ```
//! use annotate_snippets::snippet::{Snippet, Slice, Annotation, TitleAnnotation, AnnotationType};
//! use annotate_snippets::display_list::{DisplayList, DisplayLine};
//!
//! let snippet = Snippet {
//!   slice: Slice {
//!     source: r#"id: Some()"#.to_string(),
//!     line_start: 145,
//!     origin: Some("src/display_list.rs".to_string())
//!   },
//!   title: Some(TitleAnnotation {
//!       id: Some("E0061".to_string()),
//!       label: Some("this function takes 1 parameter but 0 parameters were supplied".to_string()),
//!       annotation_type: AnnotationType::Error,
//!   }),
//!   main_annotation_pos: Some(0),
//!   fold: Some(false),
//!   annotations: vec![
//!     Annotation {
//!       label: Some("expected 1 parameter".to_string()),
//!       annotation_type: AnnotationType::Error,
//!       range: Some((19, 23))
//!     }
//!   ]
//! };
//! assert_eq!(DisplayList::from(snippet), DisplayList {
//!     body: vec![
//!       DisplayLine::Raw("error[E0061]: this function takes 1 parameter but 0 parameters were supplied".to_string()),
//!       DisplayLine::Raw("  --> src/display_list.rs:52:1".to_string()),
//!       DisplayLine::EmptySource,
//!       DisplayLine::Source {
//!           lineno: 145,
//!           inline_marks: vec![],
//!           content: "id: Some()".to_string()
//!       },
//!       DisplayLine::EmptySource
//!     ]
//! });
//! ```
use snippet::{AnnotationType, Snippet};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct DisplayList {
    pub body: Vec<DisplayLine>,
}

fn format_header(snippet: &Snippet) -> Vec<DisplayLine> {
    let mut header = vec![];

    if let Some(ref annotation) = snippet.title {
        let annotation_type = match annotation.annotation_type {
            AnnotationType::Error => "error",
            AnnotationType::Warning => "warning",
        };
        let id = annotation.id.clone().unwrap_or("E0000".to_string());
        let label = annotation.label.clone().unwrap_or("".to_string());
        header.push(DisplayLine::Raw(format!(
            "{}[{}]: {}",
            annotation_type, id, label
        )));
    }

    let main_annotation = snippet
        .main_annotation_pos
        .and_then(|pos| snippet.annotations.get(pos));

    if let Some(_annotation) = main_annotation {
        let path = snippet.slice.origin.clone().unwrap_or("".to_string());
        let row = 52;
        let col = 1;
        header.push(DisplayLine::Raw(format!("  --> {}:{}:{}", path, row, col)));
    }
    header
}

fn format_body(mut snippet: Snippet) -> Vec<DisplayLine> {
    let mut body = vec![];

    let mut current_line = snippet.slice.line_start;
    let mut current_index = 0;
    let mut line_index_ranges = vec![];

    for line in snippet.slice.source.lines() {
        body.push(DisplayLine::Source {
            lineno: current_line,
            inline_marks: vec![],
            content: line.to_string(),
        });
        let line_length = line.chars().count() + 1;
        line_index_ranges.push((current_index, current_index + line_length));
        current_line += 1;
        current_index += line_length + 1;
    }

    let mut annotation_line_count = 0;
    for idx in 0..body.len() {
        let (line_start, line_end) = line_index_ranges[idx];
        snippet.annotations.drain_filter(|annotation| {
            let body_idx = idx + annotation_line_count;
            match annotation.range {
                Some((start, _)) if start > line_end => false,
                Some((start, end)) if start >= line_start && end <= line_end => {
                    let range = (start - line_start, end - line_start);
                    body.insert(
                        body_idx + 1,
                        DisplayLine::Annotation {
                            inline_marks: vec![],
                            range,
                            label: annotation.label.clone().unwrap_or("".to_string()),
                            annotation_type: DisplayAnnotationType::from(
                                annotation.annotation_type,
                            ),
                        },
                    );
                    annotation_line_count += 1;
                    true
                }
                Some((start, end))
                    if start >= line_start && start <= line_end && end > line_end =>
                {
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
                            DisplayLine::Annotation {
                                inline_marks: vec![DisplayMark::AnnotationThrough],
                                range,
                                label: annotation.label.clone().unwrap_or("".to_string()),
                                annotation_type: DisplayAnnotationType::MultilineStart,
                            },
                        );
                        annotation_line_count += 1;
                    }
                    false
                }
                Some((start, end)) if start < line_start && end > line_end => {
                    if let DisplayLine::Source {
                        ref mut inline_marks,
                        ..
                    } = body[body_idx]
                    {
                        inline_marks.push(DisplayMark::AnnotationThrough);
                    }
                    false
                }
                Some((start, end))
                    if start < line_start && end >= line_start && end <= line_end =>
                {
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
                        DisplayLine::Annotation {
                            inline_marks: vec![DisplayMark::AnnotationThrough],
                            range,
                            label: annotation.label.clone().unwrap_or("".to_string()),
                            annotation_type: DisplayAnnotationType::MultilineEnd,
                        },
                    );
                    annotation_line_count += 1;
                    true
                }
                _ => false,
            }
        });
    }

    if snippet.fold.unwrap_or(false) {
        let mut no_annotation_lines_counter = 0;
        let mut idx = 0;
        while idx < body.len() {
            match body[idx] {
                DisplayLine::Annotation { .. } => {
                    if no_annotation_lines_counter > 10 {
                        let fold_start = idx - no_annotation_lines_counter + 5;
                        let fold_end = idx - 2;
                        let fold_len = fold_end - fold_start;

                        let slice = &[DisplayLine::Fold];

                        body.splice(fold_start..fold_end, slice.iter().cloned());
                        idx -= fold_len - 1;
                    }
                    no_annotation_lines_counter += 0;
                }
                _ => no_annotation_lines_counter += 1,
            }
            idx += 1;
        }
    }

    body.insert(0, DisplayLine::EmptySource);
    body.push(DisplayLine::EmptySource);
    body
}

impl From<Snippet> for DisplayList {
    fn from(snippet: Snippet) -> Self {
        let header = format_header(&snippet);
        let body = format_body(snippet);

        DisplayList {
            body: [&header[..], &body[..]].concat(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayLine {
    Raw(String),
    EmptySource,
    Source {
        lineno: usize,
        inline_marks: Vec<DisplayMark>,
        content: String,
    },
    Annotation {
        inline_marks: Vec<DisplayMark>,
        range: (usize, usize),
        label: String,
        annotation_type: DisplayAnnotationType,
    },
    Fold,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayAnnotationType {
    Error,
    Warning,
    MultilineStart,
    MultilineEnd,
}

impl From<AnnotationType> for DisplayAnnotationType {
    fn from(at: AnnotationType) -> Self {
        match at {
            AnnotationType::Error => DisplayAnnotationType::Error,
            AnnotationType::Warning => DisplayAnnotationType::Warning,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMark {
    AnnotationThrough,
    AnnotationStart,
}

impl fmt::Display for DisplayMark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisplayMark::AnnotationThrough => write!(f, "|"),
            DisplayMark::AnnotationStart => write!(f, "/"),
        }
    }
}
