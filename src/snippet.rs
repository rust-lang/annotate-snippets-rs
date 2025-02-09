//! Structures used as an input for the library.

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

#[derive(Clone, Debug)]
pub struct Annotation<'a> {
    pub(crate) range: Range<usize>,
    pub(crate) label: Option<&'a str>,
    pub(crate) kind: AnnotationKind,
}

impl<'a> Annotation<'a> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
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
        }
    }

    pub(crate) fn is_primary(&self) -> bool {
        matches!(self, AnnotationKind::Primary)
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
