//! Structures used as an input for the library.

use alloc::borrow::{Cow, ToOwned};
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::borrow::Borrow;
use core::fmt::{self, Debug, Display, Formatter};
use core::ops::Range;

use crate::Level;
use crate::renderer::source_map::{TrimmedPatch, as_substr};

pub(crate) const ERROR_TXT: &str = "error";
pub(crate) const HELP_TXT: &str = "help";
pub(crate) const INFO_TXT: &str = "info";
pub(crate) const NOTE_TXT: &str = "note";
pub(crate) const WARNING_TXT: &str = "warning";

/// A [diagnostic message][Title] and any associated [context][Element] to help users
/// understand it
///
/// The first [`Group`] is the ["primary" group][Level::primary_title], ie it contains the diagnostic
/// message.
///
/// All subsequent [`Group`]s are for distinct pieces of [context][Level::secondary_title].
/// The primary group will be visually distinguished to help tell them apart.
pub type Report<'a> = &'a [Group<'a>];

#[derive(Clone, Debug, Default)]
pub(crate) struct Id<'a> {
    pub(crate) id: Option<Cow<'a, str>>,
    pub(crate) url: Option<Cow<'a, str>>,
}

/// A [`Title`] with supporting [context][Element] within a [`Report`]
///
/// [Decor][crate::renderer::DecorStyle] is used to visually connect [`Element`]s of a `Group`.
///
/// Generally, you will create separate group's for:
/// - New [`Snippet`]s, especially if they need their own [`AnnotationKind::Primary`]
/// - Each logically distinct set of [suggestions][Patch`]
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
    pub(crate) title: Option<Title<'a>>,
    pub(crate) elements: Vec<Element<'a>>,
}

impl<'a> Group<'a> {
    /// Create group with a [`Title`], deriving [`AnnotationKind::Primary`] from its [`Level`]
    pub fn with_title(title: Title<'a>) -> Self {
        let level = title.level.clone();
        let mut x = Self::with_level(level);
        x.title = Some(title);
        x
    }

    /// Create a title-less group with a primary [`Level`] for [`AnnotationKind::Primary`]
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
            title: None,
            elements: vec![],
        }
    }

    /// Append an [`Element`] that adds context to the [`Title`]
    pub fn element(mut self, section: impl Into<Element<'a>>) -> Self {
        self.elements.push(section.into());
        self
    }

    /// Append [`Element`]s that adds context to the [`Title`]
    pub fn elements(mut self, sections: impl IntoIterator<Item = impl Into<Element<'a>>>) -> Self {
        self.elements.extend(sections.into_iter().map(Into::into));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty() && self.title.is_none()
    }
}

