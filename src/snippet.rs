//! Structures used as an input for the library.

use crate::renderer::source_map::SourceMap;
use crate::Level;
use std::borrow::Cow;
use std::ops::Range;

pub(crate) const ERROR_TXT: &str = "error";
pub(crate) const HELP_TXT: &str = "help";
pub(crate) const INFO_TXT: &str = "info";
pub(crate) const NOTE_TXT: &str = "note";
pub(crate) const WARNING_TXT: &str = "warning";

#[derive(Clone, Debug, Default)]
pub(crate) struct Id<'a> {
    pub(crate) id: Option<Cow<'a, str>>,
    pub(crate) url: Option<Cow<'a, str>>,
}

/// An [`Element`] container
///
/// A [diagnostic][crate::Renderer::render] is made of several `Group`s.
/// `Group`s are used to [annotate][AnnotationKind::Primary] [`Snippet`]s
/// with different [semantic reasons][Title].
///
/// # Example
///
/// ```rust
/// # #[allow(clippy::needless_doctest_main)]
#[doc = include_str!("../examples/highlight_message.rs")]
/// ```
#[doc = include_str!("../examples/highlight_message.svg")]
#[derive(Clone, Debug)]
pub struct Group<'a> {
    pub(crate) primary_level: Level<'a>,
    pub(crate) elements: Vec<Element<'a>>,
}

impl<'a> Group<'a> {
    /// Create group with a title, deriving the primary [`Level`] for [`Annotation`]s from it
    pub fn with_title(title: Title<'a>) -> Self {
        let level = title.level.clone();
        Self::with_level(level).element(title)
    }

    /// Create a title-less group with a primary [`Level`] for [`Annotation`]s
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[allow(clippy::needless_doctest_main)]
    #[doc = include_str!("../examples/elide_header.rs")]
    /// ```
    #[doc = include_str!("../examples/elide_header.svg")]
    pub fn with_level(level: Level<'a>) -> Self {
        Self {
            primary_level: level,
            elements: vec![],
        }
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

/// A section of content within a [`Group`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Element<'a> {
    Title(Title<'a>),
    Message(Message<'a>),
    Cause(Snippet<'a, Annotation<'a>>),
    Suggestion(Snippet<'a, Patch<'a>>),
    Origin(Origin<'a>),
    Padding(Padding),
}

impl<'a> From<Title<'a>> for Element<'a> {
    fn from(value: Title<'a>) -> Self {
        Element::Title(value)
    }
}

impl<'a> From<Message<'a>> for Element<'a> {
    fn from(value: Message<'a>) -> Self {
        Element::Message(value)
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

impl From<Padding> for Element<'_> {
    fn from(value: Padding) -> Self {
        Self::Padding(value)
    }
}

/// A whitespace [`Element`] in a [`Group`]
#[derive(Clone, Debug)]
pub struct Padding;

/// A text [`Element`] to start a [`Group`]
///
/// See [`Level::title`] to create this.
#[derive(Clone, Debug)]
pub struct Title<'a> {
    pub(crate) level: Level<'a>,
    pub(crate) id: Option<Id<'a>>,
    pub(crate) text: Cow<'a, str>,
}

impl<'a> Title<'a> {
    /// <div class="warning">
    ///
    /// This is only relevant if the title is the first element of a group.
    ///
    /// </div>
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.id.get_or_insert(Id::default()).id = Some(id.into());
        self
    }

    /// <div class="warning">
    ///
    /// This is only relevant if the title is the first element of a group and
    /// `id` present
    ///
    /// </div>
    pub fn id_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
        self.id.get_or_insert(Id::default()).url = Some(url.into());
        self
    }
}

/// A text [`Element`] in a [`Group`]
///
/// See [`Level::message`] to create this.
#[derive(Clone, Debug)]
pub struct Message<'a> {
    pub(crate) level: Level<'a>,
    pub(crate) text: Cow<'a, str>,
}

