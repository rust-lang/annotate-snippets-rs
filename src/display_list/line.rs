use std::fmt;
use std::fmt::Write;

use super::annotation::Annotation;

#[derive(Debug, Clone)]
pub enum DisplayLine<'d> {
    Source {
        lineno: Option<usize>,
        line: DisplaySourceLine<'d>,
    },
    Raw(DisplayRawLine<'d>),
}

impl<'d> DisplayLine<'d> {
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>, lineno_max: Option<usize>) -> fmt::Result {
        let lineno_max = lineno_max.unwrap_or(1);
        match self {
            Self::Source { lineno, line } => {
                if let Some(lineno) = lineno {
                    write!(f, "{:>1$}", lineno, lineno_max)?;
                    writeln!(f, " | {}", line)
                } else {
                    write!(f, "{:>1$}", "", lineno_max)?;
                    writeln!(f, " | {}", line)
                }
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
            Self::Content { text } => f.write_str(text),
            Self::Annotation {
                annotation,
                range: (start, end),
            } => {
                write!(f, "{:>1$}", "", start)?;
                write!(f, "{:->1$} ", "", end - start)?;
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
