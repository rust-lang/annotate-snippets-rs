use crate::display_list::{DisplayList, Margin};
use crate::snippet::Snippet;
use std::fmt::Display;

#[derive(Clone)]
pub struct Renderer {
    color: bool,
    anonymized_line_numbers: bool,
    margin: Option<Margin>,
}

impl Renderer {
    /// No terminal styling
    pub fn plain() -> Self {
        Self {
            color: false,
            anonymized_line_numbers: false,
            margin: None,
        }
    }

    /// Default terminal styling
    pub fn styled() -> Self {
        Self {
            color: true,
            anonymized_line_numbers: false,
            margin: None,
        }
    }

    pub fn anonymized_line_numbers(mut self, anonymized_line_numbers: bool) -> Self {
        self.anonymized_line_numbers = anonymized_line_numbers;
        self
    }

    pub fn color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    pub fn margin(mut self, margin: Option<Margin>) -> Self {
        self.margin = margin;
        self
    }

    pub fn render<'a>(&'a self, snippet: Snippet<'a>) -> impl Display + 'a {
        DisplayList::new(
            snippet,
            self.color,
            self.anonymized_line_numbers,
            self.margin,
        )
    }
}
