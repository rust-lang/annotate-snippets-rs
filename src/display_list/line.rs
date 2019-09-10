use std::fmt;
use std::fmt::Write;

use super::annotation::Annotation;

#[derive(Debug, Clone)]
pub enum DisplayLine<'d> {
    Source {
        lineno: Option<usize>,
        inline_marks: Vec<DisplayMark>,
        line: DisplaySourceLine<'d>,
    },
    Raw(DisplayRawLine<'d>),
}

impl<'d> DisplayLine<'d> {
    pub fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
        lineno_max: Option<usize>,
        inline_marks_width: usize,
    ) -> fmt::Result {
        let lineno_max = lineno_max.unwrap_or(1);
        match self {
            Self::Source {
                lineno,
                inline_marks,
                line,
            } => {
                if let Some(lineno) = lineno {
                    write!(f, "{:>1$}", lineno, lineno_max)?;
                } else {
                    write!(f, "{:>1$}", "", lineno_max)?;
                }
                f.write_str(" | ")?;
                write!(f, "{:>1$}", "", inline_marks_width - inline_marks.len())?;
                for mark in inline_marks {
                    write!(f, "{}", mark)?;
                }
                writeln!(f, "{}", line)
            }
            Self::Raw(dl) => dl.fmt(f, lineno_max),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DisplaySourceLine<'d> {
    Content {
        text: &'d str,
    },
    Annotation {
        annotation: Annotation<'d>,
        range: (usize, usize),
    },
    Empty,
}

impl<'d> fmt::Display for DisplaySourceLine<'d> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Content { text } => {
                f.write_char(' ')?;
                f.write_str(text)
            }
            Self::Annotation {
                annotation,
                range: (start, end),
            } => {
                let indent = if start == &0 { 0 } else { start + 1 };
                write!(f, "{:>1$}", "", indent)?;
                if start == &0 {
                    write!(f, "{:_>1$}", "^", end - start + 1)?;
                } else {
                    write!(f, "{:->1$}", "", end - start)?;
                }
                f.write_char(' ')?;
                annotation.fmt(f)
            }
            Self::Empty => Ok(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DisplayRawLine<'d> {
    Origin {
        path: &'d str,
        pos: (Option<usize>, Option<usize>),
    },
}

impl<'d> DisplayRawLine<'d> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, lineno_max: usize) -> fmt::Result {
        match self {
            Self::Origin { path, pos } => {
                write!(f, "{:>1$}", "", lineno_max)?;
                write!(f, "--> {}", path)?;
                if let Some(line) = pos.0 {
                    write!(f, ":{}", line)?;
                }
                f.write_char('\n')
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplayMark {
    pub mark_type: DisplayMarkType,
    pub annotation_type: DisplayAnnotationType,
}

impl fmt::Display for DisplayMark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mark_type {
            DisplayMarkType::AnnotationStart => f.write_char('/'),
            DisplayMarkType::AnnotationThrough => f.write_char('|'),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DisplayMarkType {
    AnnotationThrough,
    AnnotationStart,
}

#[derive(Debug, Clone)]
pub enum DisplayAnnotationType {
    None,
    Error,
    Warning,
    Info,
    Note,
    Help,
}
