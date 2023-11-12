use crate::display_list::DisplayList;
use crate::snippet::Snippet;
use std::fmt::Display;

pub struct Renderer;

impl Renderer {
    pub fn render<'a>(&'a self, snippet: Snippet<'a>) -> impl Display + 'a {
        DisplayList::from(snippet)
    }
}
