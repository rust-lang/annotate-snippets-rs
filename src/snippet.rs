//! Structures used as an input for the library.
//!
//! Example:
//!
//! ```
//! use annotate_snippets::*;
//!
//! Snippet::error("mismatched types")
//!     .slice(Slice::new("Foo", 51).origin("src/format.rs"))
//!     .slice(Slice::new("Faa", 129).origin("src/display.rs"));
//! ```

use std::ops::Range;

/// Primary structure provided for formatting
pub struct Snippet<'a> {
    pub(crate) title: Label<'a>,
    pub(crate) id: Option<&'a str>,
    pub(crate) slices: Vec<Slice<'a>>,
    pub(crate) footer: Vec<Label<'a>>,
}

impl<'a> Snippet<'a> {
    pub fn title(title: Label<'a>) -> Self {
        Self {
            title,
            id: None,
            slices: vec![],
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

    pub fn slice(mut self, slice: Slice<'a>) -> Self {
        self.slices.push(slice);
        self
    }

    pub fn footer(mut self, footer: Label<'a>) -> Self {
        self.footer.push(footer);
        self
    }
}

pub struct Label<'a> {
    pub(crate) annotation_type: AnnotationType,
    pub(crate) label: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(annotation_type: AnnotationType, label: &'a str) -> Self {
        Self {
            annotation_type,
            label,
        }
    }
    pub fn error(label: &'a str) -> Self {
        Self::new(AnnotationType::Error, label)
    }

    pub fn warning(label: &'a str) -> Self {
        Self::new(AnnotationType::Warning, label)
    }

    pub fn info(label: &'a str) -> Self {
        Self::new(AnnotationType::Info, label)
    }

    pub fn note(label: &'a str) -> Self {
        Self::new(AnnotationType::Note, label)
    }

    pub fn help(label: &'a str) -> Self {
        Self::new(AnnotationType::Help, label)
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    /// Create a [SourceAnnotation] with the given span for a [Slice]
    pub fn span(&self, span: Range<usize>) -> SourceAnnotation<'a> {
        SourceAnnotation {
            range: span,
            label: self.label,
            annotation_type: self.annotation_type,
        }
    }
}

impl From<AnnotationType> for Label<'_> {
    fn from(annotation_type: AnnotationType) -> Self {
        Label {
            annotation_type,
            label: "",
        }
    }
}

/// Structure containing the slice of text to be annotated and
/// basic information about the location of the slice.
///
/// One `Slice` is meant to represent a single, continuous,
/// slice of source code that you want to annotate.
pub struct Slice<'a> {
    pub(crate) source: &'a str,
    pub(crate) line_start: usize,
    pub(crate) origin: Option<&'a str>,
    pub(crate) annotations: Vec<SourceAnnotation<'a>>,
    pub(crate) fold: bool,
}

impl<'a> Slice<'a> {
    pub fn new(source: &'a str, line_start: usize) -> Self {
        Self {
            source,
            line_start,
            origin: None,
            annotations: vec![],
            fold: false,
        }
    }

    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn annotation(mut self, annotation: SourceAnnotation<'a>) -> Self {
        self.annotations.push(annotation);
        self
    }

    pub fn fold(mut self, fold: bool) -> Self {
        self.fold = fold;
        self
    }
}

/// Types of annotations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnnotationType {
    /// Error annotations are displayed using red color and "^" character.
    Error,
    /// Warning annotations are displayed using blue color and "-" character.
    Warning,
    Info,
    Note,
    Help,
}

/// An annotation for a `Slice`.
///
/// This gets created by [Label::span].
#[derive(Debug)]
pub struct SourceAnnotation<'a> {
    /// The byte range of the annotation in the `source` string
    pub(crate) range: Range<usize>,
    pub(crate) label: &'a str,
    pub(crate) annotation_type: AnnotationType,
}
