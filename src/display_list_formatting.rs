use display_list::{DisplayAnnotationPart, DisplayAnnotationType, DisplayLine, DisplayMark,
                   DisplayTextFragment};
use std::fmt;

pub trait DisplayListFormatting {
    fn format_annotation_type(annotation_type: &DisplayAnnotationType) -> String;

    fn format_inline_marks(inline_marks: &[DisplayMark], inline_marks_width: usize) -> String;

    fn format_source_annotation_lines(
        f: &mut fmt::Formatter,
        lineno_width: usize,
        inline_marks: String,
        range: &(usize, usize),
        label: &[DisplayTextFragment],
        annotation_type: &DisplayAnnotationType,
        annotation_part: &DisplayAnnotationPart,
    ) -> fmt::Result;

    fn format_label(label: &[DisplayTextFragment]) -> String;

    fn format_line(
        f: &mut fmt::Formatter,
        dl: &DisplayLine,
        lineno_width: usize,
        inline_marks_width: usize,
    ) -> fmt::Result;
}
