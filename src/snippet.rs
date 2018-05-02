/// Primary structure provided for formatting
#[derive(Debug, Clone)]
pub struct Snippet {
    pub title: Option<Annotation>,
    pub footer: Vec<Annotation>,
    pub slices: Vec<Slice>,
}

/// Structure containing the slice of text to be annotated and
/// basic information about the location of the slice.
#[derive(Debug, Clone)]
pub struct Slice {
    pub source: String,
    pub line_start: usize,
    pub origin: Option<String>,
    pub annotations: Vec<SourceAnnotation>,
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
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub struct SourceAnnotation {
    pub range: (usize, usize),
    pub label: String,
    pub annotation_type: AnnotationType,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    /// Identifier of the annotation. Usually error code like "E0308".
    pub id: Option<String>,
    pub label: Option<String>,
    pub annotation_type: AnnotationType,
}
