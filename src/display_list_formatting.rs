use display_list::{DisplayAnnotationType, DisplayLine, DisplayMark, DisplaySnippetType};
use std::fmt;

pub trait DisplayListFormatting {
    fn format_snippet_type(snippet_type: &DisplaySnippetType) -> String;

    fn format_inline_marks(inline_marks: &[DisplayMark], inline_marks_width: usize) -> String;

    fn format_annotation_content(
        range: &(usize, usize),
        label: &Option<String>,
        annotation_type: &DisplayAnnotationType,
    ) -> String;

    fn format_line(
        f: &mut fmt::Formatter,
        dl: &DisplayLine,
        lineno_width: usize,
        inline_marks_width: usize,
    ) -> fmt::Result;
}
