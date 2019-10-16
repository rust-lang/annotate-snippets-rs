use crate::annotation::SourceAnnotation;

#[derive(Debug, Clone, Default)]
pub struct Slice<'s> {
    pub source: &'s str,
    pub line_start: Option<usize>,
    pub origin: Option<&'s str>,
    pub annotations: &'s [SourceAnnotation<'s>],
}
