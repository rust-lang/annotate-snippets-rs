//! Structures used as an input for the library.
//!
//! Example:
//!
//! ```
//! use annotate_snippets::*;
//!
//! Message::error("mismatched types")
//!     .snippet(Snippet::new("Foo", 51).origin("src/format.rs"))
//!     .snippet(Snippet::new("Faa", 129).origin("src/display.rs"));
//! ```

use std::ops::Range;

/// Primary structure provided for formatting
pub struct Message<'a> {
    pub(crate) title: Label<'a>,
    pub(crate) id: Option<&'a str>,
    pub(crate) snippets: Vec<Snippet<'a>>,
    pub(crate) footer: Vec<Label<'a>>,
}

impl<'a> Message<'a> {
    pub fn title(title: Label<'a>) -> Self {
        Self {
            title,
            id: None,
            snippets: vec![],
            footer: vec![],
        }
    }

    pub fn error(title: &'a str) -> Self {
        Self::title(Label::error(title))
    }

    pub fn warning(title: &'a str) -> Self {
        Self::title(Label::warning(title))
    }

    pub fn info(title: &'a str) -> Self {
        Self::title(Label::info(title))
    }

    pub fn note(title: &'a str) -> Self {
        Self::title(Label::note(title))
    }

    pub fn help(title: &'a str) -> Self {
        Self::title(Label::help(title))
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn snippet(mut self, slice: Snippet<'a>) -> Self {
        self.snippets.push(slice);
        self
    }

    pub fn footer(mut self, footer: Label<'a>) -> Self {
        self.footer.push(footer);
        self
    }
}

pub struct Label<'a> {
    pub(crate) level: Level,
    pub(crate) label: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(level: Level, label: &'a str) -> Self {
        Self { level, label }
    }
    pub fn error(label: &'a str) -> Self {
        Self::new(Level::Error, label)
    }

    pub fn warning(label: &'a str) -> Self {
        Self::new(Level::Warning, label)
    }

    pub fn info(label: &'a str) -> Self {
        Self::new(Level::Info, label)
    }

    pub fn note(label: &'a str) -> Self {
        Self::new(Level::Note, label)
    }

    pub fn help(label: &'a str) -> Self {
        Self::new(Level::Help, label)
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    /// Create a [`Annotation`] with the given span for a [`Snippet`]
    pub fn span(&self, span: Range<usize>) -> Annotation<'a> {
        Annotation {
            range: span,
            label: self.label,
            level: self.level,
        }
    }
}

/// Structure containing the slice of text to be annotated and
/// basic information about the location of the slice.
///
/// One `Snippet` is meant to represent a single, continuous,
/// slice of source code that you want to annotate.
pub struct Snippet<'a> {
    pub(crate) origin: Option<&'a str>,
    pub(crate) line_start: usize,

    pub(crate) source: &'a str,
    pub(crate) annotations: Vec<Annotation<'a>>,

    pub(crate) fold: bool,
}

impl<'a> Snippet<'a> {
    pub fn new(source: &'a str, line_start: usize) -> Self {
        Self {
            origin: None,
            line_start,
            source,
            annotations: vec![],
            fold: false,
        }
    }

    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn annotation(mut self, annotation: Annotation<'a>) -> Self {
        self.annotations.push(annotation);
        self
    }

    pub fn fold(mut self, fold: bool) -> Self {
        self.fold = fold;
        self
    }
}

/// An annotation for a [`Snippet`].
///
/// This gets created by [`Label::span`].
#[derive(Debug)]
pub struct Annotation<'a> {
    /// The byte range of the annotation in the `source` string
    pub(crate) range: Range<usize>,
    pub(crate) label: &'a str,
    pub(crate) level: Level,
}

/// Types of annotations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    /// Error annotations are displayed using red color and "^" character.
    Error,
    /// Warning annotations are displayed using blue color and "-" character.
    Warning,
    Info,
    Note,
    Help,
}
