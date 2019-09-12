#[derive(Debug, Clone)]
pub struct Annotation<'s> {
    pub id: Option<&'s str>,
    pub label: Option<&'s str>,
    pub annotation_type: AnnotationType,
}

#[derive(Debug, Clone)]
pub enum AnnotationType {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub struct SourceAnnotation<'s> {
    pub range: (usize, usize),
    pub label: &'s str,
    pub annotation_type: AnnotationType,
}
