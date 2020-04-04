use std::fmt;

use crate::formatter::{get_term_style, style::Stylesheet};

/// List of lines to be displayed.
pub struct DisplayList<'a> {
    pub body: Vec<DisplayLine<'a>>,
    pub stylesheet: Box<dyn Stylesheet>,
    pub anonymized_line_numbers: bool,
}

impl<'a> From<Vec<DisplayLine<'a>>> for DisplayList<'a> {
    fn from(body: Vec<DisplayLine<'a>>) -> DisplayList<'a> {
        Self {
            body,
            anonymized_line_numbers: false,
            stylesheet: get_term_style(false),
        }
    }
}

impl<'a> PartialEq for DisplayList<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body && self.anonymized_line_numbers == other.anonymized_line_numbers
    }
}

impl<'a> fmt::Debug for DisplayList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DisplayList")
            .field("body", &self.body)
            .field("anonymized_line_numbers", &self.anonymized_line_numbers)
            .finish()
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct FormatOptions {
    pub color: bool,
    pub anonymized_line_numbers: bool,
}

/// Inline annotation which can be used in either Raw or Source line.
#[derive(Debug, PartialEq)]
pub struct Annotation<'a> {
    pub annotation_type: DisplayAnnotationType,
    pub id: Option<&'a str>,
    pub label: Vec<DisplayTextFragment<'a>>,
}

/// A single line used in `DisplayList`.
#[derive(Debug, PartialEq)]
pub enum DisplayLine<'a> {
    /// A line with `lineno` portion of the slice.
    Source {
        lineno: Option<usize>,
        inline_marks: Vec<DisplayMark>,
        line: DisplaySourceLine<'a>,
    },

    /// A line indicating a folded part of the slice.
    Fold { inline_marks: Vec<DisplayMark> },

    /// A line which is displayed outside of slices.
    Raw(DisplayRawLine<'a>),
}

/// A source line.
#[derive(Debug, PartialEq)]
pub enum DisplaySourceLine<'a> {
    /// A line with the content of the Slice.
    Content {
        text: &'a str,
        range: (usize, usize), // meta information for annotation placement.
    },

    /// An annotation line which is displayed in context of the slice.
    Annotation {
        annotation: Annotation<'a>,
        range: (usize, usize),
        annotation_type: DisplayAnnotationType,
        annotation_part: DisplayAnnotationPart,
    },

    /// An empty source line.
    Empty,
}

/// Raw line - a line which does not have the `lineno` part and is not considered
/// a part of the snippet.
#[derive(Debug, PartialEq)]
pub enum DisplayRawLine<'a> {
    /// A line which provides information about the location of the given
    /// slice in the project structure.
    Origin {
        path: &'a str,
        pos: Option<(usize, usize)>,
        header_type: DisplayHeaderType,
    },

    /// An annotation line which is not part of any snippet.
    Annotation {
        annotation: Annotation<'a>,

        /// If set to `true`, the annotation will be aligned to the
        /// lineno delimiter of the snippet.
        source_aligned: bool,
        /// If set to `true`, only the label of the `Annotation` will be
        /// displayed. It allows for a multiline annotation to be aligned
        /// without displaing the meta information (`type` and `id`) to be
        /// displayed on each line.
        continuation: bool,
    },
}

/// An inline text fragment which any label is composed of.
#[derive(Debug, PartialEq)]
pub struct DisplayTextFragment<'a> {
    pub content: &'a str,
    pub style: DisplayTextStyle,
}

/// A style for the `DisplayTextFragment` which can be visually formatted.
///
/// This information may be used to emphasis parts of the label.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayTextStyle {
    Regular,
    Emphasis,
}

/// An indicator of what part of the annotation a given `Annotation` is.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayAnnotationPart {
    /// A standalone, single-line annotation.
    Standalone,
    /// A continuation of a multi-line label of an annotation.
    LabelContinuation,
    /// A consequitive annotation in case multiple annotations annotate a single line.
    Consequitive,
    /// A line starting a multiline annotation.
    MultilineStart,
    /// A line ending a multiline annotation.
    MultilineEnd,
}

/// A visual mark used in `inline_marks` field of the `DisplaySourceLine`.
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayMark {
    pub mark_type: DisplayMarkType,
    pub annotation_type: DisplayAnnotationType,
}

/// A type of the `DisplayMark`.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayMarkType {
    /// A mark indicating a multiline annotation going through the current line.
    AnnotationThrough,
    /// A mark indicating a multiline annotation starting on the given line.
    AnnotationStart,
}

/// A type of the `Annotation` which may impact the sigils, style or text displayed.
///
/// There are several ways in which the `DisplayListFormatter` uses this information
/// when formatting the `DisplayList`:
///
/// * An annotation may display the name of the type like `error` or `info`.
/// * An underline for `Error` may be `^^^` while for `Warning` it coule be `---`.
/// * `ColorStylesheet` may use different colors for different annotations.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayAnnotationType {
    None,
    Error,
    Warning,
    Info,
    Note,
    Help,
}

/// Information whether the header is the initial one or a consequitive one
/// for multi-slice cases.
// TODO: private
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayHeaderType {
    /// Initial header is the first header in the snippet.
    Initial,

    /// Continuation marks all headers of following slices in the snippet.
    Continuation,
}
