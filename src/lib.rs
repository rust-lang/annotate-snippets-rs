#![feature(drain_filter)]
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
//! In order to produce such output, the user builds a
//! [Snippet](self::snippet::Snippet) which has a single public method: `format`.

pub mod display_list;
pub mod display_list_formatting;
#[cfg(feature = "ansi_term")]
pub mod format_color;
#[cfg(not(feature = "ansi_term"))]
pub mod format;
pub mod snippet;
