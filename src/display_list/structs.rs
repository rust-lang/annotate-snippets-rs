#[derive(Debug, Clone, PartialEq)]
pub struct DisplayList {
    pub body: Vec<DisplayLine>,
}

impl From<Vec<DisplayLine>> for DisplayList {
    fn from(body: Vec<DisplayLine>) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub annotation_type: DisplayAnnotationType,
    pub id: Option<String>,
    pub label: Vec<DisplayTextFragment>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayLine {
    Source {
        lineno: Option<usize>,
        inline_marks: Vec<DisplayMark>,
        line: DisplaySourceLine,
    },
    Fold {
        inline_marks: Vec<DisplayMark>,
    },
    Raw(DisplayRawLine),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplaySourceLine {
    Content {
        text: String,
        range: (usize, usize),
    },
    Annotation {
        annotation: Annotation,
        range: (usize, usize),
        annotation_type: DisplayAnnotationType,
        annotation_part: DisplayAnnotationPart,
    },
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayRawLine {
    Origin {
        path: String,
        pos: Option<(usize, usize)>,
        header_type: DisplayHeaderType,
    },
    Annotation {
        annotation: Annotation,
        source_aligned: bool,
        continuation: bool,
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
    Standalone,
    LabelContinuation,
    Consequitive,
    MultilineStart,
    MultilineEnd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayMark {
    pub mark_type: DisplayMarkType,
    pub annotation_type: DisplayAnnotationType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayMarkType {
    AnnotationThrough,
    AnnotationStart,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayAnnotationType {
    None,
    Error,
    Warning,
    Info,
    Note,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayHeaderType {
    Initial,
    Continuation,
}
