pub struct FormattedDisplayList {
    pub body: Vec<FormattedDisplayLine>,
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
