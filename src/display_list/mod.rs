//! display_list module stores the output model for the snippet.
//!
//! `DisplayList` is a central structure in the crate, which contains
//! the structured list of lines to be displayed.
//!
//! It is made of two types of lines: `Source` and `Raw`. All `Source` lines
//! are structured using four columns:
//!
//! ```text
//!  /------------ (1) Line number column.
//!  |  /--------- (2) Line number column delimiter.
//!  |  | /------- (3) Inline marks column.
//!  |  | |   /--- (4) Content column with the source and annotations for slices.
//!  |  | |   |
//! =============================================================================
//! error[E0308]: mismatched types
//!    --> src/format.rs:51:5
//!     |
//! 151 | /   fn test() -> String {
//! 152 | |       return "test";
//! 153 | |   }
//!     | |___^ error: expected `String`, for `&str`.
//!     |
//! ```
//!
//! The first two lines of the example above are `Raw` lines, while the rest
//! are `Source` lines.
//!
//! `DisplayList` does not store column alignment information, and those are
//! only calculated by the `DisplayListFormatter` using information such as
//! styling.
//!
//! The above snippet has been built out of the following structure:
//!
//! ```
//! use annotate_snippets::display_list::*;
//!
//! let dl = DisplayList {
//!     body: vec![
//!         DisplayLine::Raw(DisplayRawLine::Annotation {
//!             annotation: Annotation {
//!                 annotation_type: DisplayAnnotationType::Error,
//!                 id: Some("E0308".to_string()),
//!                 label: vec![
//!                     DisplayTextFragment {
//!                         content: "mismatched types".to_string(),
//!                         style: DisplayTextStyle::Regular,
//!                     }
//!                 ]
//!             },
//!             source_aligned: false,
//!             continuation: false,
//!         }),
//!         DisplayLine::Raw(DisplayRawLine::Origin {
//!             path: "src/format.rs".to_string(),
//!             pos: Some((51, 5)),
//!             header_type: DisplayHeaderType::Initial,
//!         }),
//!         DisplayLine::Source {
//!             lineno: Some(151),
//!             inline_marks: vec![
//!                 DisplayMark {
//!                     mark_type: DisplayMarkType::AnnotationStart,
//!                     annotation_type: DisplayAnnotationType::Error,
//!                 }
//!             ],
//!             line: DisplaySourceLine::Content {
//!                 text: "  fn test() -> String {".to_string(),
//!                 range: (0, 24)
//!             }
//!         },
//!         DisplayLine::Source {
//!             lineno: Some(152),
//!             inline_marks: vec![
//!                 DisplayMark {
//!                     mark_type: DisplayMarkType::AnnotationThrough,
//!                     annotation_type: DisplayAnnotationType::Error,
//!                 }
//!             ],
//!             line: DisplaySourceLine::Content {
//!                 text: "      return \"test\";".to_string(),
//!                 range: (25, 46)
//!             }
//!         },
//!         DisplayLine::Source {
//!             lineno: Some(153),
//!             inline_marks: vec![
//!                 DisplayMark {
//!                     mark_type: DisplayMarkType::AnnotationThrough,
//!                     annotation_type: DisplayAnnotationType::Error,
//!                 }
//!             ],
//!             line: DisplaySourceLine::Content {
//!                 text: "  }".to_string(),
//!                 range: (47, 51)
//!             }
//!         },
//!         DisplayLine::Source {
//!             lineno: None,
//!             inline_marks: vec![],
//!             line: DisplaySourceLine::Annotation {
//!                 annotation: Annotation {
//!                     annotation_type: DisplayAnnotationType::Error,
//!                     id: None,
//!                     label: vec![
//!                         DisplayTextFragment {
//!                             content: "expected `String`, for `&str`.".to_string(),
//!                             style: DisplayTextStyle::Regular,
//!                         }
//!                     ]
//!                 },
//!                 range: (3, 4),
//!                 annotation_type: DisplayAnnotationType::Error,
//!                 annotation_part: DisplayAnnotationPart::MultilineEnd,
//!             }
//!
//!         }
//!     ]
//! };
//! ```
mod from_snippet;
mod structs;

pub use self::structs::*;