/// A source view [`Element`] in a [`Group`]
///
/// If you do not have [source][Snippet::source] available, see instead [`Origin`]
#[derive(Clone, Debug)]
pub struct Snippet<'a, T> {
    pub(crate) path: Option<Cow<'a, str>>,
    pub(crate) line_start: usize,
    pub(crate) source: Cow<'a, str>,
    pub(crate) markers: Vec<T>,
    pub(crate) fold: bool,
}

impl<'a, T: Clone> Snippet<'a, T> {
    /// The source code to be rendered
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn source(source: impl Into<Cow<'a, str>>) -> Self {
        Self {
            path: None,
            line_start: 1,
            source: source.into(),
            markers: vec![],
            fold: true,
        }
    }

    /// When manually [`fold`][Self::fold]ing,
    /// the [`source`][Self::source]s line offset from the original start
    pub fn line_start(mut self, line_start: usize) -> Self {
        self.line_start = line_start;
        self
    }

    /// The location of the [`source`][Self::source] (e.g. a path)
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn path(mut self, path: impl Into<OptionCow<'a>>) -> Self {
        self.path = path.into().0;
        self
    }

    /// Hide lines without [`Annotation`]s
    pub fn fold(mut self, fold: bool) -> Self {
        self.fold = fold;
        self
    }
}

impl<'a> Snippet<'a, Annotation<'a>> {
    /// Highlight and describe a span of text within the [`source`][Self::source]
    pub fn annotation(mut self, annotation: Annotation<'a>) -> Snippet<'a, Annotation<'a>> {
        self.markers.push(annotation);
        self
    }

    /// Highlight and describe spans of text within the [`source`][Self::source]
    pub fn annotations(mut self, annotation: impl IntoIterator<Item = Annotation<'a>>) -> Self {
        self.markers.extend(annotation);
        self
    }
}

impl<'a> Snippet<'a, Patch<'a>> {
    /// Suggest to the user an edit to the [`source`][Self::source]
    pub fn patch(mut self, patch: Patch<'a>) -> Snippet<'a, Patch<'a>> {
        self.markers.push(patch);
        self
    }

    /// Suggest to the user edits to the [`source`][Self::source]
    pub fn patches(mut self, patches: impl IntoIterator<Item = Patch<'a>>) -> Self {
        self.markers.extend(patches);
        self
    }
}

/// Highlighted and describe a span of text within a [`Snippet`]
///
/// See [`AnnotationKind`] to create an annotation.
#[derive(Clone, Debug)]
pub struct Annotation<'a> {
    pub(crate) span: Range<usize>,
    pub(crate) label: Option<Cow<'a, str>>,
    pub(crate) kind: AnnotationKind,
    pub(crate) highlight_source: bool,
}

impl<'a> Annotation<'a> {
    /// Describe the reason the span is highlighted
    ///
    /// This will be styled according to the [`AnnotationKind`]
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn label(mut self, label: impl Into<OptionCow<'a>>) -> Self {
        self.label = label.into().0;
        self
    }

    /// Style the source according to the [`AnnotationKind`]
    pub fn highlight_source(mut self, highlight_source: bool) -> Self {
        self.highlight_source = highlight_source;
        self
    }
}

/// The category of the [`Annotation`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum AnnotationKind {
    /// Shows the source that the [Group's Title][Group::with_title] references
    ///
    /// For [`Title`]-less groups, see [`Group::with_level`]
    Primary,
    /// Additional context to explain the [`Primary`][Self::Primary]
    /// [`Annotation`]
    ///
    /// See also [`Renderer::context`].
    ///
    /// [`Renderer::context`]: crate::renderer::Renderer
    Context,
}

impl AnnotationKind {
    pub fn span<'a>(self, span: Range<usize>) -> Annotation<'a> {
        Annotation {
            span,
            label: None,
            kind: self,
            highlight_source: false,
        }
    }

    pub(crate) fn is_primary(&self) -> bool {
        matches!(self, AnnotationKind::Primary)
    }
}

/// Suggested edit to the [`Snippet`]
#[derive(Clone, Debug)]
pub struct Patch<'a> {
    pub(crate) span: Range<usize>,
    pub(crate) replacement: Cow<'a, str>,
}

