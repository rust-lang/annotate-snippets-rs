//! Set of structures required to implement a stylesheet for
//! [DisplayListFormatter](super::DisplayListFormatter).
//!
//! In order to provide additional styling information for the
//! formatter, a structs can implement `Stylesheet` and `Style`
//! traits.
//!
//! Example:
//!
//! ```
//! use annotate_snippets::formatter::style::{Stylesheet, StyleClass, Style};
//!
//! struct HTMLStyle {
//!   prefix: String,
//!   postfix: String,
//! };
//!
//! impl HTMLStyle {
//!   fn new(prefix: &str, postfix: &str) -> Self {
//!     HTMLStyle {
//!       prefix: prefix.into(),
//!       postfix: postfix.into()
//!     }
//!   }
//! };
//!
//! impl Style for HTMLStyle {
//!   fn paint(&self, text: &str) -> String {
//!     format!("{}{}{}", self.prefix, text, self.postfix)
//!   }
//!
//!   fn bold(&self) -> Box<Style> {
//!     Box::new(HTMLStyle {
//!       prefix: format!("{}<b>", self.prefix),
//!       postfix: format!("</b>{}", self.postfix),
//!     })
//!   }
//! }
//!
//! struct HTMLStylesheet {};
//!
//!
//! impl Stylesheet for HTMLStylesheet {
//!   fn get_style(&self, class: StyleClass) -> Box<Style> {
//!     let s = match class {
//!       StyleClass::Error => HTMLStyle::new("<span style='color:red'>", "</span>"),
//!       StyleClass::Warning => HTMLStyle::new("<span style='color:orange'>", "</span>"),
//!       StyleClass::Info => HTMLStyle::new("<span style='color:yellow'>", "</span>"),
//!       StyleClass::Note => HTMLStyle::new("<span style='color:blue'>", "</span>"),
//!       StyleClass::Help => HTMLStyle::new("<span style='color:green'>", "</span>"),
//!       StyleClass::LineNo => HTMLStyle::new("<strong>", "</strong>"),
//!       StyleClass::Emphasis => HTMLStyle::new("<i>", "</i>"),
//!       StyleClass::None => HTMLStyle::new("", ""),
//!     };
//!     Box::new(s)
//!   }
//! }
//! ```

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
    fn paint(&self, text: &str) -> String;
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