/// A section of content within a [`Group`]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Element<'a> {
    Message(Message<'a>),
    Cause(Snippet<'a, Annotation<'a>>),
    Suggestion(Snippet<'a, Patch<'a>>),
    Origin(Origin<'a>),
    Padding(Padding),
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

/// A title that introduces a [`Group`], describing the main point
///
/// To create a `Title`, see [`Level::primary_title`] or [`Level::secondary_title`].
///
/// # Example
///
/// ```rust
/// # use annotate_snippets::*;
/// let report = &[
///     Group::with_title(
///         Level::ERROR.primary_title("mismatched types").id("E0308")
///     ),
///     Group::with_title(
///         Level::HELP.secondary_title("function defined here")
///     ),
/// ];
/// ```
#[derive(Clone, Debug)]
pub struct Title<'a> {
    pub(crate) level: Level<'a>,
    pub(crate) id: Option<Id<'a>>,
    pub(crate) text: Cow<'a, str>,
    pub(crate) allows_styling: bool,
}

impl<'a> Title<'a> {
    /// The category for this [`Report`]
    ///
    /// Useful for looking searching for more information to resolve the diagnostic.
    ///
    /// <div class="warning">
    ///
    /// Text passed to this function is considered "untrusted input", as such
    /// all text is passed through a normalization function. Styled text is
    /// not allowed to be passed to this function.
    ///
    /// </div>
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.id.get_or_insert(Id::default()).id = Some(id.into());
        self
    }

    /// Provide a URL for [`Title::id`] for more information on this diagnostic
    ///
    /// <div class="warning">
    ///
    /// This is only relevant if `id` is present
    ///
    /// </div>
    pub fn id_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
        self.id.get_or_insert(Id::default()).url = Some(url.into());
        self
    }

    /// Append an [`Element`] that adds context to the [`Title`]
    pub fn element(self, section: impl Into<Element<'a>>) -> Group<'a> {
        Group::with_title(self).element(section)
    }

    /// Append [`Element`]s that adds context to the [`Title`]
    pub fn elements(self, sections: impl IntoIterator<Item = impl Into<Element<'a>>>) -> Group<'a> {
        Group::with_title(self).elements(sections)
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
///
/// `Snippet`s come in the following styles (`T`):
/// - With [`Annotation`]s, see [`Snippet::annotation`]
/// - With [`Patch`]s, see [`Snippet::patch`]
#[derive(Clone, Debug)]
pub struct Snippet<'a, T> {
    pub(crate) path: Option<Cow<'a, str>>,
    pub(crate) line_start: usize,
    pub(crate) source: Cow<'a, str>,
    pub(crate) markers: Vec<T>,
    pub(crate) url: Option<PathUrlFormatterWrapper<'a>>,
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
            url: None,
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

    /// The URL to the [`source`][Self::source]. Can be a `file://` URL.
    ///
    /// Make sure to follow [the OSC 8 reference document][OSC-8], and in
    /// particular the [`file://` URIs and the hostname][OSC-8-file-URIs] when
    /// generating a `file://` URL.
    ///
    /// This function accepts two different kinds of arguments:
    ///
    /// - Some kind of string (`&str`, `String` or `Cow<str>`), used as is as a
    ///   URL.
    /// - An `impl Fn(usize, usize) -> String` that returns a URL from the line
    ///   and column computed by `annotate_snippets`.
    ///
    /// The following two snippets render the same:
    ///
    /// ```
    /// # use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};
    /// #
    /// # const HOSTNAME: &str = "sample_hostname";
    /// const SOURCE: &str = "hello; world!";
    ///
    /// let string_snippet = Snippet::source(SOURCE)
    ///     .path("file.txt")
    ///     .path_url(format!("file://{HOSTNAME}/path/to/file.txt:1"))
    ///     .annotation(AnnotationKind::Primary.span(5..5).label("expected `,`"));
    ///
    /// let formatter_snippet = Snippet::source(SOURCE)
    ///     .path("file.txt")
    ///     .path_url(|line, _col| format!("file://{HOSTNAME}/path/to/file.txt:{line}"))
    ///     .annotation(AnnotationKind::Primary.span(5..5).label("expected `,`"));
    /// #
    /// # let string_report = [
    /// #     Group::with_level(Level::ERROR)
    /// #         .element(string_snippet)
    /// # ];
    /// # let formatter_report = [
    /// #     Group::with_level(Level::ERROR)
    /// #         .element(formatter_snippet)
    /// # ];
    /// # let renderer = Renderer::plain();
    /// # let string_render = renderer.render(&string_report);
    /// # let formatter_render = renderer.render(&formatter_report);
    /// # assert_eq!(string_render, formatter_render);
    /// ```
    ///
    /// One can design reusable formatters using the following pattern:
    ///
    /// ```
    /// # use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};
    /// #
    /// # const HOSTNAME: &str = "sample_hostname";
    /// # const SOURCE: &str = "";
    /// #
    /// let file_url = |path| {
    ///     move |line, _col| format!("file://{HOSTNAME}/{path}:{line}")
    /// };
    ///
    /// let snippet = Snippet::source(SOURCE)
    ///     .path("foo.txt")
    ///     .path_url(file_url("/path/to/foo.txt"));
    /// #
    /// # let snippet: Snippet<()> = snippet;
    /// ```
    ///
    /// [OSC-8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
    /// [OSC-8-file-URIs]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda#file-uris-and-the-hostname
    pub fn path_url(mut self, url: impl Into<OptionPathUrlFormatter<'a>>) -> Self {
        self.url = url.into().0.map(PathUrlFormatterWrapper);
        self
    }

    /// Control whether lines without [`Annotation`]s are shown
    ///
    /// The default is `fold(true)`, collapsing uninteresting lines.
    ///
    /// See [`AnnotationKind::Visible`] to force specific spans to be shown.
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

    pub(crate) fn primary_span(&self) -> Option<Range<usize>> {
        self.markers
            .iter()
            .find_map(|marker| marker.kind.is_primary().then(|| marker.span.clone()))
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

/// Highlight and describe a span of text within a [`Snippet`]
///
/// See [`AnnotationKind`] to create an annotation.
///
/// # Example
///
/// ```rust
/// # #[allow(clippy::needless_doctest_main)]
#[doc = include_str!("../examples/expected_type.rs")]
/// ```
///
#[doc = include_str!("../examples/expected_type.svg")]
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
    ///
    /// This gives extra emphasis to this annotation
    pub fn highlight_source(mut self, highlight_source: bool) -> Self {
        self.highlight_source = highlight_source;
        self
    }
}

/// The type of [`Annotation`] being applied to a [`Snippet`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum AnnotationKind {
    /// For showing the source that the [Group's Title][Group::with_title] references
    ///
    /// For [`Title`]-less groups, see [`Group::with_level`]
    Primary,
    /// Additional context to better understand the [`Primary`][Self::Primary]
    /// [`Annotation`]
    ///
    /// See also [`Renderer::context`].
    ///
    /// [`Renderer::context`]: crate::renderer::Renderer
    Context,
    /// Prevents the annotated text from getting [folded][Snippet::fold]
    ///
    /// By default, [`Snippet`]s will [fold][`Snippet::fold`] (remove) lines
    /// that do not contain any annotations. [`Visible`][Self::Visible] makes
    /// it possible to selectively prevent this behavior for specific text,
    /// allowing context to be preserved without adding any annotation
    /// characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[allow(clippy::needless_doctest_main)]
    #[doc = include_str!("../examples/struct_name_as_context.rs")]
    /// ```
    ///
    #[doc = include_str!("../examples/struct_name_as_context.svg")]
    ///
    Visible,
}

impl AnnotationKind {
    /// Annotate a byte span within [`Snippet`]
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
///
/// See [`Snippet::patch`]
///
/// # Example
///
/// ```rust
/// # #[allow(clippy::needless_doctest_main)]
#[doc = include_str!("../examples/multi_suggestion.rs")]
/// ```
///
#[doc = include_str!("../examples/multi_suggestion.svg")]
#[derive(Clone, Debug)]
pub struct Patch<'a> {
    pub(crate) span: Range<usize>,
    pub(crate) replacement: Cow<'a, str>,
}

impl<'a> Patch<'a> {
    /// Splice `replacement` into the [`Snippet`] at the specified byte span
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

    /// Try to turn a replacement into an addition when the span that is being
    /// overwritten matches either the prefix or suffix of the replacement.
    pub(crate) fn trim_trivial_replacements(self, source: &str) -> TrimmedPatch<'a> {
        let mut trimmed = TrimmedPatch {
            original_span: self.span.clone(),
            span: self.span,
            replacement: self.replacement,
        };

        if trimmed.replacement.is_empty() {
            return trimmed;
        }
        let Some(snippet) = source.get(trimmed.original_span.clone()) else {
            return trimmed;
        };

        if let Some((prefix, substr, suffix)) = as_substr(snippet, &trimmed.replacement) {
            trimmed.span = trimmed.original_span.start + prefix
                ..trimmed.original_span.end.saturating_sub(suffix);
            trimmed.replacement = Cow::Owned(substr.to_owned());
        }
        trimmed
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
/// let report = &[
///     Level::ERROR.primary_title("mismatched types").id("E0308")
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
    pub(crate) url: Option<Cow<'a, str>>,
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
            url: None,
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

    /// The URL to the origin. Can be a `file://` URL.
    ///
    /// Make sure to follow [the OSC 8 reference document][OSC-8], and in
    /// particular the [`file://` URIs and the hostname][OSC-8-file-URIs] when
    /// generating a `file://` URL.
    ///
    /// [OSC-8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
    /// [OSC-8-file-URIs]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda#file-uris-and-the-hostname
    pub fn path_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
        self.url = Some(url.into());
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

// Wrapper over `Rc<dyn PathUrlFormatter>` that implements both `Clone` and `Debug`.
#[derive(Clone)]
pub(crate) struct PathUrlFormatterWrapper<'a>(pub(crate) Rc<dyn PathUrlFormatter + 'a>);

impl<'a> Debug for PathUrlFormatterWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        debug_path_url_formatter(&self.0).fmt(f)
    }
}

