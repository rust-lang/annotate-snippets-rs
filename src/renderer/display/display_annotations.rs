use anstyle::Style;

use crate::{renderer::stylesheet::Stylesheet, snippet};

use super::{
    constants::{ERROR_TXT, HELP_TXT, INFO_TXT, NOTE_TXT, WARNING_TXT},
    display_text::DisplayTextFragment,
};

/// A type of the `Annotation` which may impact the sigils, style or text displayed.
///
/// There are several ways to uses this information when formatting the `DisplayList`:
///
/// * An annotation may display the name of the type like `error` or `info`.
/// * An underline for `Error` may be `^^^` while for `Warning` it could be `---`.
/// * `ColorStylesheet` may use different colors for different annotations.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DisplayAnnotationType {
    None,
    Error,
    Warning,
    Info,
    Note,
    Help,
}

/// An inline text
/// An indicator of what part of the annotation a given `Annotation` is.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DisplayAnnotationPart {
    /// A standalone, single-line annotation.
    Standalone,
    /// A continuation of a multi-line label of an annotation.
    LabelContinuation,
    /// A line starting a multiline annotation.
    MultilineStart(usize),
    /// A line ending a multiline annotation.
    MultilineEnd(usize),
}

/// Inline annotation which can be used in either Raw or Source line.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Annotation<'a> {
    pub(crate) annotation_type: DisplayAnnotationType,
    pub(crate) id: Option<&'a str>,
    pub(crate) label: Vec<DisplayTextFragment<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DisplaySourceAnnotation<'a> {
    pub(crate) annotation: Annotation<'a>,
    pub(crate) range: (usize, usize),
    pub(crate) annotation_type: DisplayAnnotationType,
    pub(crate) annotation_part: DisplayAnnotationPart,
}

impl DisplaySourceAnnotation<'_> {
    pub(crate) fn has_label(&self) -> bool {
        !self
            .annotation
            .label
            .iter()
            .all(|label| label.content.is_empty())
    }

    // Length of this annotation as displayed in the stderr output
    pub(crate) fn len(&self) -> usize {
        // Account for usize underflows
        if self.range.1 > self.range.0 {
            self.range.1 - self.range.0
        } else {
            self.range.0 - self.range.1
        }
    }

    pub(crate) fn takes_space(&self) -> bool {
        // Multiline annotations always have to keep vertical space.
        matches!(
            self.annotation_part,
            DisplayAnnotationPart::MultilineStart(_) | DisplayAnnotationPart::MultilineEnd(_)
        )
    }
}

impl From<snippet::Level> for DisplayAnnotationType {
    fn from(at: snippet::Level) -> Self {
        match at {
            snippet::Level::Error => DisplayAnnotationType::Error,
            snippet::Level::Warning => DisplayAnnotationType::Warning,
            snippet::Level::Info => DisplayAnnotationType::Info,
            snippet::Level::Note => DisplayAnnotationType::Note,
            snippet::Level::Help => DisplayAnnotationType::Help,
        }
    }
}

#[inline]
pub(crate) fn annotation_type_str(annotation_type: &DisplayAnnotationType) -> &'static str {
    match annotation_type {
        DisplayAnnotationType::Error => ERROR_TXT,
        DisplayAnnotationType::Help => HELP_TXT,
        DisplayAnnotationType::Info => INFO_TXT,
        DisplayAnnotationType::Note => NOTE_TXT,
        DisplayAnnotationType::Warning => WARNING_TXT,
        DisplayAnnotationType::None => "",
    }
}

pub(crate) fn annotation_type_len(annotation_type: &DisplayAnnotationType) -> usize {
    match annotation_type {
        DisplayAnnotationType::Error => ERROR_TXT.len(),
        DisplayAnnotationType::Help => HELP_TXT.len(),
        DisplayAnnotationType::Info => INFO_TXT.len(),
        DisplayAnnotationType::Note => NOTE_TXT.len(),
        DisplayAnnotationType::Warning => WARNING_TXT.len(),
        DisplayAnnotationType::None => 0,
    }
}

pub(crate) fn get_annotation_style<'a>(
    annotation_type: &DisplayAnnotationType,
    stylesheet: &'a Stylesheet,
) -> &'a Style {
    match annotation_type {
        DisplayAnnotationType::Error => stylesheet.error(),
        DisplayAnnotationType::Warning => stylesheet.warning(),
        DisplayAnnotationType::Info => stylesheet.info(),
        DisplayAnnotationType::Note => stylesheet.note(),
        DisplayAnnotationType::Help => stylesheet.help(),
        DisplayAnnotationType::None => stylesheet.none(),
    }
}

#[inline]
pub(crate) fn is_annotation_empty(annotation: &Annotation<'_>) -> bool {
    annotation
        .label
        .iter()
        .all(|fragment| fragment.content.is_empty())
}
