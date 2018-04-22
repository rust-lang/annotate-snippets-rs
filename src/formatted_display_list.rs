use display_list::{DisplayLine, DisplayList, DisplayMark};
use std::fmt;

pub struct FormattedDisplayList {
    body: Vec<FormattedDisplayLine>,
}

impl From<DisplayList> for FormattedDisplayList {
    fn from(dl: DisplayList) -> Self {
        let max_lineno = 3;
        let body = dl.body
            .into_iter()
            .map(|line| FormattedDisplayLine::format(line, max_lineno))
            .collect();
        FormattedDisplayList { body }
    }
}

impl fmt::Display for FormattedDisplayList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some((last, elements)) = &self.body.split_last() {
            for line in elements.iter() {
                line.fmt(f)?;
                write!(f, "\n")?;
            }
            last.fmt(f)?;
        }
        Ok(())
    }
}

enum FormattedDisplayLine {
    RawLine(String),
    SourceLine {
        lineno: String,
        inline_marks: String,
        content: String,
    },
    AnnotationLine {
        lineno: String,
        inline_marks: String,
        content: String,
    },
    FoldLine,
}

impl FormattedDisplayLine {
    fn format(dl: DisplayLine, max_lineno: usize) -> Self {
        match dl {
            DisplayLine::RawLine(s) => FormattedDisplayLine::RawLine(s),
            DisplayLine::SourceLine {
                lineno,
                inline_marks,
                content,
            } => FormattedDisplayLine::SourceLine {
                lineno: format!("{: >width$}", lineno, width = max_lineno),
                inline_marks: Self::format_inline_marks(&inline_marks),
                content,
            },
            DisplayLine::AnnotationLine {
                inline_marks,
                range,
                label,
            } => FormattedDisplayLine::AnnotationLine {
                lineno: " ".repeat(max_lineno),
                inline_marks: Self::format_inline_marks(&inline_marks),
                content: Self::format_annotation_content(range, label),
            },
            DisplayLine::FoldLine => FormattedDisplayLine::FoldLine,
        }
    }

    fn format_inline_marks(inline_marks: &[DisplayMark]) -> String {
        format!(
            "{}",
            inline_marks
                .iter()
                .map(|mark| format!("{}", mark))
                .collect::<Vec<String>>()
                .join("")
        )
    }

    fn format_annotation_content(range: (usize, usize), label: String) -> String {
        format!(
            "{}{} {}",
            " ".repeat(range.0),
            "^".repeat(range.1 - range.0),
            label
        )
    }
}

impl fmt::Display for FormattedDisplayLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormattedDisplayLine::SourceLine {
                lineno,
                inline_marks,
                content,
            } => write!(f, "{} | {}{}", lineno, inline_marks, content),
            FormattedDisplayLine::RawLine(body) => write!(f, "{}", body),
            FormattedDisplayLine::AnnotationLine {
                lineno,
                inline_marks,
                content,
            } => write!(f, "{} | {}{}", lineno, inline_marks, content),
            FormattedDisplayLine::FoldLine => write!(f, " ... |"),
        }
    }
}
