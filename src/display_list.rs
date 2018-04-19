use snippet::Snippet;
use std::fmt;

pub struct DisplayList {
    body: Vec<DisplayLine>,
}

impl From<Snippet> for DisplayList {
    fn from(snippet: Snippet) -> Self {
        let mut body = vec![];
        let mut current_line = snippet.slice.line_start;
        for line in snippet.slice.source.lines() {
            body.push(DisplayLine::SourceLine {
                lineno: current_line,
                inline_marks: vec![],
                content: line.to_string(),
            });
            current_line += 1;
        }
        DisplayList { body }
    }
}

impl fmt::Display for DisplayList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|line| format!("{}", line))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub enum DisplayLine {
    RawLine(String),
    SourceLine {
        lineno: usize,
        inline_marks: Vec<DisplayMark>,
        content: String,
    },
}

impl DisplayLine {
    fn format_inline_marks(&self, inline_marks: &[DisplayMark]) -> String {
        "".to_string()
    }
}

impl fmt::Display for DisplayLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisplayLine::SourceLine {
                lineno,
                inline_marks,
                content,
            } => write!(
                f,
                "{} | {}{}",
                lineno,
                self.format_inline_marks(inline_marks),
                content
            ),
            DisplayLine::RawLine(body) => write!(f, "{}", body),
        }
    }
}

#[derive(Debug)]
pub enum DisplayMark {
    AnnotationThrough,
    AnnotationStart,
}
