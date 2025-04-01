use super::{
    display_annotations::{Annotation, DisplaySourceAnnotation},
    display_header::DisplayHeaderType,
    display_mark::DisplayMark,
    end_line::EndLine,
};

/// A single line used in `DisplayList`.
#[derive(Debug, PartialEq)]
pub(crate) enum DisplayLine<'a> {
    /// A line with `lineno` portion of the slice.
    Source {
        lineno: Option<usize>,
        inline_marks: Vec<DisplayMark>,
        line: DisplaySourceLine<'a>,
        annotations: Vec<DisplaySourceAnnotation<'a>>,
    },

    /// A line indicating a folded part of the slice.
    Fold { inline_marks: Vec<DisplayMark> },

    /// A line which is displayed outside of slices.
    Raw(DisplayRawLine<'a>),
}

/// A source line.
#[derive(Debug, PartialEq)]
pub(crate) enum DisplaySourceLine<'a> {
    /// A line with the content of the Snippet.
    Content {
        text: &'a str,
        range: (usize, usize), // meta information for annotation placement.
        end_line: EndLine,
    },
    /// An empty source line.
    Empty,
}

/// Raw line - a line which does not have the `lineno` part and is not considered
/// a part of the snippet.
#[derive(Debug, PartialEq)]
pub(crate) enum DisplayRawLine<'a> {
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
        /// without displaying the meta information (`type` and `id`) to be
        /// displayed on each line.
        continuation: bool,
    },
}
