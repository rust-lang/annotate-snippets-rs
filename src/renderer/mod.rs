// Most of this file is adapted from https://github.com/rust-lang/rust/blob/160905b6253f42967ed4aef4b98002944c7df24c/compiler/rustc_errors/src/emitter.rs

//! The renderer for [`Message`]s
//!
//! # Example
//! ```
//! use annotate_snippets::*;
//!
//! let source = r#"
//! use baz::zed::bar;
//!
//! mod baz {}
//! mod zed {
//!     pub fn bar() { println!("bar3"); }
//! }
//! fn main() {
//!     bar();
//! }
//! "#;
//! Level::Error
//!     .message("unresolved import `baz::zed`")
//!     .id("E0432")
//!     .group(
//!         Group::new().element(
//!             Snippet::source(source)
//!                 .origin("temp.rs")
//!                 .line_start(1)
//!                 .fold(true)
//!                 .annotation(
//!                     AnnotationKind::Primary
//!                         .span(10..13)
//!                          .label("could not find `zed` in `baz`"),
//!                 )
//!         )
//!     );
//! ```

mod margin;
mod source_map;
mod styled_buffer;
pub(crate) mod stylesheet;

use crate::renderer::source_map::{AnnotatedLineInfo, Loc, SourceMap};
use crate::renderer::styled_buffer::StyledBuffer;
use crate::{Annotation, AnnotationKind, Element, Group, Level, Message, Origin, Snippet, Title};
pub use anstyle::*;
use margin::Margin;
use std::borrow::Cow;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{HashMap, VecDeque};
use stylesheet::Stylesheet;

const ANONYMIZED_LINE_NUM: &str = "LL";
pub const DEFAULT_TERM_WIDTH: usize = 140;

/// A renderer for [`Message`]s
#[derive(Clone, Debug)]
pub struct Renderer {
    anonymized_line_numbers: bool,
    term_width: usize,
    stylesheet: Stylesheet,
}

