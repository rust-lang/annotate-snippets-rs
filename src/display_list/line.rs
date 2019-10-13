use std::fmt;
use std::fmt::Write;

use super::annotation::Annotation;
use crate::annotation::AnnotationType;

#[derive(Debug, Clone)]
pub enum DisplayLine<'d> {
    Source {
        lineno: Option<usize>,
        inline_marks: Vec<DisplayMark>,
        line: DisplaySourceLine<'d>,
    },
    Raw(DisplayRawLine<'d>),
}

#[derive(Debug, Clone)]
pub enum DisplaySourceLine<'d> {
    Content {
        text: &'d str,
    },
    Annotation {
        annotation: Annotation<'d>,
        range: (usize, usize),
    },
    Empty,
}

#[derive(Debug, Clone)]
pub enum DisplayRawLine<'d> {
    Origin {
        path: &'d str,
        pos: (Option<usize>, Option<usize>),
    },
    Annotation {
        annotation: Annotation<'d>,
        source_aligned: bool,
        continuation: bool,
    },
}

#[derive(Debug, Clone)]
pub struct DisplayMark {
    pub mark_type: DisplayMarkType,
    pub annotation_type: AnnotationType,
}

impl fmt::Display for DisplayMark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mark_type {
            DisplayMarkType::AnnotationStart => f.write_char('/'),
            DisplayMarkType::AnnotationThrough => f.write_char('|'),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DisplayMarkType {
    AnnotationThrough,
    AnnotationStart,
}
