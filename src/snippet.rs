//! Snippet module contains structures used to generate the Snippet to be formatted.
//!
//! # Example:
//!
//! ```
//! use annotate_snippets::snippet::{Snippet, Slice, Annotation, TitleAnnotation, AnnotationType};
//!
//! let snippet = Snippet {
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
//!     title: Some(TitleAnnotation {
//!         label: Some("mismatched types".to_string()),
//!         id: Some("E0308".to_string()),
//!         annotation_type: AnnotationType::Error,
//!     }),
//!     main_annotation_pos: Some(0),
//!     fold: Some(true),
//!     annotations: vec![
//!         Annotation {
//!             label: Some("expected `Option<String>` because of return type".to_string()),
//!             annotation_type: AnnotationType::Warning,
//!             range: Some((6, 20))
//!         },
//!         Annotation {
//!             label: Some("expected enum `std::option::Option".to_string()),
//!             annotation_type: AnnotationType::Error,
//!             range: Some((23, 787))
//!         },
//!     ]
//! };
//! let output = format!("{}", snippet);
//! ```

use display_list::DisplayList;
use formatted_display_list::FormattedDisplayList;
use std::fmt;

/// Primary structure provided for formatting
#[derive(Debug, Clone)]
pub struct Snippet {
    pub slice: Slice,
    pub annotations: Vec<Annotation>,
    /// Index of an Annotation to be used
    /// as a main one in the snippet (for the header part).
    pub main_annotation_pos: Option<usize>,
    pub title: Option<TitleAnnotation>,
    /// If set explicitly to `true`, the snippet will fold
    /// parts of the slice that don't contain any annotations.
    pub fold: Option<bool>,
}

impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dl = DisplayList::from(self.clone());
        let fdl = FormattedDisplayList::from(dl);
        write!(f, "{}", fdl)
    }
}

/// Structure containing the slice of text to be annotated and
/// basic information about the location of the slice.
#[derive(Debug, Clone)]
pub struct Slice {
    pub source: String,
    pub line_start: usize,
    pub origin: Option<String>,
}

/// Types of annotations.
#[derive(Debug, Clone, Copy)]
pub enum AnnotationType {
    /// Error annotations are displayed using red color and "^" character.
    Error,
    /// Warning annotations are displayed using blue color and "-" character.
    Warning,
}

/// An Annotation is a pointer to a place in the Slice which is to be annotated.
#[derive(Debug, Clone)]
pub struct Annotation {
    pub range: Option<(usize, usize)>,
    pub label: Option<String>,
    pub annotation_type: AnnotationType,
}

#[derive(Debug, Clone)]
pub struct TitleAnnotation {
    /// Identifier of the annotation. Usually error code like "E0308".
    pub id: Option<String>,
    pub label: Option<String>,
    pub annotation_type: AnnotationType,
}
