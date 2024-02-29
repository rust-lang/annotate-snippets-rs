#![deny(rust_2018_idioms)]

//! A library for formatting of text or programming code snippets.
//!
//! It's primary purpose is to build an ASCII-graphical representation of the snippet
//! with annotations.
//!
//! # Example
//!
//! ```text
//! error[E0308]: mismatched types
//!   --> src/format.rs:52:1
//!    |
//! 51 |   ) -> Option<String> {
//!    |        -------------- expected `Option<String>` because of return type
//! 52 | /     for ann in annotations {
//! 53 | |         match (ann.range.0, ann.range.1) {
//! 54 | |             (None, None) => continue,
//! 55 | |             (Some(start), Some(end)) if start > end_index => continue,
//! ...  |
//! 71 | |         }
//! 72 | |     }
//!    | |_____^ expected enum `std::option::Option`, found ()
//! ```
//!
//! The crate uses a three stage process with two conversions between states:
//!
//! ```text
//! Snippet --> Renderer --> impl Display
//! ```
//!
//! The input type - [Snippet] is a structure designed
//! to align with likely output from any parser whose code snippet is to be
//! annotated.
//!
//! The middle structure - [Renderer] is a structure designed
//! to convert a snippet into an internal structure that is designed to store
//! the snippet data in a way that is easy to format.
//! [Renderer] also handles the user-configurable formatting
//! options, such as color, or margins.
//!
//! Finally, `impl Display` into a final `String` output.
//!
//! # features
//! - `testing-colors` - Makes [Renderer::styled] colors OS independent, which
//! allows for easier testing when testing colored output. It should be added as
//! a feature in `[dev-dependencies]`, which can be done with the following command:
//! ```text
//! cargo add annotate-snippets --dev --feature testing-colors
//! ```
//!

pub mod renderer;
mod snippet;

#[doc(inline)]
pub use renderer::Renderer;
pub use snippet::*;