impl<'a> Patch<'a> {
    /// Splice `replacement` into the [`Snippet`] at the `span`
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn new(span: Range<usize>, replacement: impl Into<Cow<'a, str>>) -> Self {
        Self {
            span,
            replacement: replacement.into(),
        }
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
                .span_to_snippet(self.span.clone())
                // This should use `is_some_and` when our MSRV is >= 1.70
                .map_or(false, |s| {
                    as_substr(s.trim(), self.replacement.trim()).is_some()
                })
    }

    fn replaces_meaningful_content(&self, sm: &SourceMap<'_>) -> bool {
        sm.span_to_snippet(self.span.clone())
            .map_or(!self.span.is_empty(), |snippet| !snippet.trim().is_empty())
    }

    /// Try to turn a replacement into an addition when the span that is being
    /// overwritten matches either the prefix or suffix of the replacement.
    pub(crate) fn trim_trivial_replacements(&mut self, sm: &'a SourceMap<'a>) {
        if self.replacement.is_empty() {
            return;
        }
        let Some(snippet) = sm.span_to_snippet(self.span.clone()) else {
            return;
        };

        if let Some((prefix, substr, suffix)) = as_substr(snippet, &self.replacement) {
            self.span = self.span.start + prefix..self.span.end.saturating_sub(suffix);
            self.replacement = Cow::Owned(substr.to_owned());
        }
    }
}

/// A source location [`Element`] in a [`Group`]
///
/// If you have source available, see instead [`Snippet`]
///
/// # Example
///
/// ```rust
/// # use annotate_snippets::{Group, Snippet, AnnotationKind, Level, Origin};
/// let input = &[
///     Group::with_title(Level::ERROR.title("mismatched types").id("E0308"))
///         .element(
///             Origin::path("$DIR/mismatched-types.rs")
///         )
/// ];
/// ```
#[derive(Clone, Debug)]
pub struct Origin<'a> {
    pub(crate) path: Cow<'a, str>,
    pub(crate) line: Option<usize>,
    pub(crate) char_column: Option<usize>,
    pub(crate) primary: bool,
}

impl<'a> Origin<'a> {
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Pre-styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn path(path: impl Into<Cow<'a, str>>) -> Self {
        Self {
            path: path.into(),
            line: None,
            char_column: None,
            primary: false,
        }
    }

    /// Set the default line number to display
    pub fn line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Set the default column to display
    ///
    /// <div class="warning">
    ///
    /// `char_column` is only be respected if [`Origin::line`] is also set.
    ///
    /// </div>
    pub fn char_column(mut self, char_column: usize) -> Self {
        self.char_column = Some(char_column);
        self
    }

    /// Mark this as the source that the [Group's Title][Group::with_title] references
    pub fn primary(mut self, primary: bool) -> Self {
        self.primary = primary;
        self
    }
}

impl<'a> From<Cow<'a, str>> for Origin<'a> {
    fn from(origin: Cow<'a, str>) -> Self {
        Self::path(origin)
    }
}

#[derive(Debug)]
pub struct OptionCow<'a>(pub(crate) Option<Cow<'a, str>>);

impl<'a, T: Into<Cow<'a, str>>> From<Option<T>> for OptionCow<'a> {
    fn from(value: Option<T>) -> Self {
        Self(value.map(Into::into))
    }
}

impl<'a> From<&'a Cow<'a, str>> for OptionCow<'a> {
    fn from(value: &'a Cow<'a, str>) -> Self {
        Self(Some(Cow::Borrowed(value)))
    }
}

impl<'a> From<Cow<'a, str>> for OptionCow<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self(Some(value))
    }
}

impl<'a> From<&'a str> for OptionCow<'a> {
    fn from(value: &'a str) -> Self {
        Self(Some(Cow::Borrowed(value)))
    }
}
impl<'a> From<String> for OptionCow<'a> {
    fn from(value: String) -> Self {
        Self(Some(Cow::Owned(value)))
    }
}

impl<'a> From<&'a String> for OptionCow<'a> {
    fn from(value: &'a String) -> Self {
        Self(Some(Cow::Borrowed(value.as_str())))
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
