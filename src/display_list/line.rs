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

#[derive(Debug, Clone)]
pub enum DisplayMarkType {
    AnnotationThrough,
    AnnotationStart,
}
