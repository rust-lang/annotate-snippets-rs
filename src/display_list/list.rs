use super::annotation::Annotation;
use super::line::{DisplayLine, DisplayMark, DisplayMarkType, DisplayRawLine, DisplaySourceLine};
use crate::annotation::AnnotationType;
use crate::styles::{get_stylesheet, Stylesheet};
use crate::{Slice, Snippet, SourceAnnotation};
use std::cmp;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DisplayList<'d> {
    pub body: Vec<DisplayLine<'d>>,
}

impl<'d> DisplayList<'d> {
    pub fn fmt_with_style(
        &self,
        f: &mut fmt::Formatter<'_>,
        style: &impl Stylesheet,
    ) -> fmt::Result {
        let lineno_max = self.body.iter().rev().find_map(|line| {
            if let DisplayLine::Source {
                lineno: Some(lineno),
                ..
            } = line
            {
                Some(digits(*lineno))
            } else {
                None
            }
        });
        let inline_marks_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => cmp::max(inline_marks.len(), max),
            _ => max,
        });
        for line in &self.body {
            line.fmt_with_style(f, style, lineno_max, inline_marks_width)?
        }
        Ok(())
    }
}

fn get_header_pos(slice: &Slice) -> (Option<usize>, Option<usize>) {
    let line = slice.line_start;
    (line, None)
}

impl<'d> From<&Snippet<'d>> for DisplayList<'d> {
    fn from(snippet: &Snippet<'d>) -> Self {
        let mut body = vec![];

        if let Some(annotation) = &snippet.title {
            let label = annotation.label.unwrap_or_default();
            body.push(DisplayLine::Raw(DisplayRawLine::Annotation {
                annotation: Annotation {
                    annotation_type: AnnotationType::Error,
                    id: annotation.id,
                    label: &label,
                },
                source_aligned: false,
                continuation: false,
            }));
        }

        for slice in snippet.slices {
            let slice_dl: DisplayList = slice.into();
            body.extend(slice_dl.body);
        }
        DisplayList { body }
    }
}

impl<'d> From<&Slice<'d>> for DisplayList<'d> {
    fn from(slice: &Slice<'d>) -> Self {
        let mut body = vec![];

        if let Some(path) = slice.origin {
            body.push(DisplayLine::Raw(DisplayRawLine::Origin {
                path,
                pos: get_header_pos(slice),
            }));
        }

        body.push(DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Empty,
        });

        let mut annotations: Vec<&SourceAnnotation> = slice.annotations.iter().collect();

        // let mut current_annotation = annotations.next();
        let mut line_start_pos = 0;

        let mut i = slice.line_start.unwrap_or(1);
        for line in slice.source.lines() {
            let line_length = line.chars().count();

            let mut current_annotations = vec![];
            let mut inline_marks = vec![];

            annotations.retain(|ann| {
                if ann.range.0 >= line_start_pos && ann.range.1 <= line_start_pos + line_length {
                    // Annotation in this line
                    current_annotations.push(*ann);
                    false
                } else if ann.range.0 >= line_start_pos
                    && ann.range.0 <= line_start_pos + line_length
                {
                    // Annotation starts in this line
                    inline_marks.push(DisplayMark {
                        mark_type: DisplayMarkType::AnnotationStart,
                        annotation_type: AnnotationType::Error,
                    });
                    true
                } else if ann.range.0 < line_start_pos && ann.range.1 > line_start_pos + line_length
                {
                    // Annotation goes through this line
                    inline_marks.push(DisplayMark {
                        mark_type: DisplayMarkType::AnnotationThrough,
                        annotation_type: AnnotationType::Error,
                    });
                    true
                } else if ann.range.0 < line_start_pos
                    && ann.range.1 >= line_start_pos
                    && ann.range.1 <= line_start_pos + line_length
                {
                    // Annotation ends on this line
                    inline_marks.push(DisplayMark {
                        mark_type: DisplayMarkType::AnnotationThrough,
                        annotation_type: AnnotationType::Error,
                    });
                    current_annotations.push(*ann);
                    false
                } else {
                    true
                }
            });

            body.push(DisplayLine::Source {
                lineno: Some(i),
                inline_marks,
                line: DisplaySourceLine::Content { text: line },
            });
            for ann in current_annotations {
                let start = if ann.range.0 >= line_start_pos {
                    ann.range.0 - line_start_pos
                } else {
                    0
                };
                let inline_marks = if ann.range.0 < line_start_pos {
                    vec![DisplayMark {
                        mark_type: DisplayMarkType::AnnotationThrough,
                        annotation_type: AnnotationType::Error,
                    }]
                } else {
                    vec![]
                };
                body.push(DisplayLine::Source {
                    lineno: None,
                    inline_marks,
                    line: DisplaySourceLine::Annotation {
                        annotation: Annotation {
                            annotation_type: AnnotationType::Error,
                            id: None,
                            label: ann.label,
                        },
                        range: (start, ann.range.1 - line_start_pos),
                    },
                });
            }
            line_start_pos += line_length + 1;
            i += 1;
        }

        body.push(DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Empty,
        });

        DisplayList { body }
    }
}

fn digits(n: usize) -> usize {
    let mut n = n;
    let mut sum = 0;
    while n != 0 {
        n /= 10;
        sum += 1;
    }
    sum
}

impl<'d> fmt::Display for DisplayList<'d> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = get_stylesheet();
        self.fmt_with_style(f, &style)
    }
}
