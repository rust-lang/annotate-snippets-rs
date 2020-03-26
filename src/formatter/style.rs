//! Set of structures required to implement a stylesheet for
//! [DisplayListFormatter](super::DisplayListFormatter).
//!
//! In order to provide additional styling information for the
//! formatter, a structs can implement `Stylesheet` and `Style`
//! traits.
//!
use std::fmt;

/// StyleClass is a collection of named variants of style classes
/// that DisplayListFormatter uses.
pub enum StyleClass {
    /// Message indicating an error.
    Error,
    /// Message indicating a warning.
    Warning,
    /// Message indicating an information.
    Info,
    /// Message indicating a note.
    Note,
    /// Message indicating a help.
    Help,

    /// Style for line numbers.
    LineNo,

    /// Parts of the text that are to be emphasised.
    Emphasis,

    /// Parts of the text that are regular. Usually a no-op.
    None,
}

/// This trait implements a return value for the `Stylesheet::get_style`.
pub trait Style {
    /// The method used by the DisplayListFormatter to style the message.
    fn paint(&self, text: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    /// The method used by the DisplayListFormatter to display the message
    /// in bold font.
    fn bold(&self) -> Box<dyn Style>;
}

/// Trait to annotate structs that can provide `Style` implementations for
/// every `StyleClass` variant.
pub trait Stylesheet {
    /// Returns a `Style` implementer based on the requested `StyleClass` variant.
    fn get_style(&self, class: StyleClass) -> Box<dyn Style>;
}
