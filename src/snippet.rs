//! Structures used as an input for the library.

use crate::renderer::source_map::SourceMap;
use crate::renderer::stylesheet::Stylesheet;
use anstyle::Style;
use std::ops::Range;

pub(crate) const ERROR_TXT: &str = "error";
pub(crate) const HELP_TXT: &str = "help";
pub(crate) const INFO_TXT: &str = "info";
pub(crate) const NOTE_TXT: &str = "note";
pub(crate) const WARNING_TXT: &str = "warning";

#[derive(Debug)]
pub struct Message<'a> {
    pub(crate) id: Option<&'a str>, // for "correctness", could be sloppy and be on Title
    pub(crate) groups: Vec<Group<'a>>,
}

impl<'a> Message<'a> {
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn group(mut self, group: Group<'a>) -> Self {
        self.groups.push(group);
        self
    }

    pub(crate) fn max_line_number(&self) -> usize {
        self.groups
            .iter()
            .map(|v| {
                v.elements
                    .iter()
                    .map(|s| match s {
                        Element::Title(_) | Element::Origin(_) | Element::ColumnSeparator(_) => 0,
                        Element::Cause(cause) => {
                            let end = cause
                                .markers
                                .iter()
                                .map(|a| a.range.end)
                                .max()
                                .unwrap_or(cause.source.len())
                                .min(cause.source.len());

                            cause.line_start + newline_count(&cause.source[..end])
                        }
                        Element::Suggestion(suggestion) => {
                            let end = suggestion
                                .markers
                                .iter()
                                .map(|a| a.range.end)
                                .max()
                                .unwrap_or(suggestion.source.len())
                                .min(suggestion.source.len());

                            suggestion.line_start + newline_count(&suggestion.source[..end])
                        }
                    })
                    .max()
                    .unwrap_or(1)
            })
            .max()
            .unwrap_or(1)
    }
}

#[derive(Debug)]
pub struct Group<'a> {
    pub(crate) elements: Vec<Element<'a>>,
}

impl Default for Group<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Group<'a> {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn element(mut self, section: impl Into<Element<'a>>) -> Self {
        self.elements.push(section.into());
        self
    }

    pub fn elements(mut self, sections: impl IntoIterator<Item = impl Into<Element<'a>>>) -> Self {
        self.elements.extend(sections.into_iter().map(Into::into));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Element<'a> {
    Title(Title<'a>),
    Cause(Snippet<'a, Annotation<'a>>),
    Suggestion(Snippet<'a, Patch<'a>>),
    Origin(Origin<'a>),
    ColumnSeparator(ColumnSeparator),
}

impl<'a> From<Title<'a>> for Element<'a> {
    fn from(value: Title<'a>) -> Self {
        Element::Title(value)
    }
}

impl<'a> From<Snippet<'a, Annotation<'a>>> for Element<'a> {
    fn from(value: Snippet<'a, Annotation<'a>>) -> Self {
        Element::Cause(value)
    }
}

impl<'a> From<Snippet<'a, Patch<'a>>> for Element<'a> {
    fn from(value: Snippet<'a, Patch<'a>>) -> Self {
        Element::Suggestion(value)
    }
}

impl<'a> From<Origin<'a>> for Element<'a> {
    fn from(value: Origin<'a>) -> Self {
        Element::Origin(value)
    }
}

impl From<ColumnSeparator> for Element<'_> {
    fn from(value: ColumnSeparator) -> Self {
        Self::ColumnSeparator(value)
    }
}

#[derive(Debug)]
pub struct ColumnSeparator;

#[derive(Debug)]
pub struct Title<'a> {
    pub(crate) level: Level,
    pub(crate) title: &'a str,
    pub(crate) primary: bool,
}

impl Title<'_> {
    pub fn primary(mut self, primary: bool) -> Self {
        self.primary = primary;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warning,
    Info,
    Note,
    Help,
    None,
}

impl Level {
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    pub fn message(self, title: &str) -> Message<'_> {
        Message {
            id: None,
            groups: vec![Group::new().element(Element::Title(Title {
                level: self,
                title,
                primary: true,
            }))],
        }
    }

    /// Text passed to this function is allowed to be pre-styled, as such all
    /// text is considered "trusted input" and has no normalizations applied to
    /// it. [`normalize_untrusted_str`](crate::normalize_untrusted_str) can be
    /// used to normalize untrusted text before it is passed to this function.
    pub fn title(self, title: &str) -> Title<'_> {
        Title {
            level: self,
            title,
            primary: false,
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Level::Error => ERROR_TXT,
            Level::Warning => WARNING_TXT,
            Level::Info => INFO_TXT,
            Level::Note => NOTE_TXT,
            Level::Help => HELP_TXT,
            Level::None => "",
        }
    }

    pub(crate) fn style(&self, stylesheet: &Stylesheet) -> Style {
        match self {
            Level::Error => stylesheet.error,
            Level::Warning => stylesheet.warning,
            Level::Info => stylesheet.info,
            Level::Note => stylesheet.note,
            Level::Help => stylesheet.help,
            Level::None => stylesheet.none,
        }
    }
}

#[derive(Debug)]
pub struct Snippet<'a, T> {
    pub(crate) origin: Option<&'a str>,
    pub(crate) line_start: usize,
    pub(crate) source: &'a str,
    pub(crate) markers: Vec<T>,
    pub(crate) fold: bool,
}

impl<'a, T: Clone> Snippet<'a, T> {
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    pub fn source(source: &'a str) -> Self {
        Self {
            origin: None,
            line_start: 1,
            source,
            markers: vec![],
            fold: false,
        }
    }

    pub fn line_start(mut self, line_start: usize) -> Self {
        self.line_start = line_start;
        self
    }

    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn fold(mut self, fold: bool) -> Self {
        self.fold = fold;
        self
    }
}

