//! `DisplayList` is an intermittent structure which converts the Snippet structure
//! into a list of lines that resemble the final output.
//!
//! # Example:
//!
//! ```
//! use annotate_snippets::snippet::{Snippet, Slice, Annotation, TitleAnnotation, AnnotationType};
//! use annotate_snippets::display_list::{DisplayList, DisplayLine, DisplayAnnotationType,
//! DisplaySnippetType};
//!
//! let snippet = Snippet {
//!   slice: Slice {
//!     source: "id: Option<>,\nlabel: Option<String>".to_string(),
//!     line_start: 145,
//!     origin: Some("src/display_list.rs".to_string())
//!   },
//!   title: Some(TitleAnnotation {
//!       id: Some("E0061".to_string()),
//!       label: Some("this function takes 1 parameter but 0 parameters were supplied".to_string()),
//!       annotation_type: AnnotationType::Error,
//!   }),
//!   fold: Some(false),
//!   annotations: vec![
//!     Annotation {
//!       label: "expected 1 parameter".to_string(),
//!       annotation_type: AnnotationType::Error,
//!       range: (4, 12)
//!     }
//!   ]
//! };
//! assert_eq!(DisplayList::from(snippet).body, vec![
//!     DisplayLine::Description {
//!         snippet_type: DisplaySnippetType::Error,
//!         id: "E0061".to_string(),
//!         label: "this function takes 1 parameter but 0 parameters were supplied".to_string(),
//!     },
//!     DisplayLine::Origin {
//!         path: "src/display_list.rs".to_string(),
//!         row: 145,
//!         col: 4,
//!     },
//!     DisplayLine::EmptySource,
//!     DisplayLine::Source {
//!         lineno: 145,
//!         inline_marks: vec![],
//!         content: "id: Option<>,".to_string(),
//!         range: (0, 14)
//!     },
//!     DisplayLine::Annotation {
//!         label: Some("expected 1 parameter".to_string()),
//!         range: (4, 12),
//!         inline_marks: vec![],
//!         annotation_type: DisplayAnnotationType::Error,
//!     },
//!     DisplayLine::Source {
//!         lineno: 146,
//!         inline_marks: vec![],
//!         content: "label: Option<String>".to_string(),
//!         range: (15, 37)
//!     },
//!     DisplayLine::EmptySource
//! ]);
//! ```
use snippet::{AnnotationType, Snippet};

pub struct DisplayList {
    pub body: Vec<DisplayLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayLine {
    Description {
        snippet_type: DisplaySnippetType,
        id: String,
        label: String,
    },
    Origin {
        path: String,
        row: usize,
        col: usize,
    },
    EmptySource,
    Source {
        lineno: usize,
        inline_marks: Vec<DisplayMark>,
        content: String,
        range: (usize, usize),
    },
    Annotation {
        inline_marks: Vec<DisplayMark>,
        range: (usize, usize),
        label: Option<String>,
        annotation_type: DisplayAnnotationType,
    },
    Fold,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayAnnotationType {
    Error,
    Warning,
    MultilineStart,
    MultilineEnd,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayMark {
    AnnotationThrough,
    AnnotationStart,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplaySnippetType {
    Error,
    Warning,
}

// Formatting

fn format_header(snippet: &Snippet, body: &[DisplayLine]) -> Vec<DisplayLine> {
    let mut header = vec![];

    if let Some(ref annotation) = snippet.title {
        let id = annotation.id.clone().unwrap_or("".to_string());
        let label = annotation.label.clone().unwrap_or("".to_string());
        header.push(DisplayLine::Description {
            snippet_type: DisplaySnippetType::from(annotation.annotation_type),
            id,
            label,
        })
    }

    let main_annotation = snippet.annotations.get(0);

    if let Some(annotation) = main_annotation {
        let mut col = 1;
        let mut row = snippet.slice.line_start;

        for idx in 0..body.len() {
            if let DisplayLine::Source { range, .. } = body[idx] {
                if annotation.range.0 >= range.0 && annotation.range.0 <= range.1 {
                    col = annotation.range.0 - range.0;
                    break;
                }
                row += 1;
            }
        }
        if let Some(ref path) = snippet.slice.origin {
            header.push(DisplayLine::Origin {
                path: path.to_string(),
                row,
                col,
            });
        }
    }
    header
}

fn format_body(snippet: &Snippet) -> Vec<DisplayLine> {
    let mut body = vec![];

    let mut current_line = snippet.slice.line_start;
    let mut current_index = 0;
    let mut line_index_ranges = vec![];

    for line in snippet.slice.source.lines() {
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
    let mut annotations = snippet.annotations.clone();
    for idx in 0..body.len() {
        let (line_start, line_end) = line_index_ranges[idx];
        annotations.drain_filter(|annotation| {
            let body_idx = idx + annotation_line_count;
            match annotation.range {
                (start, _) if start > line_end => false,
                (start, end) if start >= line_start && end <= line_end => {
                    let range = (start - line_start, end - line_start);
                    body.insert(
                        body_idx + 1,
                        DisplayLine::Annotation {
                            inline_marks: vec![],
                            range,
                            label: Some(annotation.label.clone()),
                            annotation_type: DisplayAnnotationType::from(
                                annotation.annotation_type,
                            ),
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
                            DisplayLine::Annotation {
                                inline_marks: vec![],
                                range,
                                label: None,
                                annotation_type: DisplayAnnotationType::MultilineStart,
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
                        DisplayLine::Annotation {
                            inline_marks: vec![DisplayMark::AnnotationThrough],
                            range,
                            label: Some(annotation.label.clone()),
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
    if let Some(DisplayLine::Source { .. }) = body.last() {
        body.push(DisplayLine::EmptySource);
    }
    body
}

impl From<Snippet> for DisplayList {
    fn from(snippet: Snippet) -> Self {
        let body = format_body(&snippet);
        let header = format_header(&snippet, &body);

        Self {
            body: vec![&header[..], &body[..]].concat(),
        }
    }
}

impl From<AnnotationType> for DisplayAnnotationType {
    fn from(at: AnnotationType) -> Self {
        match at {
            AnnotationType::Error => DisplayAnnotationType::Error,
            AnnotationType::Warning => DisplayAnnotationType::Warning,
        }
    }
}

impl From<AnnotationType> for DisplaySnippetType {
    fn from(at: AnnotationType) -> Self {
        match at {
            AnnotationType::Error => DisplaySnippetType::Error,
            AnnotationType::Warning => DisplaySnippetType::Warning,
        }
    }
}
