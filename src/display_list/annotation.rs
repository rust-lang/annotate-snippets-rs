use crate::annotation::AnnotationType;

#[derive(Debug, Clone)]
pub struct Annotation<'d> {
    pub annotation_type: AnnotationType,
    pub id: Option<&'d str>,
    pub label: &'d str,
}
