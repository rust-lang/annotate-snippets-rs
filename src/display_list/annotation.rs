use std::fmt;

#[derive(Debug, Clone)]
pub struct Annotation<'d> {
    pub annotation_type: DisplayAnnotationType,
    pub id: Option<&'d str>,
    pub label: &'d str,
}

impl<'d> fmt::Display for Annotation<'d> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label)
    }
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

impl fmt::Display for DisplayAnnotationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::Error => f.write_str("error"),
            Self::Warning => f.write_str("warning"),
            Self::Info => f.write_str("info"),
            Self::Note => f.write_str("note"),
            Self::Help => f.write_str("help"),
        }
    }
}
