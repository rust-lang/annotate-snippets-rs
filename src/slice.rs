use crate::annotation::{InlineAnnotation, SourceAnnotation};

#[derive(Debug, Clone, Default)]
pub struct Slice<'s> {
    pub source: &'s str,
    pub line_start: Option<usize>,
    pub origin: Option<&'s str>,
    pub annotations: &'s [SourceAnnotation<'s>],
    pub inline_annotations: &'s [InlineAnnotation],
}
