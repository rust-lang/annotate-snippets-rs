#![feature(drain_filter)]
//! Annotate Snippets is a library for formatting of text or programming code snippets.
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
//! # Input
//!
//! On the input Annotate Snippets takes a Slice of text, a list of Annotation objects
//! and some optional meta information.
//!
//! An example input data to produce the above output would be:
//!
//! ```
//! # use annotate_snippets::snippet::{Snippet, Slice, Annotation, AnnotationType};
//! Snippet {
//!     slice: Slice {
//!         source: r#"
//!         ) -> Option<String> {
//!             for ann in annotations {
//!                 match (ann.range.0, ann.range.1) {
//!                     (None, None) => continue,
//!                     (Some(start), Some(end)) if start > end_index => continue,
//!                     (Some(start), Some(end)) if start >= start_index => {
//!                         let label = if let Some(ref label) = ann.label {
//!                             format!(" {}", label)
//!                         } else {
//!                             String::from("")
//!                         };
//!
//!                         return Some(format!(
//!                             "{}{}{}",
//!                             " ".repeat(start - start_index),
//!                             "^".repeat(end - start),
//!                             label
//!                         ));
//!                     }
//!                     _ => continue,
//!                 }
//!             }
//!         "#.to_string(),
//!         line_start: 51,
//!         origin: Some("src/format.rs".to_string())
//!     },
//!     title_annotation_pos: Some(0),
//!     main_annotation_pos: Some(0),
//!     fold: Some(true),
//!     annotations: vec![
//!         Annotation {
//!             label: Some("mismatched types".to_string()),
//!             id: Some("E0308".to_string()),
//!             annotation_type: AnnotationType::Error,
//!             range: (None, None)
//!         },
//!         Annotation {
//!             label: Some("expected `Option<String>` because of return type".to_string()),
//!             id: None,
//!             annotation_type: AnnotationType::Warning,
//!             range: (Some(6), Some(20))
//!         },
//!         Annotation {
//!             label: Some("expected enum `std::option::Option".to_string()),
//!             id: None,
//!             annotation_type: AnnotationType::Error,
//!             range: (Some(23), Some(787))
//!         },
//!     ]
//! };
//! ```

mod display_list;
mod format;
mod formatted_display_list;

pub mod snippet;

pub use format::format_snippet;