impl Renderer {
    /// No terminal styling
    pub const fn plain() -> Self {
        Self {
            anonymized_line_numbers: false,
            term_width: DEFAULT_TERM_WIDTH,
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
                context: BRIGHT_BLUE.effects(Effects::BOLD),
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

    // Set the terminal width
    pub const fn term_width(mut self, term_width: usize) -> Self {
        self.term_width = term_width;
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
}

impl Renderer {
    pub fn render(&self, mut message: Message<'_>) -> String {
        let mut buffer = StyledBuffer::new();
        let max_line_num_len = if self.anonymized_line_numbers {
            ANONYMIZED_LINE_NUM.len()
        } else {
            let n = message.max_line_number();
            num_decimal_digits(n)
        };
        let title = message.groups.remove(0).elements.remove(0);
        let level = if let Element::Title(title) = &title {
            title.level
        } else {
            panic!("Expected a title as the first element of the message")
        };
        if let Some(first) = message.groups.first_mut() {
            first.elements.insert(0, title);
        } else {
            message.groups.push(Group::new().element(title));
        }
        self.render_message(&mut buffer, message, max_line_num_len);

        buffer.render(level, &self.stylesheet).unwrap()
    }

    fn render_message(
        &self,
        buffer: &mut StyledBuffer,
        message: Message<'_>,
        max_line_num_len: usize,
    ) {
        let group_len = message.groups.len();
        for (g, group) in message.groups.into_iter().enumerate() {
            let primary_origin = group
                .elements
                .iter()
                .find_map(|s| match &s {
                    Element::Cause(cause) => {
                        if cause.markers.iter().any(|m| m.kind.is_primary()) {
                            Some(cause.origin)
                        } else {
                            None
                        }
                    }
                    Element::Origin(origin) => {
                        if origin.primary {
                            Some(Some(origin.origin))
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .unwrap_or(
                    group
                        .elements
                        .iter()
                        .find_map(|s| match &s {
                            Element::Cause(cause) => Some(cause.origin),
                            Element::Origin(origin) => Some(Some(origin.origin)),
                            _ => None,
                        })
                        .unwrap_or_default(),
                );
            let mut source_map_annotated_lines = VecDeque::new();
            let mut max_depth = 0;
            for e in &group.elements {
                if let Element::Cause(cause) = e {
                    let source_map = SourceMap::new(cause.source, cause.line_start);
                    let (depth, annotated_lines) =
                        source_map.annotated_lines(cause.markers.clone(), cause.fold);
                    max_depth = max(max_depth, depth);
                    source_map_annotated_lines.push_back((source_map, annotated_lines));
                }
            }
            let mut message_iter = group.elements.iter().enumerate().peekable();
            while let Some((i, section)) = message_iter.next() {
                let peek = message_iter.peek().map(|(_, s)| s).copied();
                match &section {
                    Element::Title(title) => {
                        self.render_title(
                            buffer,
                            title,
                            peek,
                            max_line_num_len,
                            if i == 0 { false } else { !title.primary },
                            message.id.as_ref().and_then(|id| {
                                if g == 0 && i == 0 {
                                    Some(id)
                                } else {
                                    None
                                }
                            }),
                        );
                    }
                    Element::Cause(cause) => {
                        if let Some((source_map, annotated_lines)) =
                            source_map_annotated_lines.pop_front()
                        {
                            self.render_snippet_annotations(
                                buffer,
                                max_line_num_len,
                                cause,
                                primary_origin,
                                &source_map,
                                &annotated_lines,
                                max_depth,
                            );

                            if g == 0 && group_len > 1 {
                                if matches!(peek, Some(Element::Title(level)) if level.level != Level::None)
                                {
                                    self.draw_col_separator_no_space(
                                        buffer,
                                        buffer.num_lines(),
                                        max_line_num_len + 1,
                                    );
                                // We want to draw the separator when it is
                                // requested, or when it is the last element
                                } else if peek.is_none() {
                                    self.draw_col_separator_no_space_with_style(
                                        buffer,
                                        '|',
                                        buffer.num_lines(),
                                        max_line_num_len + 1,
                                        ElementStyle::LineNumber,
                                    );
                                }
                            }
                        }
                    }
                    Element::Origin(origin) => {
                        self.render_origin(buffer, max_line_num_len, origin);
                    }
                    Element::ColumnSeparator(_) => {
                        self.draw_col_separator_no_space(
                            buffer,
                            buffer.num_lines(),
                            max_line_num_len + 1,
                        );
                    }
                }
                if g == 0
                    && (matches!(section, Element::Origin(_))
                        || (matches!(section, Element::Title(_)) && i == 0)
                        || matches!(section, Element::Title(level) if level.level == Level::None))
                {
                    if peek.is_none() && group_len > 1 {
                        self.draw_col_separator_no_space_with_style(
                            buffer,
                            '|',
                            buffer.num_lines(),
                            max_line_num_len + 1,
                            ElementStyle::LineNumber,
                        );
                    } else if matches!(peek, Some(Element::Title(level)) if level.level != Level::None)
                    {
                        self.draw_col_separator_no_space(
                            buffer,
                            buffer.num_lines(),
                            max_line_num_len + 1,
                        );
                    }
                }
            }
        }
    }

    fn render_title(
        &self,
        buffer: &mut StyledBuffer,
        title: &Title<'_>,
        next_section: Option<&Element<'_>>,
        max_line_num_len: usize,
        is_secondary: bool,
        id: Option<&&str>,
    ) {
        let line_offset = buffer.num_lines();

        let (has_primary_spans, has_span_labels) =
            next_section.map_or((false, false), |s| match s {
                Element::Title(_) | Element::ColumnSeparator(_) => (false, false),
                Element::Cause(cause) => (
                    cause.markers.iter().any(|m| m.kind.is_primary()),
                    cause.markers.iter().any(|m| m.label.is_some()),
                ),
                Element::Origin(_) => (false, true),
            });

        if !has_primary_spans && !has_span_labels && is_secondary {
            // This is a secondary message with no span info
            for _ in 0..max_line_num_len {
                buffer.prepend(line_offset, " ", ElementStyle::NoStyle);
            }
            if title.level != Level::None {
                buffer.puts(
                    line_offset,
                    max_line_num_len + 1,
                    "= ",
                    ElementStyle::LineNumber,
                );
                buffer.append(
                    line_offset,
                    title.level.as_str(),
                    ElementStyle::MainHeaderMsg,
                );
                buffer.append(line_offset, ": ", ElementStyle::NoStyle);
            }
            self.msgs_to_buffer(buffer, title.title, max_line_num_len, "note", None);
        } else {
            let mut label_width = 0;

            if title.level != Level::None {
                buffer.append(
                    line_offset,
                    title.level.as_str(),
                    ElementStyle::Level(title.level),
                );
            }
            label_width += title.level.as_str().len();
            if let Some(id) = id {
                buffer.append(line_offset, "[", ElementStyle::Level(title.level));
                buffer.append(line_offset, id, ElementStyle::Level(title.level));
                buffer.append(line_offset, "]", ElementStyle::Level(title.level));
                label_width += 2 + id.len();
            }
            let header_style = if is_secondary {
                ElementStyle::HeaderMsg
            } else {
                ElementStyle::MainHeaderMsg
            };
            if title.level != Level::None {
                buffer.append(line_offset, ": ", header_style);
                label_width += 2;
            }
            if !title.title.is_empty() {
                for (line, text) in normalize_whitespace(title.title).lines().enumerate() {
                    buffer.append(
                        line_offset + line,
                        &format!(
                            "{}{}",
                            if line == 0 {
                                String::new()
                            } else {
                                " ".repeat(label_width)
                            },
                            text
                        ),
                        header_style,
                    );
                }
            }
        }
    }

    /// Adds a left margin to every line but the first, given a padding length and the label being
    /// displayed, keeping the provided highlighting.
    fn msgs_to_buffer(
        &self,
        buffer: &mut StyledBuffer,
        title: &str,
        padding: usize,
        label: &str,
        override_style: Option<ElementStyle>,
    ) -> usize {
        // The extra 5 ` ` is padding that's always needed to align to the `note: `:
        //
        //   error: message
        //     --> file.rs:13:20
        //      |
        //   13 |     <CODE>
        //      |      ^^^^
        //      |
        //      = note: multiline
        //              message
        //   ++^^^----xx
        //    |  |   | |
        //    |  |   | magic `2`
        //    |  |   length of label
        //    |  magic `3`
        //    `max_line_num_len`
        let padding = " ".repeat(padding + label.len() + 5);

        let mut line_number = buffer.num_lines().saturating_sub(1);

        // Provided the following diagnostic message:
        //
        //     let msgs = vec![
        //       ("
        //       ("highlighted multiline\nstring to\nsee how it ", Style::NoStyle),
        //       ("looks", Style::Highlight),
        //       ("with\nvery ", Style::NoStyle),
        //       ("weird", Style::Highlight),
        //       (" formats\n", Style::NoStyle),
        //       ("see?", Style::Highlight),
        //     ];
        //
        // the expected output on a note is (* surround the highlighted text)
        //
        //        = note: highlighted multiline
        //                string to
        //                see how it *looks* with
        //                very *weird* formats
        //                see?
        let style = if let Some(override_style) = override_style {
            override_style
        } else {
            ElementStyle::NoStyle
        };
        let text = &normalize_whitespace(title);
        let lines = text.split('\n').collect::<Vec<_>>();
        if lines.len() > 1 {
            for (i, line) in lines.iter().enumerate() {
                if i != 0 {
                    line_number += 1;
                    buffer.append(line_number, &padding, ElementStyle::NoStyle);
                }
                buffer.append(line_number, line, style);
            }
        } else {
            buffer.append(line_number, text, style);
        }
        line_number
    }

    fn render_origin(
        &self,
        buffer: &mut StyledBuffer,
        max_line_num_len: usize,
        origin: &Origin<'_>,
    ) {
        let buffer_msg_line_offset = buffer.num_lines();
        if origin.primary {
            buffer.prepend(
                buffer_msg_line_offset,
                self.file_start(),
                ElementStyle::LineNumber,
            );
        } else {
            // if !origin.standalone {
            //     // Add spacing line, as shown:
            //     //   --> $DIR/file:54:15
            //     //    |
            //     // LL |         code
            //     //    |         ^^^^
            //     //    | (<- It prints *this* line)
            //     //   ::: $DIR/other_file.rs:15:5
            //     //    |
            //     // LL |     code
            //     //    |     ----
            //     self.draw_col_separator_no_space(
            //         buffer,
            //         buffer_msg_line_offset,
            //         max_line_num_len + 1,
            //     );
            //
            //     buffer_msg_line_offset += 1;
            // }
            // Then, the secondary file indicator
            buffer.prepend(
                buffer_msg_line_offset,
                self.secondary_file_start(),
                ElementStyle::LineNumber,
            );
        }

        let str = match (&origin.line, &origin.char_column) {
            (Some(line), Some(col)) => {
                format!("{}:{}:{}", origin.origin, line, col)
            }
            (Some(line), None) => format!("{}:{}", origin.origin, line),
            _ => origin.origin.to_owned(),
        };
        buffer.append(buffer_msg_line_offset, &str, ElementStyle::LineAndColumn);
        for _ in 0..max_line_num_len {
            buffer.prepend(buffer_msg_line_offset, " ", ElementStyle::NoStyle);
        }

        if let Some(label) = &origin.label {
            self.draw_col_separator_no_space(
                buffer,
                buffer_msg_line_offset + 1,
                max_line_num_len + 1,
            );
            let title = Level::Note.title(label);
            self.render_title(buffer, &title, None, max_line_num_len, true, None);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_snippet_annotations(
        &self,
        buffer: &mut StyledBuffer,
        max_line_num_len: usize,
        snippet: &Snippet<'_, Annotation<'_>>,
        primary_origin: Option<&str>,
        sm: &SourceMap<'_>,
        annotated_lines: &[AnnotatedLineInfo<'_>],
        multiline_depth: usize,
    ) {
        if let Some(origin) = snippet.origin {
            let mut origin = Origin::new(origin);
            // print out the span location and spacer before we print the annotated source
            // to do this, we need to know if this span will be primary
            let is_primary = primary_origin == Some(origin.origin);

            if is_primary {
                origin.primary = true;
                if let Some(primary_line) = annotated_lines
                    .iter()
                    .find(|l| l.annotations.iter().any(LineAnnotation::is_primary))
                    .or(annotated_lines.iter().find(|l| !l.annotations.is_empty()))
                {
                    origin.line = Some(primary_line.line_index);
                    if let Some(first_annotation) = primary_line
                        .annotations
                        .iter()
                        .find(|a| a.is_primary())
                        .or(primary_line.annotations.first())
                    {
                        origin.char_column = Some(first_annotation.start.char + 1);
                    }
                }
            } else {
                let buffer_msg_line_offset = buffer.num_lines();
                // Add spacing line, as shown:
                //   --> $DIR/file:54:15
                //    |
                // LL |         code
                //    |         ^^^^
                //    | (<- It prints *this* line)
                //   ::: $DIR/other_file.rs:15:5
                //    |
                // LL |     code
                //    |     ----
                self.draw_col_separator_no_space(
                    buffer,
                    buffer_msg_line_offset,
                    max_line_num_len + 1,
                );
                if let Some(first_line) = annotated_lines.first() {
                    origin.line = Some(first_line.line_index);
                    if let Some(first_annotation) = first_line.annotations.first() {
                        origin.char_column = Some(first_annotation.start.char + 1);
                    }
                }
            }
            self.render_origin(buffer, max_line_num_len, &origin);
        }

        // Put in the spacer between the location and annotated source
        let buffer_msg_line_offset = buffer.num_lines();
        self.draw_col_separator_no_space(buffer, buffer_msg_line_offset, max_line_num_len + 1);

        // Contains the vertical lines' positions for active multiline annotations
        let mut multilines = Vec::new();

        // Get the left-side margin to remove it
        let mut whitespace_margin = usize::MAX;
        for line_info in annotated_lines {
            // Whitespace can only be removed (aka considered leading)
            // if the lexer considers it whitespace.
            // non-rustc_lexer::is_whitespace() chars are reported as an
            // error (ex. no-break-spaces \u{a0}), and thus can't be considered
            // for removal during error reporting.
            let leading_whitespace = line_info
                .line
                .chars()
                .take_while(|c| c.is_whitespace())
                .map(|c| {
                    match c {
                        // Tabs are displayed as 4 spaces
                        '\t' => 4,
                        _ => 1,
                    }
                })
                .sum();
            if line_info.line.chars().any(|c| !c.is_whitespace()) {
                whitespace_margin = min(whitespace_margin, leading_whitespace);
            }
        }
        if whitespace_margin == usize::MAX {
            whitespace_margin = 0;
        }

        // Left-most column any visible span points at.
        let mut span_left_margin = usize::MAX;
        for line_info in annotated_lines {
            for ann in &line_info.annotations {
                span_left_margin = min(span_left_margin, ann.start.display);
                span_left_margin = min(span_left_margin, ann.end.display);
            }
        }
        if span_left_margin == usize::MAX {
            span_left_margin = 0;
        }

        // Right-most column any visible span points at.
        let mut span_right_margin = 0;
        let mut label_right_margin = 0;
        let mut max_line_len = 0;
        for line_info in annotated_lines {
            max_line_len = max(max_line_len, line_info.line.len());
            for ann in &line_info.annotations {
                span_right_margin = max(span_right_margin, ann.start.display);
                span_right_margin = max(span_right_margin, ann.end.display);
                // FIXME: account for labels not in the same line
                let label_right = ann.label.as_ref().map_or(0, |l| l.len() + 1);
                label_right_margin = max(label_right_margin, ann.end.display + label_right);
            }
        }
        let width_offset = 3 + max_line_num_len;
        let code_offset = if multiline_depth == 0 {
            width_offset
        } else {
            width_offset + multiline_depth + 1
        };

        let column_width = self.term_width.saturating_sub(code_offset);

        let margin = Margin::new(
            whitespace_margin,
            span_left_margin,
            span_right_margin,
            label_right_margin,
            column_width,
            max_line_len,
        );

        // Next, output the annotate source for this file
        for annotated_line_idx in 0..annotated_lines.len() {
            let previous_buffer_line = buffer.num_lines();

            let depths = self.render_source_line(
                &annotated_lines[annotated_line_idx],
                buffer,
                width_offset,
                code_offset,
                max_line_num_len,
                margin,
            );

            let mut to_add = HashMap::new();

            for (depth, style) in depths {
                if let Some(index) = multilines.iter().position(|(d, _)| d == &depth) {
                    multilines.swap_remove(index);
                } else {
                    to_add.insert(depth, style);
                }
            }

            // Set the multiline annotation vertical lines to the left of
            // the code in this line.
            for (depth, style) in &multilines {
                for line in previous_buffer_line..buffer.num_lines() {
                    self.draw_multiline_line(buffer, line, width_offset, *depth, *style);
                }
            }
            // check to see if we need to print out or elide lines that come between
            // this annotated line and the next one.
            if annotated_line_idx < (annotated_lines.len() - 1) {
                let line_idx_delta = annotated_lines[annotated_line_idx + 1].line_index
                    - annotated_lines[annotated_line_idx].line_index;
                match line_idx_delta.cmp(&2) {
                    Ordering::Greater => {
                        let last_buffer_line_num = buffer.num_lines();
                        buffer.puts(last_buffer_line_num, 0, "...", ElementStyle::LineNumber);

                        // Set the multiline annotation vertical lines on `...` bridging line.
                        for (depth, style) in &multilines {
                            self.draw_multiline_line(
                                buffer,
                                last_buffer_line_num,
                                width_offset,
                                *depth,
                                *style,
                            );
                        }
                        if let Some(line) = annotated_lines.get(annotated_line_idx) {
                            for ann in &line.annotations {
                                if let LineAnnotationType::MultilineStart(pos) = ann.annotation_type
                                {
                                    // In the case where we have elided the entire start of the
                                    // multispan because those lines were empty, we still need
                                    // to draw the `|`s across the `...`.
                                    self.draw_multiline_line(
                                        buffer,
                                        last_buffer_line_num,
                                        width_offset,
                                        pos,
                                        if ann.is_primary() {
                                            ElementStyle::UnderlinePrimary
                                        } else {
                                            ElementStyle::UnderlineSecondary
                                        },
                                    );
                                }
                            }
                        }
                    }

                    Ordering::Equal => {
                        let unannotated_line = sm
                            .get_line(annotated_lines[annotated_line_idx].line_index + 1)
                            .unwrap_or("");

                        let last_buffer_line_num = buffer.num_lines();

                        self.draw_line(
                            buffer,
                            &normalize_whitespace(unannotated_line),
                            annotated_lines[annotated_line_idx + 1].line_index - 1,
                            last_buffer_line_num,
                            width_offset,
                            code_offset,
                            max_line_num_len,
                            margin,
                        );

                        for (depth, style) in &multilines {
                            self.draw_multiline_line(
                                buffer,
                                last_buffer_line_num,
                                width_offset,
                                *depth,
                                *style,
                            );
                        }
                        if let Some(line) = annotated_lines.get(annotated_line_idx) {
                            for ann in &line.annotations {
                                if let LineAnnotationType::MultilineStart(pos) = ann.annotation_type
                                {
                                    self.draw_multiline_line(
                                        buffer,
                                        last_buffer_line_num,
                                        width_offset,
                                        pos,
                                        if ann.is_primary() {
                                            ElementStyle::UnderlinePrimary
                                        } else {
                                            ElementStyle::UnderlineSecondary
                                        },
                                    );
                                }
                            }
                        }
                    }
                    Ordering::Less => {}
                }
            }

            multilines.extend(to_add);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_source_line(
        &self,
        line_info: &AnnotatedLineInfo<'_>,
        buffer: &mut StyledBuffer,
        width_offset: usize,
        code_offset: usize,
        max_line_num_len: usize,
        margin: Margin,
    ) -> Vec<(usize, ElementStyle)> {
        // Draw:
        //
        //   LL | ... code ...
        //      |     ^^-^ span label
        //      |       |
        //      |       secondary span label
        //
        //   ^^ ^ ^^^ ^^^^ ^^^ we don't care about code too far to the right of a span, we trim it
        //   |  | |   |
        //   |  | |   actual code found in your source code and the spans we use to mark it
        //   |  | when there's too much wasted space to the left, trim it
        //   |  vertical divider between the column number and the code
        //   column number

        if line_info.line_index == 0 {
            return Vec::new();
        }

        let source_string = normalize_whitespace(line_info.line);

        let line_offset = buffer.num_lines();

        // Left trim
        let left = margin.left(source_string.len());

        // FIXME: This looks fishy. See #132860.
        // Account for unicode characters of width !=0 that were removed.
        let left = source_string.chars().take(left).map(char_width).sum();

        self.draw_line(
            buffer,
            &source_string,
            line_info.line_index,
            line_offset,
            width_offset,
            code_offset,
            max_line_num_len,
            margin,
        );

        // Special case when there's only one annotation involved, it is the start of a multiline
        // span and there's no text at the beginning of the code line. Instead of doing the whole
        // graph:
        //
        // 2 |   fn foo() {
        //   |  _^
        // 3 | |
        // 4 | | }
        //   | |_^ test
        //
        // we simplify the output to:
        //
        // 2 | / fn foo() {
        // 3 | |
        // 4 | | }
        //   | |_^ test
        let mut buffer_ops = vec![];
        let mut annotations = vec![];
        let mut short_start = true;
        for ann in &line_info.annotations {
            if let LineAnnotationType::MultilineStart(depth) = ann.annotation_type {
                if source_string
                    .chars()
                    .take(ann.start.display)
                    .all(char::is_whitespace)
                {
                    let uline = self.underline(ann.is_primary());
                    let chr = uline.multiline_whole_line;
                    annotations.push((depth, uline.style));
                    buffer_ops.push((line_offset, width_offset + depth - 1, chr, uline.style));
                } else {
                    short_start = false;
                    break;
                }
            } else if let LineAnnotationType::MultilineLine(_) = ann.annotation_type {
            } else {
                short_start = false;
                break;
            }
        }
        if short_start {
            for (y, x, c, s) in buffer_ops {
                buffer.putc(y, x, c, s);
            }
            return annotations;
        }

        // We want to display like this:
        //
        //      vec.push(vec.pop().unwrap());
        //      ---      ^^^               - previous borrow ends here
        //      |        |
        //      |        error occurs here
        //      previous borrow of `vec` occurs here
        //
        // But there are some weird edge cases to be aware of:
        //
        //      vec.push(vec.pop().unwrap());
        //      --------                    - previous borrow ends here
        //      ||
        //      |this makes no sense
        //      previous borrow of `vec` occurs here
        //
        // For this reason, we group the lines into "highlight lines"
        // and "annotations lines", where the highlight lines have the `^`.

        // Sort the annotations by (start, end col)
        // The labels are reversed, sort and then reversed again.
        // Consider a list of annotations (A1, A2, C1, C2, B1, B2) where
        // the letter signifies the span. Here we are only sorting by the
        // span and hence, the order of the elements with the same span will
        // not change. On reversing the ordering (|a, b| but b.cmp(a)), you get
        // (C1, C2, B1, B2, A1, A2). All the elements with the same span are
        // still ordered first to last, but all the elements with different
        // spans are ordered by their spans in last to first order. Last to
        // first order is important, because the jiggly lines and | are on
        // the left, so the rightmost span needs to be rendered first,
        // otherwise the lines would end up needing to go over a message.

        let mut annotations = line_info.annotations.clone();
        annotations.sort_by_key(|a| Reverse(a.start.display));

        // First, figure out where each label will be positioned.
        //
        // In the case where you have the following annotations:
        //
        //      vec.push(vec.pop().unwrap());
        //      --------                    - previous borrow ends here [C]
        //      ||
        //      |this makes no sense [B]
        //      previous borrow of `vec` occurs here [A]
        //
        // `annotations_position` will hold [(2, A), (1, B), (0, C)].
        //
        // We try, when possible, to stick the rightmost annotation at the end
        // of the highlight line:
        //
        //      vec.push(vec.pop().unwrap());
        //      ---      ---               - previous borrow ends here
        //
        // But sometimes that's not possible because one of the other
        // annotations overlaps it. For example, from the test
        // `span_overlap_label`, we have the following annotations
        // (written on distinct lines for clarity):
        //
        //      fn foo(x: u32) {
        //      --------------
        //             -
        //
        // In this case, we can't stick the rightmost-most label on
        // the highlight line, or we would get:
        //
        //      fn foo(x: u32) {
        //      -------- x_span
        //      |
        //      fn_span
        //
        // which is totally weird. Instead we want:
        //
        //      fn foo(x: u32) {
        //      --------------
        //      |      |
        //      |      x_span
        //      fn_span
        //
        // which is...less weird, at least. In fact, in general, if
        // the rightmost span overlaps with any other span, we should
        // use the "hang below" version, so we can at least make it
        // clear where the span *starts*. There's an exception for this
        // logic, when the labels do not have a message:
        //
        //      fn foo(x: u32) {
        //      --------------
        //             |
        //             x_span
        //
        // instead of:
        //
        //      fn foo(x: u32) {
        //      --------------
        //      |      |
        //      |      x_span
        //      <EMPTY LINE>
        //
        let mut annotations_position = vec![];
        let mut line_len: usize = 0;
        let mut p = 0;
        for (i, annotation) in annotations.iter().enumerate() {
            for (j, next) in annotations.iter().enumerate() {
                if overlaps(next, annotation, 0)  // This label overlaps with another one and both
                    && annotation.has_label()     // take space (they have text and are not
                    && j > i                      // multiline lines).
                    && p == 0
                // We're currently on the first line, move the label one line down
                {
                    // If we're overlapping with an un-labelled annotation with the same span
                    // we can just merge them in the output
                    if next.start.display == annotation.start.display
                        && next.end.display == annotation.end.display
                        && !next.has_label()
                    {
                        continue;
                    }

                    // This annotation needs a new line in the output.
                    p += 1;
                    break;
                }
            }
            annotations_position.push((p, annotation));
            for (j, next) in annotations.iter().enumerate() {
                if j > i {
                    let l = next.label.as_ref().map_or(0, |label| label.len() + 2);
                    if (overlaps(next, annotation, l) // Do not allow two labels to be in the same
                        // line if they overlap including padding, to
                        // avoid situations like:
                        //
                        //      fn foo(x: u32) {
                        //      -------^------
                        //      |      |
                        //      fn_spanx_span
                        //
                        && annotation.has_label()    // Both labels must have some text, otherwise
                        && next.has_label())         // they are not overlapping.
                        // Do not add a new line if this annotation
                        // or the next are vertical line placeholders.
                        || (annotation.takes_space() // If either this or the next annotation is
                        && next.has_label())     // multiline start/end, move it to a new line
                        || (annotation.has_label()   // so as not to overlap the horizontal lines.
                        && next.takes_space())
                        || (annotation.takes_space() && next.takes_space())
                        || (overlaps(next, annotation, l)
                        && next.end.display <= annotation.end.display
                        && next.has_label()
                        && p == 0)
                    // Avoid #42595.
                    {
                        // This annotation needs a new line in the output.
                        p += 1;
                        break;
                    }
                }
            }
            line_len = max(line_len, p);
        }

        if line_len != 0 {
            line_len += 1;
        }

        // If there are no annotations or the only annotations on this line are
        // MultilineLine, then there's only code being shown, stop processing.
        if line_info.annotations.iter().all(LineAnnotation::is_line) {
            return vec![];
        }

        if annotations_position
            .iter()
            .all(|(_, ann)| matches!(ann.annotation_type, LineAnnotationType::MultilineStart(_)))
        {
            if let Some(max_pos) = annotations_position.iter().map(|(pos, _)| *pos).max() {
                // Special case the following, so that we minimize overlapping multiline spans.
                //
                // 3 │       X0 Y0 Z0
                //   │ ┏━━━━━┛  │  │     < We are writing these lines
                //   │ ┃┌───────┘  │     < by reverting the "depth" of
                //   │ ┃│┌─────────┘     < their multiline spans.
                // 4 │ ┃││   X1 Y1 Z1
                // 5 │ ┃││   X2 Y2 Z2
                //   │ ┃│└────╿──│──┘ `Z` label
                //   │ ┃└─────│──┤
                //   │ ┗━━━━━━┥  `Y` is a good letter too
                //   ╰╴       `X` is a good letter
                for (pos, _) in &mut annotations_position {
                    *pos = max_pos - *pos;
                }
                // We know then that we don't need an additional line for the span label, saving us
                // one line of vertical space.
                line_len = line_len.saturating_sub(1);
            }
        }

        // Write the column separator.
        //
        // After this we will have:
        //
        // 2 |   fn foo() {
        //   |
        //   |
        //   |
        // 3 |
        // 4 |   }
        //   |
        for pos in 0..=line_len {
            self.draw_col_separator_no_space(buffer, line_offset + pos + 1, width_offset - 2);
        }

        // Write the horizontal lines for multiline annotations
        // (only the first and last lines need this).
        //
        // After this we will have:
        //
        // 2 |   fn foo() {
        //   |  __________
        //   |
        //   |
        // 3 |
        // 4 |   }
        //   |  _
        for &(pos, annotation) in &annotations_position {
            let underline = self.underline(annotation.is_primary());
            let pos = pos + 1;
            match annotation.annotation_type {
                LineAnnotationType::MultilineStart(depth)
                | LineAnnotationType::MultilineEnd(depth) => {
                    self.draw_range(
                        buffer,
                        underline.multiline_horizontal,
                        line_offset + pos,
                        width_offset + depth,
                        (code_offset + annotation.start.display).saturating_sub(left),
                        underline.style,
                    );
                }
                _ => {}
            }
        }

        // Write the vertical lines for labels that are on a different line as the underline.
        //
        // After this we will have:
        //
        // 2 |   fn foo() {
        //   |  __________
        //   | |    |
        //   | |
        // 3 | |
        // 4 | | }
        //   | |_
        for &(pos, annotation) in &annotations_position {
            let underline = self.underline(annotation.is_primary());
            let pos = pos + 1;

            if pos > 1 && (annotation.has_label() || annotation.takes_space()) {
                for p in line_offset + 1..=line_offset + pos {
                    buffer.putc(
                        p,
                        (code_offset + annotation.start.display).saturating_sub(left),
                        match annotation.annotation_type {
                            LineAnnotationType::MultilineLine(_) => underline.multiline_vertical,
                            _ => underline.vertical_text_line,
                        },
                        underline.style,
                    );
                }
                if let LineAnnotationType::MultilineStart(_) = annotation.annotation_type {
                    buffer.putc(
                        line_offset + pos,
                        (code_offset + annotation.start.display).saturating_sub(left),
                        underline.bottom_right,
                        underline.style,
                    );
                }
                if matches!(
                    annotation.annotation_type,
                    LineAnnotationType::MultilineEnd(_)
                ) && annotation.has_label()
                {
                    buffer.putc(
                        line_offset + pos,
                        (code_offset + annotation.start.display).saturating_sub(left),
                        underline.multiline_bottom_right_with_text,
                        underline.style,
                    );
                }
            }
            match annotation.annotation_type {
                LineAnnotationType::MultilineStart(depth) => {
                    buffer.putc(
                        line_offset + pos,
                        width_offset + depth - 1,
                        underline.top_left,
                        underline.style,
                    );
                    for p in line_offset + pos + 1..line_offset + line_len + 2 {
                        buffer.putc(
                            p,
                            width_offset + depth - 1,
                            underline.multiline_vertical,
                            underline.style,
                        );
                    }
                }
                LineAnnotationType::MultilineEnd(depth) => {
                    for p in line_offset..line_offset + pos {
                        buffer.putc(
                            p,
                            width_offset + depth - 1,
                            underline.multiline_vertical,
                            underline.style,
                        );
                    }
                    buffer.putc(
                        line_offset + pos,
                        width_offset + depth - 1,
                        underline.bottom_left,
                        underline.style,
                    );
                }
                _ => (),
            }
        }

        // Write the labels on the annotations that actually have a label.
        //
        // After this we will have:
        //
        // 2 |   fn foo() {
        //   |  __________
        //   |      |
        //   |      something about `foo`
        // 3 |
        // 4 |   }
        //   |  _  test
        for &(pos, annotation) in &annotations_position {
            let style = if annotation.is_primary() {
                ElementStyle::LabelPrimary
            } else {
                ElementStyle::LabelSecondary
            };
            let (pos, col) = if pos == 0 {
                if annotation.end.display == 0 {
                    (pos + 1, (annotation.end.display + 2).saturating_sub(left))
                } else {
                    (pos + 1, (annotation.end.display + 1).saturating_sub(left))
                }
            } else {
                (pos + 2, annotation.start.display.saturating_sub(left))
            };
            if let Some(label) = annotation.label {
                buffer.puts(line_offset + pos, code_offset + col, label, style);
            }
        }

        // Sort from biggest span to smallest span so that smaller spans are
        // represented in the output:
        //
        // x | fn foo()
        //   | ^^^---^^
        //   | |  |
        //   | |  something about `foo`
        //   | something about `fn foo()`
        annotations_position.sort_by_key(|(_, ann)| {
            // Decreasing order. When annotations share the same length, prefer `Primary`.
            (Reverse(ann.len()), ann.is_primary())
        });

        // Write the underlines.
        //
        // After this we will have:
        //
        // 2 |   fn foo() {
        //   |  ____-_____^
        //   |      |
        //   |      something about `foo`
        // 3 |
        // 4 |   }
        //   |  _^  test
        for &(pos, annotation) in &annotations_position {
            let uline = self.underline(annotation.is_primary());
            for p in annotation.start.display..annotation.end.display {
                // The default span label underline.
                buffer.putc(
                    line_offset + 1,
                    (code_offset + p).saturating_sub(left),
                    uline.underline,
                    uline.style,
                );
            }

            if pos == 0
                && matches!(
                    annotation.annotation_type,
                    LineAnnotationType::MultilineStart(_) | LineAnnotationType::MultilineEnd(_)
                )
            {
                // The beginning of a multiline span with its leftward moving line on the same line.
                buffer.putc(
                    line_offset + 1,
                    (code_offset + annotation.start.display).saturating_sub(left),
                    match annotation.annotation_type {
                        LineAnnotationType::MultilineStart(_) => uline.top_right_flat,
                        LineAnnotationType::MultilineEnd(_) => uline.multiline_end_same_line,
                        _ => panic!("unexpected annotation type: {annotation:?}"),
                    },
                    uline.style,
                );
            } else if pos != 0
                && matches!(
                    annotation.annotation_type,
                    LineAnnotationType::MultilineStart(_) | LineAnnotationType::MultilineEnd(_)
                )
            {
                // The beginning of a multiline span with its leftward moving line on another line,
                // so we start going down first.
                buffer.putc(
                    line_offset + 1,
                    (code_offset + annotation.start.display).saturating_sub(left),
                    match annotation.annotation_type {
                        LineAnnotationType::MultilineStart(_) => uline.multiline_start_down,
                        LineAnnotationType::MultilineEnd(_) => uline.multiline_end_up,
                        _ => panic!("unexpected annotation type: {annotation:?}"),
                    },
                    uline.style,
                );
            } else if pos != 0 && annotation.has_label() {
                // The beginning of a span label with an actual label, we'll point down.
                buffer.putc(
                    line_offset + 1,
                    (code_offset + annotation.start.display).saturating_sub(left),
                    uline.label_start,
                    uline.style,
                );
            }
        }
        annotations_position
            .iter()
            .filter_map(|&(_, annotation)| match annotation.annotation_type {
                LineAnnotationType::MultilineStart(p) | LineAnnotationType::MultilineEnd(p) => {
                    let style = if annotation.is_primary() {
                        ElementStyle::LabelPrimary
                    } else {
                        ElementStyle::LabelSecondary
                    };
                    Some((p, style))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_line(
        &self,
        buffer: &mut StyledBuffer,
        source_string: &str,
        line_index: usize,
        line_offset: usize,
        width_offset: usize,
        code_offset: usize,
        max_line_num_len: usize,
        margin: Margin,
    ) {
        // Tabs are assumed to have been replaced by spaces in calling code.
        debug_assert!(!source_string.contains('\t'));
        let line_len = source_string.len();
        // Create the source line we will highlight.
        let left = margin.left(line_len);
        let right = margin.right(line_len);
        // FIXME: The following code looks fishy. See #132860.
        // On long lines, we strip the source line, accounting for unicode.
        let mut taken = 0;
        let code: String = source_string
            .chars()
            .skip(left)
            .take_while(|ch| {
                // Make sure that the trimming on the right will fall within the terminal width.
                let next = char_width(*ch);
                if taken + next > right - left {
                    return false;
                }
                taken += next;
                true
            })
            .collect();

        buffer.puts(line_offset, code_offset, &code, ElementStyle::Quotation);
        if margin.was_cut_left() {
            // We have stripped some code/whitespace from the beginning, make it clear.
            buffer.puts(line_offset, code_offset, "...", ElementStyle::LineNumber);
        }
        if margin.was_cut_right(line_len) {
            // We have stripped some code after the rightmost span end, make it clear we did so.
            buffer.puts(
                line_offset,
                code_offset + taken - 3,
                "...",
                ElementStyle::LineNumber,
            );
        }
        buffer.puts(
            line_offset,
            0,
            &format!("{:>max_line_num_len$}", self.maybe_anonymized(line_index)),
            ElementStyle::LineNumber,
        );

        self.draw_col_separator_no_space(buffer, line_offset, width_offset - 2);
    }

    fn draw_range(
        &self,
        buffer: &mut StyledBuffer,
        symbol: char,
        line: usize,
        col_from: usize,
        col_to: usize,
        style: ElementStyle,
    ) {
        for col in col_from..col_to {
            buffer.putc(line, col, symbol, style);
        }
    }

    fn draw_multiline_line(
        &self,
        buffer: &mut StyledBuffer,
        line: usize,
        offset: usize,
        depth: usize,
        style: ElementStyle,
    ) {
        buffer.putc(line, offset + depth - 1, '|', style);
    }

    fn draw_col_separator_no_space(&self, buffer: &mut StyledBuffer, line: usize, col: usize) {
        self.draw_col_separator_no_space_with_style(
            buffer,
            '|',
            line,
            col,
            ElementStyle::LineNumber,
        );
    }

    fn draw_col_separator_no_space_with_style(
        &self,
        buffer: &mut StyledBuffer,
        chr: char,
        line: usize,
        col: usize,
        style: ElementStyle,
    ) {
        buffer.putc(line, col, chr, style);
    }

    fn maybe_anonymized(&self, line_num: usize) -> Cow<'static, str> {
        if self.anonymized_line_numbers {
            Cow::Borrowed(ANONYMIZED_LINE_NUM)
        } else {
            Cow::Owned(line_num.to_string())
        }
    }

    fn file_start(&self) -> &str {
        "--> "
    }

    fn secondary_file_start(&self) -> &str {
        "::: "
    }

    fn underline(&self, is_primary: bool) -> UnderlineParts {
        if is_primary {
            UnderlineParts {
                style: ElementStyle::UnderlinePrimary,
                underline: '^',
                label_start: '^',
                vertical_text_line: '|',
                multiline_vertical: '|',
                multiline_horizontal: '_',
                multiline_whole_line: '/',
                multiline_start_down: '^',
                bottom_right: '|',
                top_left: ' ',
                top_right_flat: '^',
                bottom_left: '|',
                multiline_end_up: '^',
                multiline_end_same_line: '^',
                multiline_bottom_right_with_text: '|',
            }
        } else {
            UnderlineParts {
                style: ElementStyle::UnderlineSecondary,
                underline: '-',
                label_start: '-',
                vertical_text_line: '|',
                multiline_vertical: '|',
                multiline_horizontal: '_',
                multiline_whole_line: '/',
                multiline_start_down: '-',
                bottom_right: '|',
                top_left: ' ',
                top_right_flat: '-',
                bottom_left: '|',
                multiline_end_up: '-',
                multiline_end_same_line: '-',
                multiline_bottom_right_with_text: '|',
            }
        }
    }
}

// instead of taking the String length or dividing by 10 while > 0, we multiply a limit by 10 until
// we're higher. If the loop isn't exited by the `return`, the last multiplication will wrap, which
// is OK, because while we cannot fit a higher power of 10 in a usize, the loop will end anyway.
// This is also why we need the max number of decimal digits within a `usize`.
fn num_decimal_digits(num: usize) -> usize {
    #[cfg(target_pointer_width = "64")]
    const MAX_DIGITS: usize = 20;

    #[cfg(target_pointer_width = "32")]
    const MAX_DIGITS: usize = 10;

    #[cfg(target_pointer_width = "16")]
    const MAX_DIGITS: usize = 5;

    let mut lim = 10;
    for num_digits in 1..MAX_DIGITS {
        if num < lim {
            return num_digits;
        }
        lim = lim.wrapping_mul(10);
    }
    MAX_DIGITS
}

pub fn str_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}

pub fn char_width(ch: char) -> usize {
    // FIXME: `unicode_width` sometimes disagrees with terminals on how wide a `char` is. For now,
    // just accept that sometimes the code line will be longer than desired.
    match ch {
        '\t' => 4,
        // Keep the following list in sync with `rustc_errors::emitter::OUTPUT_REPLACEMENTS`. These
        // are control points that we replace before printing with a visible codepoint for the sake
        // of being able to point at them with underlines.
        '\u{0000}' | '\u{0001}' | '\u{0002}' | '\u{0003}' | '\u{0004}' | '\u{0005}'
        | '\u{0006}' | '\u{0007}' | '\u{0008}' | '\u{000B}' | '\u{000C}' | '\u{000D}'
        | '\u{000E}' | '\u{000F}' | '\u{0010}' | '\u{0011}' | '\u{0012}' | '\u{0013}'
        | '\u{0014}' | '\u{0015}' | '\u{0016}' | '\u{0017}' | '\u{0018}' | '\u{0019}'
        | '\u{001A}' | '\u{001B}' | '\u{001C}' | '\u{001D}' | '\u{001E}' | '\u{001F}'
        | '\u{007F}' | '\u{202A}' | '\u{202B}' | '\u{202D}' | '\u{202E}' | '\u{2066}'
        | '\u{2067}' | '\u{2068}' | '\u{202C}' | '\u{2069}' => 1,
        _ => unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1),
    }
}

fn num_overlap(
    a_start: usize,
    a_end: usize,
    b_start: usize,
    b_end: usize,
    inclusive: bool,
) -> bool {
    let extra = usize::from(inclusive);
    (b_start..b_end + extra).contains(&a_start) || (a_start..a_end + extra).contains(&b_start)
}

fn overlaps(a1: &LineAnnotation<'_>, a2: &LineAnnotation<'_>, padding: usize) -> bool {
    num_overlap(
        a1.start.display,
        a1.end.display + padding,
        a2.start.display,
        a2.end.display,
        false,
    )
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum LineAnnotationType {
    /// Annotation under a single line of code
    Singleline,

    // The Multiline type above is replaced with the following three in order
    // to reuse the current label drawing code.
    //
    // Each of these corresponds to one part of the following diagram:
    //
    //     x |   foo(1 + bar(x,
    //       |  _________^              < MultilineStart
    //     x | |             y),        < MultilineLine
    //       | |______________^ label   < MultilineEnd
    //     x |       z);
    /// Annotation marking the first character of a fully shown multiline span
    MultilineStart(usize),
    /// Annotation marking the last character of a fully shown multiline span
    MultilineEnd(usize),
    /// Line at the left enclosing the lines of a fully shown multiline span
    // Just a placeholder for the drawing algorithm, to know that it shouldn't skip the first 4
    // and last 2 lines of code. The actual line is drawn in `emit_message_default` and not in
    // `draw_multiline_line`.
    MultilineLine(usize),
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct LineAnnotation<'a> {
    /// Start column.
    /// Note that it is important that this field goes
    /// first, so that when we sort, we sort orderings by start
    /// column.
    pub start: Loc,

    /// End column within the line (exclusive)
    pub end: Loc,

    /// level
    pub kind: AnnotationKind,

    /// Optional label to display adjacent to the annotation.
    pub label: Option<&'a str>,

    /// Is this a single line, multiline or multiline span minimized down to a
    /// smaller span.
    pub annotation_type: LineAnnotationType,
}

impl LineAnnotation<'_> {
    pub(crate) fn is_primary(&self) -> bool {
        self.kind == AnnotationKind::Primary
    }

    /// Whether this annotation is a vertical line placeholder.
    pub(crate) fn is_line(&self) -> bool {
        matches!(self.annotation_type, LineAnnotationType::MultilineLine(_))
    }

    /// Length of this annotation as displayed in the stderr output
    pub(crate) fn len(&self) -> usize {
        // Account for usize underflows
        if self.end.display > self.start.display {
            self.end.display - self.start.display
        } else {
            self.start.display - self.end.display
        }
    }

    pub(crate) fn has_label(&self) -> bool {
        if let Some(label) = self.label {
            // Consider labels with no text as effectively not being there
            // to avoid weird output with unnecessary vertical lines, like:
            //
            //     X | fn foo(x: u32) {
            //       | -------^------
            //       | |      |
            //       | |
            //       |
            //
            // Note that this would be the complete output users would see.
            !label.is_empty()
        } else {
            false
        }
    }

    pub(crate) fn takes_space(&self) -> bool {
        // Multiline annotations always have to keep vertical space.
        matches!(
            self.annotation_type,
            LineAnnotationType::MultilineStart(_) | LineAnnotationType::MultilineEnd(_)
        )
    }
}

// We replace some characters so the CLI output is always consistent and underlines aligned.
// Keep the following list in sync with `rustc_span::char_width`.
const OUTPUT_REPLACEMENTS: &[(char, &str)] = &[
    // In terminals without Unicode support the following will be garbled, but in *all* terminals
    // the underlying codepoint will be as well. We could gate this replacement behind a "unicode
    // support" gate.
    ('\0', "␀"),
    ('\u{0001}', "␁"),
    ('\u{0002}', "␂"),
    ('\u{0003}', "␃"),
    ('\u{0004}', "␄"),
    ('\u{0005}', "␅"),
    ('\u{0006}', "␆"),
    ('\u{0007}', "␇"),
    ('\u{0008}', "␈"),
    ('\t', "    "), // We do our own tab replacement
    ('\u{000b}', "␋"),
    ('\u{000c}', "␌"),
    ('\u{000d}', "␍"),
    ('\u{000e}', "␎"),
    ('\u{000f}', "␏"),
    ('\u{0010}', "␐"),
    ('\u{0011}', "␑"),
    ('\u{0012}', "␒"),
    ('\u{0013}', "␓"),
    ('\u{0014}', "␔"),
    ('\u{0015}', "␕"),
    ('\u{0016}', "␖"),
    ('\u{0017}', "␗"),
    ('\u{0018}', "␘"),
    ('\u{0019}', "␙"),
    ('\u{001a}', "␚"),
    ('\u{001b}', "␛"),
    ('\u{001c}', "␜"),
    ('\u{001d}', "␝"),
    ('\u{001e}', "␞"),
    ('\u{001f}', "␟"),
    ('\u{007f}', "␡"),
    ('\u{200d}', ""), // Replace ZWJ for consistent terminal output of grapheme clusters.
    ('\u{202a}', "�"), // The following unicode text flow control characters are inconsistently
    ('\u{202b}', "�"), // supported across CLIs and can cause confusion due to the bytes on disk
    ('\u{202c}', "�"), // not corresponding to the visible source code, so we replace them always.
    ('\u{202d}', "�"),
    ('\u{202e}', "�"),
    ('\u{2066}', "�"),
    ('\u{2067}', "�"),
    ('\u{2068}', "�"),
    ('\u{2069}', "�"),
];

fn normalize_whitespace(s: &str) -> String {
    // Scan the input string for a character in the ordered table above.
    // If it's present, replace it with its alternative string (it can be more than 1 char!).
    // Otherwise, retain the input char.
    s.chars().fold(String::with_capacity(s.len()), |mut s, c| {
        match OUTPUT_REPLACEMENTS.binary_search_by_key(&c, |(k, _)| *k) {
            Ok(i) => s.push_str(OUTPUT_REPLACEMENTS[i].1),
            _ => s.push(c),
        }
        s
    })
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum ElementStyle {
    MainHeaderMsg,
    HeaderMsg,
    LineAndColumn,
    LineNumber,
    Quotation,
    UnderlinePrimary,
    UnderlineSecondary,
    LabelPrimary,
    LabelSecondary,
    NoStyle,
    Level(Level),
}

impl ElementStyle {
    fn color_spec(&self, level: Level, stylesheet: &Stylesheet) -> Style {
        match self {
            ElementStyle::LineAndColumn => stylesheet.none,
            ElementStyle::LineNumber => stylesheet.line_no,
            ElementStyle::Quotation => stylesheet.none,
            ElementStyle::MainHeaderMsg => stylesheet.emphasis,
            ElementStyle::UnderlinePrimary | ElementStyle::LabelPrimary => level.style(stylesheet),
            ElementStyle::UnderlineSecondary | ElementStyle::LabelSecondary => stylesheet.context,
            ElementStyle::HeaderMsg | ElementStyle::NoStyle => stylesheet.none,
            ElementStyle::Level(lvl) => lvl.style(stylesheet),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct UnderlineParts {
    style: ElementStyle,
    underline: char,
    label_start: char,
    vertical_text_line: char,
    multiline_vertical: char,
    multiline_horizontal: char,
    multiline_whole_line: char,
    multiline_start_down: char,
    bottom_right: char,
    top_left: char,
    top_right_flat: char,
    bottom_left: char,
    multiline_end_up: char,
    multiline_end_same_line: char,
    multiline_bottom_right_with_text: char,
}

#[cfg(test)]
mod test {
    use super::OUTPUT_REPLACEMENTS;
    use snapbox::IntoData;

    fn format_replacements(replacements: Vec<(char, &str)>) -> String {
        replacements
            .into_iter()
            .map(|r| format!("    {r:?}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    /// The [`OUTPUT_REPLACEMENTS`] array must be sorted (for binary search to
    /// work) and must contain no duplicate entries
    fn ensure_output_replacements_is_sorted() {
        let mut expected = OUTPUT_REPLACEMENTS.to_owned();
        expected.sort_by_key(|r| r.0);
        expected.dedup_by_key(|r| r.0);
        let expected = format_replacements(expected);
        let actual = format_replacements(OUTPUT_REPLACEMENTS.to_owned());
        snapbox::assert_data_eq!(actual, expected.into_data().raw());
    }
}
