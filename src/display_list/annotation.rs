use crate::annotation::AnnotationType;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Annotation<'d> {
    pub annotation_type: AnnotationType,
    pub id: Option<&'d str>,
    pub label: &'d str,
}

impl fmt::Display for AnnotationType {
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
