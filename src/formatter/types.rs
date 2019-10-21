use crate::{DebugAndDisplay, Level, Message, Title};
use std::fmt;

// Cannot derive Debug, Clone because we need to bound Span::Subspan
// so #[derive(Debug, Clone)] is manually expanded here (ugh)

pub struct FormattedSnippet<'d, Span: crate::Span> {
    pub lines: Vec<DisplayLine<'d, Span>>,
}

impl<Span: crate::Span> fmt::Debug for FormattedSnippet<'_, Span>
where
    Span: fmt::Debug,
    Span::Subspan: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FormattedSnippet")
            .field("inner", &self.lines)
            .finish()
    }
}

impl<Span: crate::Span> Clone for FormattedSnippet<'_, Span>
where
    Span::Subspan: Clone,
{
    fn clone(&self) -> Self {
        FormattedSnippet {
            lines: self.lines.clone(),
        }
    }
}

pub enum DisplayLine<'d, Span: crate::Span> {
    Source {
        lineno: Option<usize>,
        inline_marks: Vec<Mark>,
        line: SourceLine<'d, Span>,
    },
    Raw(RawLine<'d>),
}

// #[derive(Debug)]
impl<Span: crate::Span> fmt::Debug for DisplayLine<'_, Span>
where
    Span: fmt::Debug,
    Span::Subspan: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisplayLine::Source {
                lineno,
                inline_marks,
                line,
            } => f
                .debug_struct("Source")
                .field("lineno", lineno)
                .field("inline_marks", inline_marks)
                .field("line", line)
                .finish(),
            DisplayLine::Raw(raw) => f.debug_tuple("Raw").field(raw).finish(),
        }
    }
}

// #[derive(Clone)]
impl<Span: crate::Span> Clone for DisplayLine<'_, Span>
where
    Span::Subspan: Clone,
{
    fn clone(&self) -> Self {
        match self {
            DisplayLine::Source {
                lineno,
                inline_marks,
                line,
            } => DisplayLine::Source {
                lineno: *lineno,
                inline_marks: inline_marks.clone(),
                line: (*line).clone(),
            },
            DisplayLine::Raw(raw) => DisplayLine::Raw(*raw),
        }
    }
}

pub enum SourceLine<'d, Span: crate::Span> {
    Content {
        span: &'d Span,
        subspan: Span::Subspan,
    },
    Annotation {
        message: Option<Message<'d>>,
        underline: (usize, usize),
    },
    Empty,
}

// #[derive(Debug)]
impl<Span: crate::Span> fmt::Debug for SourceLine<'_, Span>
where
    Span: fmt::Debug,
    Span::Subspan: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SourceLine::Content { span, subspan } => f
                .debug_struct("Content")
                .field("span", span)
                .field("subspan", subspan)
                .finish(),
            SourceLine::Annotation { message, underline } => f
                .debug_struct("Annotation")
                .field("message", message)
                .field("underline", underline)
                .finish(),
            SourceLine::Empty => f.debug_struct("Empty").finish(),
        }
    }
}

// #[derive(Copy)]
impl<Span: crate::Span> Copy for SourceLine<'_, Span> where Span::Subspan: Copy {}

// #[derive(Clone)]
impl<Span: crate::Span> Clone for SourceLine<'_, Span>
where
    Span::Subspan: Clone,
{
    fn clone(&self) -> Self {
        match self {
            SourceLine::Content { span, subspan } => SourceLine::Content {
                span: span.clone(),
                subspan: subspan.clone(),
            },
            SourceLine::Annotation { message, underline } => SourceLine::Annotation {
                message: *message,
                underline: *underline,
            },
            SourceLine::Empty => SourceLine::Empty,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RawLine<'d> {
    Origin {
        path: &'d dyn DebugAndDisplay,
        pos: Option<(usize, usize)>,
    },
    Title {
        title: Title<'d>,
    },
    Message {
        message: Message<'d>,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct Mark {
    pub kind: MarkKind,
    pub level: Level,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MarkKind {
    Start,
    Continue,
    Here,
}
