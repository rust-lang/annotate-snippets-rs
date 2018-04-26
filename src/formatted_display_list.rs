use display_list::{DisplayLine, DisplayList};

pub struct FormattedDisplayList {
    pub body: Vec<FormattedDisplayLine>,
}

impl From<DisplayList> for FormattedDisplayList {
    fn from(dl: DisplayList) -> Self {
        let lineno_width = dl.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { lineno, .. } => {
                let width = lineno.to_string().len();
                if width > max {
                    width
                } else {
                    max
                }
            }
            _ => max,
        });
        let inline_marks_width = dl.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => {
                let width = inline_marks.len();
                if width > max {
                    width + 1
                } else {
                    max
                }
            }
            _ => max,
        });
        let body = dl.body
            .into_iter()
            .map(|line| FormattedDisplayLine::format(line, lineno_width, inline_marks_width))
            .collect();
        FormattedDisplayList { body }
    }
}

#[derive(Debug)]
pub enum FormattedDisplayLine {
    Raw(String),
    EmptySource {
        lineno: String,
    },
    Source {
        lineno: String,
        inline_marks: String,
        content: String,
    },
    Annotation {
        lineno: String,
        inline_marks: String,
        content: String,
    },
    Fold,
}
