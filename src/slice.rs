use crate::display_list::DisplayList;
use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct Slice<'s> {
    pub source: &'s str,
    pub line_start: Option<usize>,
    pub origin: Option<&'s str>,
    pub annotations: Vec<SourceAnnotation<'s>>,
}

impl<'s> fmt::Display for Slice<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dl: DisplayList = self.into();
        write!(f, "{}", dl)
    }
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
