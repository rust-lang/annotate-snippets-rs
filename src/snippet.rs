#[derive(Debug)]
pub struct Snippet {
    pub slice: Slice,
    pub annotations: Vec<Annotation>,
    pub main_annotation_pos: Option<usize>,
    pub title_annotation_pos: Option<usize>,
    pub fold: Option<bool>,
}

#[derive(Debug)]
pub struct Slice {
    pub source: String,
    pub line_start: usize,
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum AnnotationType {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct Annotation {
    pub range: (Option<usize>, Option<usize>),
    pub label: Option<String>,
    pub id: Option<String>,
    pub annotation_type: AnnotationType,
}
