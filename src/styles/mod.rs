use crate::AnnotationType;
use std::fmt;
use std::fmt::Display;

#[cfg(feature = "ansi_term")]
pub mod color;
pub mod plain;

pub enum StyleClass {
    TitleLineAnnotationType,
    TitleLine,
    AnnotationTypeError,
}

pub trait Stylesheet {
    fn format(
        &self,
        f: &mut fmt::Formatter,
        style: &[StyleClass],
        value: impl Display,
    ) -> fmt::Result;
}

pub fn get_stylesheet() -> impl Stylesheet {
    #[cfg(feature = "ansi_term")]
    return color::StylesheetColor::default();

    #[cfg(not(feature = "ansi_term"))]
    return plain::StylesheetPlain::default();
}