impl<'a> Snippet<'a, Annotation<'a>> {
    pub fn annotation(mut self, annotation: Annotation<'a>) -> Snippet<'a, Annotation<'a>> {
        self.markers.push(annotation);
        self
    }

    pub fn annotations(mut self, annotation: impl IntoIterator<Item = Annotation<'a>>) -> Self {
        self.markers.extend(annotation);
        self
    }
}

impl<'a> Snippet<'a, Patch<'a>> {
    pub fn patch(mut self, patch: Patch<'a>) -> Snippet<'a, Patch<'a>> {
        self.markers.push(patch);
        self
    }

    pub fn patches(mut self, patches: impl IntoIterator<Item = Patch<'a>>) -> Self {
        self.markers.extend(patches);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Annotation<'a> {
    pub(crate) range: Range<usize>,
    pub(crate) label: Option<&'a str>,
    pub(crate) kind: AnnotationKind,
    pub(crate) highlight_source: bool,
}

impl<'a> Annotation<'a> {
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn highlight_source(mut self, highlight_source: bool) -> Self {
        self.highlight_source = highlight_source;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationKind {
    /// Color to [`Message`]'s [`Level`]
    Primary,
    /// "secondary"; fixed color
    Context,
}

impl AnnotationKind {
    pub fn span<'a>(self, span: Range<usize>) -> Annotation<'a> {
        Annotation {
            range: span,
            label: None,
            kind: self,
            highlight_source: false,
        }
    }

    pub(crate) fn is_primary(&self) -> bool {
        matches!(self, AnnotationKind::Primary)
    }
}

#[derive(Clone, Debug)]
pub struct Patch<'a> {
    pub(crate) range: Range<usize>,
    pub(crate) replacement: &'a str,
}

impl<'a> Patch<'a> {
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    pub fn new(range: Range<usize>, replacement: &'a str) -> Self {
        Self { range, replacement }
    }

    pub(crate) fn is_addition(&self, sm: &SourceMap<'_>) -> bool {
        !self.replacement.is_empty() && !self.replaces_meaningful_content(sm)
    }

    pub(crate) fn is_deletion(&self, sm: &SourceMap<'_>) -> bool {
        self.replacement.trim().is_empty() && self.replaces_meaningful_content(sm)
    }

    pub(crate) fn is_replacement(&self, sm: &SourceMap<'_>) -> bool {
        !self.replacement.is_empty() && self.replaces_meaningful_content(sm)
    }

    /// Whether this is a replacement that overwrites source with a snippet
    /// in a way that isn't a superset of the original string. For example,
    /// replacing "abc" with "abcde" is not destructive, but replacing it
    /// it with "abx" is, since the "c" character is lost.
    pub(crate) fn is_destructive_replacement(&self, sm: &SourceMap<'_>) -> bool {
        self.is_replacement(sm)
            && !sm
                .span_to_snippet(self.range.clone())
                // This should use `is_some_and` when our MSRV is >= 1.70
                .map_or(false, |s| {
                    as_substr(s.trim(), self.replacement.trim()).is_some()
                })
    }

    fn replaces_meaningful_content(&self, sm: &SourceMap<'_>) -> bool {
        sm.span_to_snippet(self.range.clone())
            .map_or(!self.range.is_empty(), |snippet| !snippet.trim().is_empty())
    }

    /// Try to turn a replacement into an addition when the span that is being
    /// overwritten matches either the prefix or suffix of the replacement.
    pub(crate) fn trim_trivial_replacements(&mut self, sm: &'a SourceMap<'a>) {
        if self.replacement.is_empty() {
            return;
        }
        let Some(snippet) = sm.span_to_snippet(self.range.clone()) else {
            return;
        };

        if let Some((prefix, substr, suffix)) = as_substr(snippet, self.replacement) {
            self.range = self.range.start + prefix..self.range.end.saturating_sub(suffix);
            self.replacement = substr;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Origin<'a> {
    pub(crate) origin: &'a str,
    pub(crate) line: Option<usize>,
    pub(crate) char_column: Option<usize>,
    pub(crate) primary: bool,
    pub(crate) label: Option<&'a str>,
}

impl<'a> Origin<'a> {
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    pub fn new(origin: &'a str) -> Self {
        Self {
            origin,
            line: None,
            char_column: None,
            primary: false,
            label: None,
        }
    }

    pub fn line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn char_column(mut self, char_column: usize) -> Self {
        self.char_column = Some(char_column);
        self
    }

    pub fn primary(mut self, primary: bool) -> Self {
        self.primary = primary;
        self
    }

    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}

fn newline_count(body: &str) -> usize {
    #[cfg(feature = "simd")]
    {
        memchr::memchr_iter(b'\n', body.as_bytes())
            .count()
            .saturating_sub(1)
    }
    #[cfg(not(feature = "simd"))]
    {
        body.lines().count().saturating_sub(1)
    }
}

/// Given an original string like `AACC`, and a suggestion like `AABBCC`, try to detect
/// the case where a substring of the suggestion is "sandwiched" in the original, like
/// `BB` is. Return the length of the prefix, the "trimmed" suggestion, and the length
/// of the suffix.
fn as_substr<'a>(original: &'a str, suggestion: &'a str) -> Option<(usize, &'a str, usize)> {
    let common_prefix = original
        .chars()
        .zip(suggestion.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .map(|(c, _)| c.len_utf8())
        .sum();
    let original = &original[common_prefix..];
    let suggestion = &suggestion[common_prefix..];
    if let Some(stripped) = suggestion.strip_suffix(original) {
        let common_suffix = original.len();
        Some((common_prefix, stripped, common_suffix))
    } else {
        None
    }
}
