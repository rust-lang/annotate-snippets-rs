/// List of lines to be displayed.
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayList {
    pub body: Vec<DisplayLine>,
}

impl From<Vec<DisplayLine>> for DisplayList {
    fn from(body: Vec<DisplayLine>) -> Self {
        Self { body }
    }
}

/// Inline annotation which can be used in either Raw or Source line.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub annotation_type: DisplayAnnotationType,
    pub id: Option<String>,
    pub label: Vec<DisplayTextFragment>,
}

/// A single line used in `DisplayList`.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayLine {
    /// A line with `lineno` portion of the slice.
    Source {
        lineno: Option<usize>,
        inline_marks: Vec<DisplayMark>,
        line: DisplaySourceLine,
    },

    /// A line indicating a folded part of the slice.
    Fold { inline_marks: Vec<DisplayMark> },

    /// A line which is displayed outside of slices.
    Raw(DisplayRawLine),
}

/// A source line.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplaySourceLine {
    /// A line with the content of the Slice.
    Content {
        text: String,
        range: (usize, usize), // meta information for annotation placement.
    },

    /// An annotation line which is displayed in context of the slice.
    Annotation {
        annotation: Annotation,
        range: (usize, usize),
        annotation_type: DisplayAnnotationType,
        annotation_part: DisplayAnnotationPart,
    },

    /// An empty source line.
    Empty,
}

/// Raw line - a line which does not have the `lineno` part and is not considered
/// a part of the snippet.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayRawLine {
    /// A line which provides information about the location of the given
    /// slice in the project structure.
    Origin {
        path: String,
        pos: Option<(usize, usize)>,
        header_type: DisplayHeaderType,
    },

    /// An annotation line which is not part of any snippet.
    Annotation {
        annotation: Annotation,

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
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayTextFragment {
    pub content: String,
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
    ///
    /// Example:
    /// ```
    /// use annotate_snippets::display_list::*;
    /// use annotate_snippets::formatter::DisplayListFormatter;
    ///
    /// let dlf = DisplayListFormatter::new(false); // Don't use colors
    ///
    /// let dl = DisplayList {
    ///     body: vec![
    ///         DisplayLine::Source {
    ///             lineno: Some(51),
    ///             inline_marks: vec![
    ///                 DisplayMark {
    ///                     mark_type: DisplayMarkType::AnnotationThrough,
    ///                     annotation_type: DisplayAnnotationType::Error,
    ///                 }
    ///             ],
    ///             line: DisplaySourceLine::Content {
    ///                 text: "Example".to_string(),
    ///                 range: (0, 7),
    ///             }
    ///         }
    ///     ]
    /// };
    /// assert_eq!(dlf.format(&dl), "51 | | Example");
    /// ```
    AnnotationThrough,

    /// A mark indicating a multiline annotation starting on the given line.
    ///
    /// Example:
    /// ```
    /// use annotate_snippets::display_list::*;
    /// use annotate_snippets::formatter::DisplayListFormatter;
    ///
    /// let dlf = DisplayListFormatter::new(false); // Don't use colors
    ///
    /// let dl = DisplayList {
    ///     body: vec![
    ///         DisplayLine::Source {
    ///             lineno: Some(51),
    ///             inline_marks: vec![
    ///                 DisplayMark {
    ///                     mark_type: DisplayMarkType::AnnotationStart,
    ///                     annotation_type: DisplayAnnotationType::Error,
    ///                 }
    ///             ],
    ///             line: DisplaySourceLine::Content {
    ///                 text: "Example".to_string(),
    ///                 range: (0, 7),
    ///             }
    ///         }
    ///     ]
    /// };
    /// assert_eq!(dlf.format(&dl), "51 | / Example");
    /// ```
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
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayHeaderType {
    /// Initial header is the first header in the snippet.
    ///
    /// Example:
    /// ```
    /// use annotate_snippets::display_list::*;
    /// use annotate_snippets::formatter::DisplayListFormatter;
    ///
    /// let dlf = DisplayListFormatter::new(false); // Don't use colors
    ///
    /// let dl = DisplayList {
    ///     body: vec![
    ///         DisplayLine::Raw(DisplayRawLine::Origin {
    ///             path: "file1.rs".to_string(),
    ///             pos: Some((51, 5)),
    ///             header_type: DisplayHeaderType::Initial,
    ///         })
    ///     ]
    /// };
    /// assert_eq!(dlf.format(&dl), "--> file1.rs:51:5");
    /// ```
    Initial,

    /// Continuation marks all headers of following slices in the snippet.
    ///
    /// Example:
    /// ```
    /// use annotate_snippets::display_list::*;
    /// use annotate_snippets::formatter::DisplayListFormatter;
    ///
    /// let dlf = DisplayListFormatter::new(false); // Don't use colors
    ///
    /// let dl = DisplayList {
    ///     body: vec![
    ///         DisplayLine::Raw(DisplayRawLine::Origin {
    ///             path: "file1.rs".to_string(),
    ///             pos: Some((51, 5)),
    ///             header_type: DisplayHeaderType::Continuation,
    ///         })
    ///     ]
    /// };
    /// assert_eq!(dlf.format(&dl), "::: file1.rs:51:5");
    /// ```
    Continuation,
}
