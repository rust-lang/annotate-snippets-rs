use crate::annotation::Annotation;
use crate::slice::Slice;

#[derive(Debug, Clone)]
pub struct Snippet<'s> {
    pub title: Option<Annotation<'s>>,
    pub footer: &'s [Annotation<'s>],
    pub slices: &'s [Slice<'s>],
}
