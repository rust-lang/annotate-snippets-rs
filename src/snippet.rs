/// Primary structure provided for formatting
#[derive(Debug, Clone)]
pub struct Snippet {
    pub title: Option<TitleAnnotation>,
    pub slices: Vec<Slice>,
}

/// Structure containing the slice of text to be annotated and
/// basic information about the location of the slice.
#[derive(Debug, Clone)]
pub struct Slice {
    pub source: String,
    pub line_start: usize,
    pub origin: Option<String>,
    pub annotations: Vec<Annotation>,
    /// If set explicitly to `true`, the snippet will fold
    /// parts of the slice that don't contain any annotations.
    pub fold: bool,
}

/// Types of annotations.
#[derive(Debug, Clone, Copy)]
pub enum AnnotationType {
    /// Error annotations are displayed using red color and "^" character.
    Error,
    /// Warning annotations are displayed using blue color and "-" character.
    Warning,
}

/// An Annotation is a pointer to a place in the Slice which is to be annotated.
#[derive(Debug, Clone)]
pub struct Annotation {
    pub range: (usize, usize),
    pub label: String,
    pub annotation_type: AnnotationType,
}

/// An annotation used to describe the whole snippet.
#[derive(Debug, Clone)]
pub struct TitleAnnotation {
    /// Identifier of the annotation. Usually error code like "E0308".
    pub id: Option<String>,
    pub label: Option<String>,
    pub annotation_type: AnnotationType,
}
