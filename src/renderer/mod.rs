//! The renderer for [`Snippet`]s
//!
//! # Example
//! ```
//! use annotate_snippets::{Annotation, AnnotationType, Renderer, Slice, Snippet};
//! let snippet = Snippet {
//!     title: Some(Annotation {
//!         label: Some("mismatched types"),
//!         id: None,
//!         annotation_type: AnnotationType::Error,
//!     }),
//!     footer: vec![],
//!     slices: vec![
//!         Slice {
//!             source: "Foo",
//!             line_start: 51,
//!             origin: Some("src/format.rs"),
//!             fold: false,
//!             annotations: vec![],
//!         },
//!         Slice {
//!             source: "Faa",
//!             line_start: 129,
//!             origin: Some("src/display.rs"),
//!             fold: false,
//!             annotations: vec![],
//!         },
//!     ],
//!  };
//!
//!  let renderer = Renderer::styled();
//!  println!("{}", renderer.render(snippet));

mod display_list;
mod margin;
pub(crate) mod stylesheet;

use crate::snippet::Snippet;
pub use anstyle::*;
use display_list::DisplayList;
pub use margin::Margin;
use std::fmt::Display;
use stylesheet::Stylesheet;

/// A renderer for [`Snippet`]s
#[derive(Clone)]
pub struct Renderer {
    anonymized_line_numbers: bool,
    margin: Option<Margin>,
    stylesheet: Stylesheet,
}

impl Renderer {
    /// No terminal styling
    pub const fn plain() -> Self {
        Self {
            anonymized_line_numbers: false,
            margin: None,
            stylesheet: Stylesheet::plain(),
        }
    }

    /// Default terminal styling
    ///
    /// # Note
    /// When testing styled terminal output, see the [`testing-colors` feature](crate#features)
    pub const fn styled() -> Self {
        const USE_WINDOWS_COLORS: bool = cfg!(windows) && !cfg!(feature = "testing-colors");
        const BRIGHT_BLUE: Style = if USE_WINDOWS_COLORS {
            AnsiColor::BrightCyan.on_default()
        } else {
            AnsiColor::BrightBlue.on_default()
        };
        Self {
            stylesheet: Stylesheet {
                error: AnsiColor::BrightRed.on_default().effects(Effects::BOLD),
                warning: if USE_WINDOWS_COLORS {
                    AnsiColor::BrightYellow.on_default()
                } else {
                    AnsiColor::Yellow.on_default()
                }
                .effects(Effects::BOLD),
                info: BRIGHT_BLUE.effects(Effects::BOLD),
                note: AnsiColor::BrightGreen.on_default().effects(Effects::BOLD),
                help: AnsiColor::BrightCyan.on_default().effects(Effects::BOLD),
                line_no: BRIGHT_BLUE.effects(Effects::BOLD),
                emphasis: if USE_WINDOWS_COLORS {
                    AnsiColor::BrightWhite.on_default()
                } else {
                    Style::new()
                }
                .effects(Effects::BOLD),
                none: Style::new(),
            },
            ..Self::plain()
        }
    }

    /// Anonymize line numbers
    ///
    /// This enables (or disables) line number anonymization. When enabled, line numbers are replaced
    /// with `LL`.
    ///
    /// # Example
    ///
    /// ```text
    ///   --> $DIR/whitespace-trimming.rs:4:193
    ///    |
    /// LL | ...                   let _: () = 42;
    ///    |                                   ^^ expected (), found integer
    ///    |
    /// ```
    pub const fn anonymized_line_numbers(mut self, anonymized_line_numbers: bool) -> Self {
        self.anonymized_line_numbers = anonymized_line_numbers;
        self
    }

    /// Set the margin for the output
    ///
    /// This controls the various margins of the output.
    ///
    /// # Example
    ///
    /// ```text
    /// error: expected type, found `22`
    ///   --> examples/footer.rs:29:25
    ///    |
    /// 26 | ...         annotations: vec![SourceAnnotation {
    ///    |                               ---------------- info: while parsing this struct
    /// ...
    /// 29 | ...         range: <22, 25>,
    ///    |                     ^^
    ///    |
    /// ```
    pub const fn margin(mut self, margin: Option<Margin>) -> Self {
        self.margin = margin;
        self
    }

    /// Set the output style for `error`
    pub const fn error(mut self, style: Style) -> Self {
        self.stylesheet.error = style;
        self
    }

    /// Set the output style for `warning`
    pub const fn warning(mut self, style: Style) -> Self {
        self.stylesheet.warning = style;
        self
    }

    /// Set the output style for `info`
    pub const fn info(mut self, style: Style) -> Self {
        self.stylesheet.info = style;
        self
    }

    /// Set the output style for `note`
    pub const fn note(mut self, style: Style) -> Self {
        self.stylesheet.note = style;
        self
    }

    /// Set the output style for `help`
    pub const fn help(mut self, style: Style) -> Self {
        self.stylesheet.help = style;
        self
    }

    /// Set the output style for line numbers
    pub const fn line_no(mut self, style: Style) -> Self {
        self.stylesheet.line_no = style;
        self
    }

    /// Set the output style for emphasis
    pub const fn emphasis(mut self, style: Style) -> Self {
        self.stylesheet.emphasis = style;
        self
    }

    /// Set the output style for none
    pub const fn none(mut self, style: Style) -> Self {
        self.stylesheet.none = style;
        self
    }

    /// Render a snippet into a `Display`able object
    pub fn render<'a>(&'a self, snippet: Snippet<'a>) -> impl Display + 'a {
        DisplayList::new(
            snippet,
            &self.stylesheet,
            self.anonymized_line_numbers,
            self.margin,
        )
    }
}