pub struct OptionPathUrlFormatter<'a>(pub(crate) Option<Rc<dyn PathUrlFormatter + 'a>>);

impl<'a> Debug for OptionPathUrlFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.as_ref().map(debug_path_url_formatter).fmt(f)
    }
}

// Workaround edge cases in `clippy::needless_lifetime::needless_lifetimes`.
// https://github.com/rust-lang/rust-clippy/issues/10495
// https://github.com/rust-lang/rust-clippy/issues/12908
#[allow(clippy::needless_lifetimes)]
// FIXME: when MSRV is bumped to >= 1.93.0, replace this with `std::fmt::from_fn`.
fn debug_path_url_formatter<'a>(
    formatter: &'a impl Borrow<dyn PathUrlFormatter + 'a>,
) -> impl Debug {
    struct D<'a>(&'a dyn PathUrlFormatter);

    impl<'a> Debug for D<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self.0.debug_literal() {
                Some(str) => Debug::fmt(str, f),
                None => Display::fmt("dyn PathUrlFormatter", f),
            }
        }
    }

    D(formatter.borrow())
}

impl<'a> OptionPathUrlFormatter<'a> {
    fn some(value: impl PathUrlFormatter + 'a) -> Self {
        OptionPathUrlFormatter(Some(Rc::new(value)))
    }

