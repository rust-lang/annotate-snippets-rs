use super::display_annotations::DisplayAnnotationType;

/// A visual mark used in `inline_marks` field of the `DisplaySourceLine`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DisplayMark {
    pub(crate) mark_type: DisplayMarkType,
    pub(crate) annotation_type: DisplayAnnotationType,
}

/// A type of the `DisplayMark`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DisplayMarkType {
    /// A mark indicating a multiline annotation going through the current line.
    AnnotationThrough(usize),
}
