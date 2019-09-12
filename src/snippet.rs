use crate::annotation::Annotation;
use crate::display_list::DisplayList;
use crate::slice::Slice;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Snippet<'s> {
    pub title: Option<Annotation<'s>>,
    pub footer: &'s [Annotation<'s>],
    pub slices: &'s [Slice<'s>],
}

impl<'s> fmt::Display for Snippet<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dl: DisplayList = self.into();
        write!(f, "{}", dl)
    }
}