    fn none() -> Self {
        OptionPathUrlFormatter(None)
    }
}

pub trait PathUrlFormatter {
    fn format_url(&self, line: usize, byte_col: usize) -> String;

    // Allows to provide a consistent `Debug` implementation for `String`,
    // `&str` and `Cow<str>`. Not exposed to the user.
    #[doc(hidden)]
    fn debug_literal(&self) -> Option<&str> {
        None
    }
}

impl<'a> PathUrlFormatter for PathUrlFormatterWrapper<'a> {
    fn format_url(&self, line: usize, byte_col: usize) -> String {
        self.0.format_url(line, byte_col)
    }

    #[doc(hidden)]
    fn debug_literal(&self) -> Option<&str> {
        self.0.debug_literal()
    }
}

impl<'a> From<&'a str> for OptionPathUrlFormatter<'a> {
    fn from(value: &'a str) -> Self {
        Self::some(value)
    }
}

impl<'a> From<String> for OptionPathUrlFormatter<'a> {
    fn from(value: String) -> Self {
        Self::some(value)
    }
}

impl<'a> From<Cow<'a, str>> for OptionPathUrlFormatter<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self::some(value)
    }
}

impl<'a, T> From<T> for OptionPathUrlFormatter<'a>
where
    T: Fn(usize, usize) -> String + 'a,
{
    fn from(value: T) -> Self {
        Self::some(value)
    }
}

impl<'a, T> From<Option<T>> for OptionPathUrlFormatter<'a>
where
    T: Fn(usize, usize) -> String + 'a,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => OptionPathUrlFormatter::some(value),
            None => OptionPathUrlFormatter::none(),
        }
    }
}

impl PathUrlFormatter for &str {
    fn format_url(&self, _line: usize, _byte_col: usize) -> String {
        self.to_string()
    }

    #[doc(hidden)]
    fn debug_literal(&self) -> Option<&str> {
        Some(self)
    }
}

impl PathUrlFormatter for String {
    fn format_url(&self, _line: usize, _byte_col: usize) -> String {
        self.clone()
    }

    #[doc(hidden)]
    fn debug_literal(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> PathUrlFormatter for Cow<'a, str> {
    fn format_url(&self, _line: usize, _byte_col: usize) -> String {
        self.to_string()
    }

    #[doc(hidden)]
    fn debug_literal(&self) -> Option<&str> {
        Some(self)
    }
}

impl<T> PathUrlFormatter for T
where
    T: Fn(usize, usize) -> String,
{
    fn format_url(&self, line: usize, byte_col: usize) -> String {
        self(line, byte_col)
    }
}
