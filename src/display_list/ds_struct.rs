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
