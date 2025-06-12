//! A library for formatting of text or programming code snippets.
//!
//! It's primary purpose is to build an ASCII-graphical representation of the snippet
//! with annotations.
//!
//! # Example
//!
//! ```rust
//! # #[allow(clippy::needless_doctest_main)]
#![doc = include_str!("../examples/expected_type.rs")]
//! ```
//!
#![doc = include_str!("../examples/expected_type.svg")]
//!
//! # features
//! - `testing-colors` - Makes [Renderer::styled] colors OS independent, which
//! allows for easier testing when testing colored output. It should be added as
//! a feature in `[dev-dependencies]`, which can be done with the following command:
//! ```text
//! cargo add annotate-snippets --dev --feature testing-colors
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(missing_debug_implementations)]

pub mod level;
pub mod renderer;
mod snippet;

/// Normalize the string to avoid any unicode control characters.
/// This is important for untrusted input, as it can contain
/// invalid unicode sequences.
pub fn normalize_untrusted_str(s: &str) -> String {
    renderer::normalize_whitespace(s)
}

#[doc(inline)]
pub use level::Level;
#[doc(inline)]
pub use renderer::Renderer;
pub use snippet::*;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
